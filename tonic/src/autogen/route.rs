#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    #[prost(int32, tag = "1")]
    pub latitude: i32,
    #[prost(int32, tag = "2")]
    pub longitude: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rectangle {
    #[prost(message, optional, tag = "1")]
    pub lo: ::std::option::Option<Point>,
    #[prost(message, optional, tag = "2")]
    pub hi: ::std::option::Option<Point>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub location: ::std::option::Option<Point>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteNote {
    #[prost(message, optional, tag = "1")]
    pub location: ::std::option::Option<Point>,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteSummary {
    #[prost(int32, tag = "1")]
    pub point_count: i32,
    #[prost(int32, tag = "2")]
    pub feature_count: i32,
    #[prost(int32, tag = "3")]
    pub distance: i32,
    #[prost(int32, tag = "4")]
    pub elapsed_time: i32,
}
#[doc = r" Generated client implementations."]
pub mod route_guide_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Please see the linked file for the descriptions."]
    #[doc = ""]
    #[doc = " https://github.com/hyperium/tonic/blob/master/examples/proto/routeguide/route_guide.proto"]
    pub struct RouteGuideClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RouteGuideClient<tonic::transport::Channel> {
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
    impl<T> RouteGuideClient<T>
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
        pub async fn get_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::Point>,
        ) -> Result<tonic::Response<super::Feature>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/route.RouteGuide/GetFeature");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_features(
            &mut self,
            request: impl tonic::IntoRequest<super::Rectangle>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Feature>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/route.RouteGuide/ListFeatures");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn record_route(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::Point>,
        ) -> Result<tonic::Response<super::RouteSummary>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/route.RouteGuide/RecordRoute");
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        pub async fn route_chat(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::RouteNote>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::RouteNote>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/route.RouteGuide/RouteChat");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for RouteGuideClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod route_guide_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RouteGuideServer."]
    #[async_trait]
    pub trait RouteGuide: Send + Sync + 'static {
        async fn get_feature(
            &self,
            request: tonic::Request<super::Point>,
        ) -> Result<tonic::Response<super::Feature>, tonic::Status>;
        #[doc = "Server streaming response type for the ListFeatures method."]
        type ListFeaturesStream: Stream<Item = Result<super::Feature, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn list_features(
            &self,
            request: tonic::Request<super::Rectangle>,
        ) -> Result<tonic::Response<Self::ListFeaturesStream>, tonic::Status>;
        async fn record_route(
            &self,
            request: tonic::Request<tonic::Streaming<super::Point>>,
        ) -> Result<tonic::Response<super::RouteSummary>, tonic::Status>;
        #[doc = "Server streaming response type for the RouteChat method."]
        type RouteChatStream: Stream<Item = Result<super::RouteNote, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn route_chat(
            &self,
            request: tonic::Request<tonic::Streaming<super::RouteNote>>,
        ) -> Result<tonic::Response<Self::RouteChatStream>, tonic::Status>;
    }
    #[doc = " Please see the linked file for the descriptions."]
    #[doc = ""]
    #[doc = " https://github.com/hyperium/tonic/blob/master/examples/proto/routeguide/route_guide.proto"]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct RouteGuideServer<T: RouteGuide> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: RouteGuide> RouteGuideServer<T> {
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
    impl<T: RouteGuide> Service<http::Request<HyperBody>> for RouteGuideServer<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/route.RouteGuide/GetFeature" => {
                    struct GetFeatureSvc<T: RouteGuide>(pub Arc<T>);
                    impl<T: RouteGuide> tonic::server::UnaryService<super::Point> for GetFeatureSvc<T> {
                        type Response = super::Feature;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Point>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_feature(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetFeatureSvc(inner);
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
                "/route.RouteGuide/ListFeatures" => {
                    struct ListFeaturesSvc<T: RouteGuide>(pub Arc<T>);
                    impl<T: RouteGuide> tonic::server::ServerStreamingService<super::Rectangle> for ListFeaturesSvc<T> {
                        type Response = super::Feature;
                        type ResponseStream = T::ListFeaturesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Rectangle>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_features(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ListFeaturesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/route.RouteGuide/RecordRoute" => {
                    struct RecordRouteSvc<T: RouteGuide>(pub Arc<T>);
                    impl<T: RouteGuide> tonic::server::ClientStreamingService<super::Point> for RecordRouteSvc<T> {
                        type Response = super::RouteSummary;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::Point>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.record_route(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = RecordRouteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/route.RouteGuide/RouteChat" => {
                    struct RouteChatSvc<T: RouteGuide>(pub Arc<T>);
                    impl<T: RouteGuide> tonic::server::StreamingService<super::RouteNote> for RouteChatSvc<T> {
                        type Response = super::RouteNote;
                        type ResponseStream = T::RouteChatStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::RouteNote>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.route_chat(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = RouteChatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
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
    impl<T: RouteGuide> Clone for RouteGuideServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: RouteGuide> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RouteGuide> tonic::transport::NamedService for RouteGuideServer<T> {
        const NAME: &'static str = "route.RouteGuide";
    }
}
