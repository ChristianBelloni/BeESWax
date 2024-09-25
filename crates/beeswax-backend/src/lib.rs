use app_state::AppState;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router, ServiceExt};
use axum_tonic::RestGrpcService;
use beeswax_core::BeeswaxCore;
use std::{net::SocketAddr, path::Path};
use tower_http::{
    services::ServeDir,
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub mod app_state;

pub fn create_router(
    state: &AppState,
    public: impl AsRef<Path> + Send + Sync + 'static + Clone,
) -> axum::extract::connect_info::IntoMakeServiceWithConnectInfo<RestGrpcService, SocketAddr> {
    let app: Router = Router::new()
        .fallback_service(get(|req| async move {
            match tower::ServiceExt::oneshot(ServeDir::new(public), req).await {
                Ok(res) => res.into_response(),
                Err(err) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("error {err}")).into_response()
                }
            }
        }))
        .with_state(state.clone())
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );
    let grpc_service = BeeswaxCore::new().run().layer(
        TraceLayer::new_for_grpc()
            .on_request(DefaultOnRequest::new().level(tracing::Level::DEBUG))
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
    );
    let service = RestGrpcService::new(app, grpc_service);

    service.into_make_service_with_connect_info()
}
