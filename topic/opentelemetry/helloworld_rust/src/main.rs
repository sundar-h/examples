mod greeter;
mod grpc_meter;
mod grpc_trace;
mod trace;

#[macro_use]
extern crate log;

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

use actix_web::{App, HttpResponse, HttpServer};
use std::error::Error;

use crate::{greeter::MyGreeter, trace::tracing_init};
use helloworld::greeter_server::GreeterServer;
use opentelemetry::global;
// use tonic::service::Interceptor;
// use env_logger::Env;
use crate::grpc_meter::{run, GrpcMeter};
use crate::grpc_trace::GrpcTrace;
use crate::trace::with_grpc_span;
use log::{debug, error, info, log_enabled, trace, Level};
use tonic::{transport::Server, Request, Status};
use tracing::Instrument;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing_init();

    let addr = "127.0.0.1:8102".parse().unwrap();
    let greeter = MyGreeter::default();
    let greeter = GreeterServer::new(greeter);
    // let greeter = GreeterServer::with_interceptor(greeter, intercept);
    // let greeter = tonic_web::config()
    //     .allow_origins(vec!["127.0.0.1"])
    //     .enable(greeter);
    let greeter = GrpcTrace::new(greeter);
    let svc = run();
    tokio::spawn(async move { svc.await });
    let greeter = GrpcMeter::new(greeter);

    // tonic_web::enable(greeter);
    // let a = Svc {};

    // GreeterServer::new(greeter)
    println!("start server...");
    Server::builder()
        // .trace_fn(add_span3)
        // .accept_http1(true)
        .add_service(greeter)
        .serve(addr)
        .await?;

    global::shutdown_tracer_provider(); // export remaining spans
    Ok(())
}
