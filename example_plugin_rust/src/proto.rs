#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventsRequest {
    #[prost(string, tag = "1")]
    pub plugin_interface: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventsResponse {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractionRequest {
    #[prost(string, tag = "1")]
    pub plugin_interface: std::string::String,
    #[prost(string, tag = "2")]
    pub function: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractionResponse {
    #[prost(string, tag = "1")]
    pub value: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod plugin_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct PluginClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PluginClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PluginClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn on_load(
            &mut self,
            request: impl tonic::IntoRequest<super::EventsRequest>,
        ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Plugin/OnLoad");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn on_unload(
            &mut self,
            request: impl tonic::IntoRequest<super::EventsRequest>,
        ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Plugin/OnUnload");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn on_install(
            &mut self,
            request: impl tonic::IntoRequest<super::EventsRequest>,
        ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Plugin/OnInstall");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn on_uninstall(
            &mut self,
            request: impl tonic::IntoRequest<super::EventsRequest>,
        ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Plugin/OnUninstall");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn on_start(
            &mut self,
            request: impl tonic::IntoRequest<super::EventsRequest>,
        ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Plugin/OnStart");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn on_stop(
            &mut self,
            request: impl tonic::IntoRequest<super::EventsRequest>,
        ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Plugin/OnStop");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn interaction(
            &mut self,
            request: impl tonic::IntoRequest<super::InteractionRequest>,
        ) -> Result<tonic::Response<super::InteractionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Plugin/Interaction");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PluginClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PluginClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PluginClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod plugin_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PluginServer."]
    #[async_trait]
    pub trait Plugin: Send + Sync + 'static {
        async fn on_load(
            &self,
            request: tonic::Request<super::EventsRequest>,
        ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status>;
        async fn on_unload(
            &self,
            request: tonic::Request<super::EventsRequest>,
        ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status>;
        async fn on_install(
            &self,
            request: tonic::Request<super::EventsRequest>,
        ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status>;
        async fn on_uninstall(
            &self,
            request: tonic::Request<super::EventsRequest>,
        ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status>;
        async fn on_start(
            &self,
            request: tonic::Request<super::EventsRequest>,
        ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status>;
        async fn on_stop(
            &self,
            request: tonic::Request<super::EventsRequest>,
        ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status>;
        async fn interaction(
            &self,
            request: tonic::Request<super::InteractionRequest>,
        ) -> Result<tonic::Response<super::InteractionResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PluginServer<T: Plugin> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Plugin> PluginServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for PluginServer<T>
    where
        T: Plugin,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/proto.Plugin/OnLoad" => {
                    #[allow(non_camel_case_types)]
                    struct OnLoadSvc<T: Plugin>(pub Arc<T>);
                    impl<T: Plugin> tonic::server::UnaryService<super::EventsRequest> for OnLoadSvc<T> {
                        type Response = super::EventsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).on_load(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = OnLoadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Plugin/OnUnload" => {
                    #[allow(non_camel_case_types)]
                    struct OnUnloadSvc<T: Plugin>(pub Arc<T>);
                    impl<T: Plugin> tonic::server::UnaryService<super::EventsRequest> for OnUnloadSvc<T> {
                        type Response = super::EventsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).on_unload(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = OnUnloadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Plugin/OnInstall" => {
                    #[allow(non_camel_case_types)]
                    struct OnInstallSvc<T: Plugin>(pub Arc<T>);
                    impl<T: Plugin> tonic::server::UnaryService<super::EventsRequest> for OnInstallSvc<T> {
                        type Response = super::EventsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).on_install(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = OnInstallSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Plugin/OnUninstall" => {
                    #[allow(non_camel_case_types)]
                    struct OnUninstallSvc<T: Plugin>(pub Arc<T>);
                    impl<T: Plugin> tonic::server::UnaryService<super::EventsRequest> for OnUninstallSvc<T> {
                        type Response = super::EventsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).on_uninstall(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = OnUninstallSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Plugin/OnStart" => {
                    #[allow(non_camel_case_types)]
                    struct OnStartSvc<T: Plugin>(pub Arc<T>);
                    impl<T: Plugin> tonic::server::UnaryService<super::EventsRequest> for OnStartSvc<T> {
                        type Response = super::EventsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).on_start(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = OnStartSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Plugin/OnStop" => {
                    #[allow(non_camel_case_types)]
                    struct OnStopSvc<T: Plugin>(pub Arc<T>);
                    impl<T: Plugin> tonic::server::UnaryService<super::EventsRequest> for OnStopSvc<T> {
                        type Response = super::EventsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).on_stop(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = OnStopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Plugin/Interaction" => {
                    #[allow(non_camel_case_types)]
                    struct InteractionSvc<T: Plugin>(pub Arc<T>);
                    impl<T: Plugin> tonic::server::UnaryService<super::InteractionRequest> for InteractionSvc<T> {
                        type Response = super::InteractionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InteractionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).interaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = InteractionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Plugin> Clone for PluginServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Plugin> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Plugin> tonic::transport::NamedService for PluginServer<T> {
        const NAME: &'static str = "proto.Plugin";
    }
}
