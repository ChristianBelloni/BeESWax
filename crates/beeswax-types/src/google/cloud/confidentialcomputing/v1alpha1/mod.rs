// This file is @generated by prost-build.
/// A Challenge from the server used to guarantee freshness of attestations
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Challenge {
    /// Output only. The resource name for this Challenge in the format
    /// `projects/*/locations/*/challenges/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time at which this Challenge was created
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this Challenge will no longer be usable. It
    /// is also the expiration time for any tokens generated from this Challenge.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Indicates if this challenge has been used to generate a token.
    #[prost(bool, tag = "4")]
    pub used: bool,
    /// Output only. Random data which should be used when calling TPM2_Quote.
    /// --
    #[prost(bytes = "bytes", tag = "5")]
    pub nonce: ::prost::bytes::Bytes,
}
/// Message for creating a Challenge
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChallengeRequest {
    /// Required. The resource name of the location where the Challenge will be
    /// used, in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Challenge to be created. Currently this field can be empty as
    /// all the Challenge fields are set by the server.
    #[prost(message, optional, tag = "2")]
    pub challenge: ::core::option::Option<Challenge>,
}
/// A request for an OIDC token, providing all the necessary information needed
/// for this service to verify the plaform state of the requestor.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyAttestationRequest {
    /// Required. The name of the Challenge whose nonce was used to generate the
    /// attestation, in the format `projects/*/locations/*/challenges/*`. The
    /// provided Challenge will be consumed, and cannot be used again.
    #[prost(string, tag = "1")]
    pub challenge: ::prost::alloc::string::String,
    /// Optional. Credentials used to populate the "emails" claim in the
    /// claims_token.
    #[prost(message, optional, tag = "2")]
    pub gcp_credentials: ::core::option::Option<GcpCredentials>,
    /// Required. The TPM-specific data provided by the attesting platform, used to
    /// populate any of the claims regarding platform state.
    #[prost(message, optional, tag = "3")]
    pub tpm_attestation: ::core::option::Option<TpmAttestation>,
}
/// A response once an attestation has been successfully verified, containing a
/// signed OIDC token.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyAttestationResponse {
    /// Output only. The OIDC token issued by this service. It contains specific
    /// platform claims based on the contents of the provided attestation.
    /// --
    #[prost(bytes = "bytes", tag = "1")]
    pub claims_token: ::prost::bytes::Bytes,
}
/// Credentials issued by GCP which are linked to the platform attestation. These
/// will be verified server-side as part of attestaion verification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcpCredentials {
    /// A list of service account OpenID Connect ID tokens identifying which
    /// service account identities should be included in the claims_token. These
    /// can be generated by calling `serviceAccounts.generateIdToken`. The
    /// Challenge.name must be used as the `audience` parameter, and the
    /// `includeEmail` parameter must be `true`.
    /// --
    #[prost(bytes = "bytes", repeated, tag = "1")]
    pub id_tokens: ::prost::alloc::vec::Vec<::prost::bytes::Bytes>,
}
/// TPM2 data containing everything necessary to validate any platform state
/// measured into the TPM.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TpmAttestation {
    /// TPM2 PCR Quotes generated by calling TPM2_Quote on each PCR bank.
    #[prost(message, repeated, tag = "1")]
    pub quotes: ::prost::alloc::vec::Vec<tpm_attestation::Quote>,
    /// The binary TCG Event Log containing events measured into the TPM by the
    /// platform firmware and operating system. Formatted as described in the
    /// "TCG PC Client Platform Firmware Profile Specification".
    #[prost(bytes = "bytes", tag = "2")]
    pub tcg_event_log: ::prost::bytes::Bytes,
    /// An Event Log containing additional events measured into the TPM that are
    /// not already present in the tcg_event_log. Formatted as described in the
    /// "Canonical Event Log Format" TCG Specification.
    #[prost(bytes = "bytes", tag = "3")]
    pub canonical_event_log: ::prost::bytes::Bytes,
    /// DER-encoded X.509 certificate of the Attestation Key (otherwise known as
    /// an AK or a TPM restricted signing key) used to generate the quotes.
    #[prost(bytes = "bytes", tag = "4")]
    pub ak_cert: ::prost::bytes::Bytes,
    /// List of DER-encoded X.509 certificates which, together with the ak_cert,
    /// chain back to a trusted Root Certificate.
    #[prost(bytes = "bytes", repeated, tag = "5")]
    pub cert_chain: ::prost::alloc::vec::Vec<::prost::bytes::Bytes>,
}
/// Nested message and enum types in `TpmAttestation`.
pub mod tpm_attestation {
    /// Information about Platform Control Registers (PCRs) including a signature
    /// over their values, which can be used for remote validation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Quote {
        /// The hash algorithm of the PCR bank being quoted, encoded as a TPM_ALG_ID
        #[prost(int32, tag = "1")]
        pub hash_algo: i32,
        /// Raw binary values of each PCRs being quoted.
        #[prost(btree_map = "int32, bytes", tag = "2")]
        pub pcr_values: ::prost::alloc::collections::BTreeMap<
            i32,
            ::prost::bytes::Bytes,
        >,
        /// TPM2 quote, encoded as a TPMS_ATTEST
        #[prost(bytes = "bytes", tag = "3")]
        pub raw_quote: ::prost::bytes::Bytes,
        /// TPM2 signature, encoded as a TPMT_SIGNATURE
        #[prost(bytes = "bytes", tag = "4")]
        pub raw_signature: ::prost::bytes::Bytes,
    }
}
/// Generated client implementations.
pub mod confidential_computing_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources
    #[derive(Debug, Clone)]
    pub struct ConfidentialComputingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConfidentialComputingClient<T>
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
        ) -> ConfidentialComputingClient<InterceptedService<T, F>>
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
            ConfidentialComputingClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new Challenge in a given project and location.
        pub async fn create_challenge(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateChallengeRequest>,
        ) -> std::result::Result<tonic::Response<super::Challenge>, tonic::Status> {
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
                "/google.cloud.confidentialcomputing.v1alpha1.ConfidentialComputing/CreateChallenge",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.confidentialcomputing.v1alpha1.ConfidentialComputing",
                        "CreateChallenge",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Verifies the provided attestation info, returning a signed OIDC token.
        pub async fn verify_attestation(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyAttestationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyAttestationResponse>,
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
                "/google.cloud.confidentialcomputing.v1alpha1.ConfidentialComputing/VerifyAttestation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.confidentialcomputing.v1alpha1.ConfidentialComputing",
                        "VerifyAttestation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod confidential_computing_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ConfidentialComputingServer.
    #[async_trait]
    pub trait ConfidentialComputing: std::marker::Send + std::marker::Sync + 'static {
        /// Creates a new Challenge in a given project and location.
        async fn create_challenge(
            &self,
            request: tonic::Request<super::CreateChallengeRequest>,
        ) -> std::result::Result<tonic::Response<super::Challenge>, tonic::Status>;
        /// Verifies the provided attestation info, returning a signed OIDC token.
        async fn verify_attestation(
            &self,
            request: tonic::Request<super::VerifyAttestationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyAttestationResponse>,
            tonic::Status,
        >;
    }
    /// Service describing handlers for resources
    #[derive(Debug)]
    pub struct ConfidentialComputingServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ConfidentialComputingServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for ConfidentialComputingServer<T>
    where
        T: ConfidentialComputing,
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
                "/google.cloud.confidentialcomputing.v1alpha1.ConfidentialComputing/CreateChallenge" => {
                    #[allow(non_camel_case_types)]
                    struct CreateChallengeSvc<T: ConfidentialComputing>(pub Arc<T>);
                    impl<
                        T: ConfidentialComputing,
                    > tonic::server::UnaryService<super::CreateChallengeRequest>
                    for CreateChallengeSvc<T> {
                        type Response = super::Challenge;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateChallengeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConfidentialComputing>::create_challenge(
                                        &inner,
                                        request,
                                    )
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
                        let method = CreateChallengeSvc(inner);
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
                "/google.cloud.confidentialcomputing.v1alpha1.ConfidentialComputing/VerifyAttestation" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyAttestationSvc<T: ConfidentialComputing>(pub Arc<T>);
                    impl<
                        T: ConfidentialComputing,
                    > tonic::server::UnaryService<super::VerifyAttestationRequest>
                    for VerifyAttestationSvc<T> {
                        type Response = super::VerifyAttestationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VerifyAttestationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConfidentialComputing>::verify_attestation(
                                        &inner,
                                        request,
                                    )
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
                        let method = VerifyAttestationSvc(inner);
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
    impl<T> Clone for ConfidentialComputingServer<T> {
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
    pub const SERVICE_NAME: &str = "google.cloud.confidentialcomputing.v1alpha1.ConfidentialComputing";
    impl<T> tonic::server::NamedService for ConfidentialComputingServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
