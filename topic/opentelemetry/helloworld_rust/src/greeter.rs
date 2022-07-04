// use helloworld::greeter_server::Greeter;
use crate::helloworld::greeter_server::Greeter;
use crate::helloworld::{HelloReply, HelloRequest};
use std::fmt::Debug;
use tonic::{Code, Request, Response, Status};
use tracing::instrument;

use crate::trace::with_grpc_span;

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    #[instrument]
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        println!("received new request...");
        with_grpc_span(request.metadata());

        let name = request.into_inner().name;

        if name == "err" {
            return Err(Status::new(
                Code::Internal,
                "caught error from rust backend",
            ));
        }

        // Return an instance of type HelloReply
        let reply = HelloReply {
            message: format!("Hello {}!", name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
