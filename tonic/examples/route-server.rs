//! [Route] guide server
//!
//! [route]: https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
use std::{error, pin::Pin};

use futures_channel::mpsc;
use futures_core::stream::Stream;
use tonic::{transport::Server, Request, Response, Status};

use tonic_book::{
    autogen::route::{Feature, Point, Rectangle, RouteNote, RouteSummary},
    route_guide_server, GreeterService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("[::1]:8080"))
        .parse()?;
    let service = RouteGuideService {};
    Ok(Server::builder()
        .add_service(route_guide_server::RouteGuideServer::new(service))
        .add_service(GreeterService::build())
        .serve(addr)
        .await?)
}

#[derive(Debug)]
struct RouteGuideService;

#[tonic::async_trait]
impl route_guide_server::RouteGuide for RouteGuideService {
    async fn get_feature(&self, _req: Request<Point>) -> Result<Response<Feature>, Status> {
        todo!();
    }
    type ListFeaturesStream = mpsc::Receiver<Result<Feature, Status>>;
    async fn list_features(
        &self,
        _req: Request<Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        todo!();
    }
    async fn record_route(
        &self,
        _req: Request<tonic::Streaming<Point>>,
    ) -> Result<Response<RouteSummary>, Status> {
        todo!();
    }
    type RouteChatStream =
        Pin<Box<dyn Stream<Item = Result<RouteNote, Status>> + Send + Sync + 'static>>;
    async fn route_chat(
        &self,
        _req: Request<tonic::Streaming<RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        todo!();
    }
}
