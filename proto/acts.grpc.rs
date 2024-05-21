#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListValue {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<ProtoJsonValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Struct {
    #[prost(map = "string, message", tag = "1")]
    pub fields: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ProtoJsonValue,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoJsonValue {
    #[prost(oneof = "proto_json_value::Kind", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
    pub kind: ::core::option::Option<proto_json_value::Kind>,
}
/// Nested message and enum types in `ProtoJsonValue`.
pub mod proto_json_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(enumeration = "super::NullValue", tag = "1")]
        NullValue(i32),
        #[prost(double, tag = "2")]
        F64Value(f64),
        #[prost(int64, tag = "3")]
        I64Value(i64),
        #[prost(uint64, tag = "4")]
        U64Value(u64),
        #[prost(string, tag = "5")]
        StringValue(::prost::alloc::string::String),
        #[prost(bool, tag = "6")]
        BoolValue(bool),
        #[prost(message, tag = "7")]
        StructValue(super::Struct),
        #[prost(message, tag = "8")]
        ListValue(super::ListValue),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageOptions {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// message type
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// message event
    #[prost(string, tag = "3")]
    pub state: ::prost::alloc::string::String,
    /// model tag
    #[prost(string, tag = "4")]
    pub tag: ::prost::alloc::string::String,
    /// message key
    #[prost(string, tag = "5")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowOptions {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionOptions {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<ProtoJsonValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowState {
    #[prost(string, tag = "1")]
    pub pid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub mid: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub event: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub state: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub start_time: i64,
    #[prost(int64, tag = "6")]
    pub end_time: i64,
    #[prost(message, optional, tag = "7")]
    pub outputs: ::core::option::Option<ProtoJsonValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowModel {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowMessage {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub state: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub source: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub model: ::core::option::Option<WorkflowModel>,
    #[prost(string, tag = "9")]
    pub pid: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub tid: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub tag: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "13")]
    pub inputs: ::core::option::Option<ProtoJsonValue>,
    #[prost(message, optional, tag = "14")]
    pub outputs: ::core::option::Option<ProtoJsonValue>,
    #[prost(int64, tag = "15")]
    pub start_time: i64,
    #[prost(int64, tag = "16")]
    pub end_time: i64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NullValue {
    NullValue = 0,
}
impl NullValue {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NullValue::NullValue => "NULL_VALUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NULL_VALUE" => Some(Self::NullValue),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod acts_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ActsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ActsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ActsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> ActsServiceClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            ActsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn on_message(
            &mut self,
            request: impl tonic::IntoRequest<super::MessageOptions>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::WorkflowMessage>>,
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
                "/acts.grpc.ActsService/OnMessage",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn action(
            &mut self,
            request: impl tonic::IntoRequest<super::ActionOptions>,
        ) -> Result<tonic::Response<super::ProtoJsonValue>, tonic::Status> {
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
                "/acts.grpc.ActsService/action",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod acts_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ActsServiceServer.
    #[async_trait]
    pub trait ActsService: Send + Sync + 'static {
        /// Server streaming response type for the OnMessage method.
        type OnMessageStream: futures_core::Stream<
                Item = Result<super::WorkflowMessage, tonic::Status>,
            >
            + Send
            + 'static;
        async fn on_message(
            &self,
            request: tonic::Request<super::MessageOptions>,
        ) -> Result<tonic::Response<Self::OnMessageStream>, tonic::Status>;
        async fn action(
            &self,
            request: tonic::Request<super::ActionOptions>,
        ) -> Result<tonic::Response<super::ProtoJsonValue>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ActsServiceServer<T: ActsService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ActsService> ActsServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ActsServiceServer<T>
    where
        T: ActsService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/acts.grpc.ActsService/OnMessage" => {
                    #[allow(non_camel_case_types)]
                    struct OnMessageSvc<T: ActsService>(pub Arc<T>);
                    impl<
                        T: ActsService,
                    > tonic::server::ServerStreamingService<super::MessageOptions>
                    for OnMessageSvc<T> {
                        type Response = super::WorkflowMessage;
                        type ResponseStream = T::OnMessageStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MessageOptions>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).on_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OnMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/acts.grpc.ActsService/action" => {
                    #[allow(non_camel_case_types)]
                    struct actionSvc<T: ActsService>(pub Arc<T>);
                    impl<
                        T: ActsService,
                    > tonic::server::UnaryService<super::ActionOptions>
                    for actionSvc<T> {
                        type Response = super::ProtoJsonValue;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ActionOptions>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).action(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = actionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
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
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ActsService> Clone for ActsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ActsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ActsService> tonic::server::NamedService for ActsServiceServer<T> {
        const NAME: &'static str = "acts.grpc.ActsService";
    }
}
