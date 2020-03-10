#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    /// Request message contains the name to be greeted.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloResponse {
    /// Response contains the greeting message.
    #[prost(string, tag = "1")]
    pub message: std::string::String,
}
#[doc = r" Generated server implementations."]
pub mod greeter_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GreeterServer."]
    #[async_trait]
    pub trait Greeter: Send + Sync + 'static {
        #[doc = " Our SayHello rpc accepts HelloRequests and returns HelloReplies."]
        async fn say_hello(
            &self,
            request: tonic::Request<super::HelloRequest>,
        ) -> Result<tonic::Response<super::HelloResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct GreeterServer<T: Greeter> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Greeter> GreeterServer<T> {
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
    impl<T: Greeter> Service<http::Request<HyperBody>> for GreeterServer<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/hello.Greeter/SayHello" => {
                    struct SayHelloSvc<T: Greeter>(pub Arc<T>);
                    impl<T: Greeter> tonic::server::UnaryService<super::HelloRequest> for SayHelloSvc<T> {
                        type Response = super::HelloResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HelloRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.say_hello(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SayHelloSvc(inner);
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
    impl<T: Greeter> Clone for GreeterServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Greeter> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Greeter> tonic::transport::NamedService for GreeterServer<T> {
        const NAME: &'static str = "hello.Greeter";
    }
}
