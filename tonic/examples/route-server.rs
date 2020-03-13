//! [Route] guide server
//!
//! [route]: https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
use std::{error, pin::Pin};

use futures_channel::mpsc;
use futures_core::stream::Stream;
use tonic::{Request, Response, Status};

use tonic_book::{
    route::{Feature, Point, Rectangle, RouteNote, RouteSummary},
    route_guide_server::RouteGuide,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let svc = RouteGuideService {};
    println!("RouteGuideService = {:?}!", svc);
    Ok(())
}

#[derive(Debug)]
struct RouteGuideService;

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
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
