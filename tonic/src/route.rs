//! [RouteGuideService]
//!
//! [routeguideservice]: https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
use std::{pin::Pin, sync::Arc};

use futures_channel::mpsc;
use futures_core::stream::Stream;
use tonic::{Request, Response, Status};

use crate::{
    autogen::route::{Feature, Point, Rectangle, RouteNote, RouteSummary},
    RouteGuide, RouteGuideServer,
};

#[derive(Debug)]
pub struct RouteGuideService {
    features: Arc<Vec<Feature>>,
}

impl RouteGuideService {
    pub fn new() -> RouteGuideServer<Self> {
        RouteGuideServer::new(Self::default())
    }
}

impl Default for RouteGuideService {
    fn default() -> Self {
        let features = crate::data::load().expect("cannot load features data");
        Self {
            features: Arc::new(features),
        }
    }
}

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
