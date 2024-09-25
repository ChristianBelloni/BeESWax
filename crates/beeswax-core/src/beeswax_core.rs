use beeswax_types::build_event_stream::BuildEvent as BazelBuildEvent;
use beeswax_types::google::devtools::build::v1::{publish_build_event_server::*, *};
use prost::Message;
use std::collections::HashMap;
use std::sync::Arc;
use std::{net::ToSocketAddrs, pin::Pin};
use tokio::sync::Mutex;
use tokio_stream::{wrappers::*, Stream, StreamExt};
use tonic::async_trait;
use tonic::{transport::Server, Status};
use tracing::{instrument, Instrument};

use crate::store::BuildHandler;

type ResponseStream =
    Pin<Box<dyn Stream<Item = Result<PublishBuildToolEventStreamResponse, Status>> + Send>>;

#[derive(Default)]
pub struct BeeswaxCore {
    handlers: Arc<Mutex<HashMap<String, BuildHandler>>>,
}

#[async_trait]
impl PublishBuildEvent for BeeswaxCore {
    type PublishBuildToolEventStreamStream = ResponseStream;

    #[instrument(skip(self, request), err)]
    async fn publish_lifecycle_event(
        &self,
        request: tonic::Request<PublishLifecycleEventRequest>,
    ) -> Result<tonic::Response<()>, Status> {
        let request = request.into_inner();

        let build_event = request.build_event.unwrap();

        let id = build_event.stream_id.unwrap();

        match build_event.event.unwrap().event.unwrap() {
            build_event::Event::BuildEnqueued(_) => {
                self.handlers
                    .lock()
                    .await
                    .insert(id.build_id.clone(), BuildHandler::new(id.build_id));
            }
            build_event::Event::BuildFinished(_) => {}

            _ => {}
        };
        Ok(tonic::Response::new(()))
    }

    #[instrument(skip(self, request), err)]
    async fn publish_build_tool_event_stream(
        &self,
        request: tonic::Request<tonic::Streaming<PublishBuildToolEventStreamRequest>>,
    ) -> Result<tonic::Response<Self::PublishBuildToolEventStreamStream>, Status> {
        let (ack_tx, ack_rx) = tokio::sync::mpsc::unbounded_channel();
        let mut stream = request.into_inner();

        let handlers = self.handlers.clone();
        tracing::info!("opening new stream");

        let (build_id_tx, build_id_rx) = tokio::sync::oneshot::channel();

        let abort_handle = tokio::spawn(
            async move {
                let mut build_id = None;
                let mut build_id_tx = Some(build_id_tx);
                while let Some(Ok(next)) = stream.next().await {
                    let evnt = next.ordered_build_event.unwrap();
                    let stream_id = evnt.stream_id.clone();
                    build_id.replace(stream_id.as_ref().unwrap().build_id.clone());
                    let sequence_number = evnt.sequence_number;

                    match evnt.event.unwrap().event.unwrap() {
                        build_event::Event::InvocationAttemptStarted(_) => {}
                        build_event::Event::InvocationAttemptFinished(_) => {}
                        build_event::Event::BuildEnqueued(_) => {}
                        build_event::Event::BuildFinished(_) => {}
                        build_event::Event::ConsoleOutput(_) => {}
                        build_event::Event::ComponentStreamFinished(_) => {}
                        build_event::Event::BazelEvent(evnt) => {
                            if let Some(build_id_tx) = build_id_tx.take() {
                                _ = build_id_tx.send(build_id.clone().unwrap());
                            }
                            let buf: &[u8] = &evnt.value;
                            let p: BazelBuildEvent = Message::decode(buf).unwrap();

                            if let Some(handler) =
                                handlers.lock().await.get_mut(build_id.as_ref().unwrap())
                            {
                                handler.handle_event(p)
                            }
                        }
                        build_event::Event::BuildExecutionEvent(_) => {}
                        build_event::Event::SourceFetchEvent(_) => {}
                    };

                    ack_tx
                        .send(Ok(PublishBuildToolEventStreamResponse {
                            stream_id,
                            sequence_number,
                        }))
                        .unwrap();
                }

                tracing::info!("closing stream");
                handlers.lock().await.remove(build_id.as_ref().unwrap())
            }
            .instrument(tracing::Span::current()),
        )
        .abort_handle();

        if let Ok(build_id) = build_id_rx.await {
            if let Some(handler) = self.handlers.lock().await.get_mut(&build_id) {
                handler.set_abort_handle(abort_handle);
            }
        } else {
            abort_handle.abort();
        }

        Ok(tonic::Response::new(
            Box::pin(UnboundedReceiverStream::new(ack_rx)) as _,
        ))
    }
}

impl BeeswaxCore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(self) -> tonic::service::AxumRouter {
        Server::builder()
            .add_service(PublishBuildEventServer::new(self))
            .into_service()
            .into_axum_router()
    }
}
