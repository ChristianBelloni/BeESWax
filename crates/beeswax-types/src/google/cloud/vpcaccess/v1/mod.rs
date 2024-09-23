// This file is @generated by prost-build.
/// Definition of a Serverless VPC Access connector.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connector {
    /// The resource name in the format `projects/*/locations/*/connectors/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Name of a VPC network.
    #[prost(string, tag = "2")]
    pub network: ::prost::alloc::string::String,
    /// The range of internal addresses that follows RFC 4632 notation.
    /// Example: `10.132.0.0/28`.
    #[prost(string, tag = "3")]
    pub ip_cidr_range: ::prost::alloc::string::String,
    /// Output only. State of the VPC access connector.
    #[prost(enumeration = "connector::State", tag = "4")]
    pub state: i32,
    /// Minimum throughput of the connector in Mbps. Default and min is 200.
    #[prost(int32, tag = "5")]
    pub min_throughput: i32,
    /// Maximum throughput of the connector in Mbps. Default is 300, max is 1000.
    #[prost(int32, tag = "6")]
    pub max_throughput: i32,
    /// Output only. List of projects using the connector.
    #[prost(string, repeated, tag = "7")]
    pub connected_projects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The subnet in which to house the VPC Access Connector.
    #[prost(message, optional, tag = "8")]
    pub subnet: ::core::option::Option<connector::Subnet>,
    /// Machine type of VM Instance underlying connector. Default is e2-micro
    #[prost(string, tag = "10")]
    pub machine_type: ::prost::alloc::string::String,
    /// Minimum value of instances in autoscaling group underlying the connector.
    #[prost(int32, tag = "11")]
    pub min_instances: i32,
    /// Maximum value of instances in autoscaling group underlying the connector.
    #[prost(int32, tag = "12")]
    pub max_instances: i32,
}
/// Nested message and enum types in `Connector`.
pub mod connector {
    /// The subnet in which to house the connector
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Subnet {
        /// Subnet name (relative, not fully qualified).
        /// E.g. if the full subnet selfLink is
        /// <https://compute.googleapis.com/compute/v1/projects/{project}/regions/{region}/subnetworks/{subnetName}>
        /// the correct input for this field would be {subnetName}
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Project in which the subnet exists.
        /// If not set, this project is assumed to be the project for which
        /// the connector create request was issued.
        #[prost(string, tag = "2")]
        pub project_id: ::prost::alloc::string::String,
    }
    /// State of a connector.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Invalid state.
        Unspecified = 0,
        /// Connector is deployed and ready to receive traffic.
        Ready = 1,
        /// An Insert operation is in progress. Transient condition.
        Creating = 2,
        /// A Delete operation is in progress. Transient condition.
        Deleting = 3,
        /// Connector is in a bad state, manual deletion recommended.
        Error = 4,
        /// The connector is being updated.
        Updating = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "STATE_UNSPECIFIED",
                Self::Ready => "READY",
                Self::Creating => "CREATING",
                Self::Deleting => "DELETING",
                Self::Error => "ERROR",
                Self::Updating => "UPDATING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "READY" => Some(Self::Ready),
                "CREATING" => Some(Self::Creating),
                "DELETING" => Some(Self::Deleting),
                "ERROR" => Some(Self::Error),
                "UPDATING" => Some(Self::Updating),
                _ => None,
            }
        }
    }
}
/// Request for creating a Serverless VPC Access connector.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectorRequest {
    /// Required. The project and location in which the configuration should be created,
    /// specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID to use for this connector.
    #[prost(string, tag = "2")]
    pub connector_id: ::prost::alloc::string::String,
    /// Required. Resource to create.
    #[prost(message, optional, tag = "3")]
    pub connector: ::core::option::Option<Connector>,
}
/// Request for getting a Serverless VPC Access connector.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectorRequest {
    /// Required. Name of a Serverless VPC Access connector to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for listing Serverless VPC Access connectors in a location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectorsRequest {
    /// Required. The project and location from which the routes should be listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of functions to return per call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for listing Serverless VPC Access connectors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectorsResponse {
    /// List of Serverless VPC Access connectors.
    #[prost(message, repeated, tag = "1")]
    pub connectors: ::prost::alloc::vec::Vec<Connector>,
    /// Continuation token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for deleting a Serverless VPC Access connector.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConnectorRequest {
    /// Required. Name of a Serverless VPC Access connector to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metadata for google.longrunning.Operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. Method that initiated the operation e.g.
    /// google.cloud.vpcaccess.v1.Connectors.CreateConnector.
    #[prost(string, tag = "1")]
    pub method: ::prost::alloc::string::String,
    /// Output only. Time when the operation was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the operation completed.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Name of the resource that this operation is acting on e.g.
    /// projects/my-project/locations/us-central1/connectors/v1.
    #[prost(string, tag = "5")]
    pub target: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod vpc_access_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Serverless VPC Access API allows users to create and manage connectors for
    /// App Engine, Cloud Functions and Cloud Run to have internal connections to
    /// Virtual Private Cloud networks.
    #[derive(Debug, Clone)]
    pub struct VpcAccessServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VpcAccessServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> VpcAccessServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            VpcAccessServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Creates a Serverless VPC Access connector, returns an operation.
        pub async fn create_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConnectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vpcaccess.v1.VpcAccessService/CreateConnector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vpcaccess.v1.VpcAccessService",
                        "CreateConnector",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a Serverless VPC Access connector. Returns NOT_FOUND if the resource
        /// does not exist.
        pub async fn get_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectorRequest>,
        ) -> std::result::Result<tonic::Response<super::Connector>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vpcaccess.v1.VpcAccessService/GetConnector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vpcaccess.v1.VpcAccessService",
                        "GetConnector",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Serverless VPC Access connectors.
        pub async fn list_connectors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConnectorsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vpcaccess.v1.VpcAccessService/ListConnectors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vpcaccess.v1.VpcAccessService",
                        "ListConnectors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a Serverless VPC Access connector. Returns NOT_FOUND if the
        /// resource does not exist.
        pub async fn delete_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConnectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vpcaccess.v1.VpcAccessService/DeleteConnector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vpcaccess.v1.VpcAccessService",
                        "DeleteConnector",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod vpc_access_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VpcAccessServiceServer.
    #[async_trait]
    pub trait VpcAccessService: std::marker::Send + std::marker::Sync + 'static {
        /// Creates a Serverless VPC Access connector, returns an operation.
        async fn create_connector(
            &self,
            request: tonic::Request<super::CreateConnectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        /// Gets a Serverless VPC Access connector. Returns NOT_FOUND if the resource
        /// does not exist.
        async fn get_connector(
            &self,
            request: tonic::Request<super::GetConnectorRequest>,
        ) -> std::result::Result<tonic::Response<super::Connector>, tonic::Status>;
        /// Lists Serverless VPC Access connectors.
        async fn list_connectors(
            &self,
            request: tonic::Request<super::ListConnectorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConnectorsResponse>,
            tonic::Status,
        >;
        /// Deletes a Serverless VPC Access connector. Returns NOT_FOUND if the
        /// resource does not exist.
        async fn delete_connector(
            &self,
            request: tonic::Request<super::DeleteConnectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    /// Serverless VPC Access API allows users to create and manage connectors for
    /// App Engine, Cloud Functions and Cloud Run to have internal connections to
    /// Virtual Private Cloud networks.
    #[derive(Debug)]
    pub struct VpcAccessServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> VpcAccessServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for VpcAccessServiceServer<T>
    where
        T: VpcAccessService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/google.cloud.vpcaccess.v1.VpcAccessService/CreateConnector" => {
                    #[allow(non_camel_case_types)]
                    struct CreateConnectorSvc<T: VpcAccessService>(pub Arc<T>);
                    impl<
                        T: VpcAccessService,
                    > tonic::server::UnaryService<super::CreateConnectorRequest>
                    for CreateConnectorSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateConnectorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as VpcAccessService>::create_connector(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateConnectorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.vpcaccess.v1.VpcAccessService/GetConnector" => {
                    #[allow(non_camel_case_types)]
                    struct GetConnectorSvc<T: VpcAccessService>(pub Arc<T>);
                    impl<
                        T: VpcAccessService,
                    > tonic::server::UnaryService<super::GetConnectorRequest>
                    for GetConnectorSvc<T> {
                        type Response = super::Connector;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetConnectorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as VpcAccessService>::get_connector(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetConnectorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.vpcaccess.v1.VpcAccessService/ListConnectors" => {
                    #[allow(non_camel_case_types)]
                    struct ListConnectorsSvc<T: VpcAccessService>(pub Arc<T>);
                    impl<
                        T: VpcAccessService,
                    > tonic::server::UnaryService<super::ListConnectorsRequest>
                    for ListConnectorsSvc<T> {
                        type Response = super::ListConnectorsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListConnectorsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as VpcAccessService>::list_connectors(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListConnectorsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.vpcaccess.v1.VpcAccessService/DeleteConnector" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteConnectorSvc<T: VpcAccessService>(pub Arc<T>);
                    impl<
                        T: VpcAccessService,
                    > tonic::server::UnaryService<super::DeleteConnectorRequest>
                    for DeleteConnectorSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteConnectorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as VpcAccessService>::delete_connector(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteConnectorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", tonic::Code::Unimplemented as i32)
                                .header(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                )
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T> Clone for VpcAccessServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "google.cloud.vpcaccess.v1.VpcAccessService";
    impl<T> tonic::server::NamedService for VpcAccessServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
