//! Greeter service
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

use tonic::{Request, Response, Status};

use crate::{Greeter, GreeterServer, HelloRequest, HelloResponse};

#[derive(Debug, Default)]
pub struct GreeterService {
    counter: AtomicUsize,
}

impl GreeterService {
    pub fn new() -> GreeterServer<Self> {
        GreeterServer::new(Self::default())
    }
}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        req: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let counter = self.counter.fetch_add(1, SeqCst);
        println!("[{}]: got a request", counter);
        let resp = HelloResponse {
            message: format!("Hello {}#{}!", req.into_inner().name, counter),
        };
        Ok(Response::new(resp))
    }
}
