use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{rt, web, App, HttpResponse, HttpServer, Responder};
use actix_web_opentelemetry::RequestTracing;
use http::{Request, Response, Version};
use hyper::Body;
use opentelemetry::metrics::UpDownCounter;
use opentelemetry::{global, KeyValue};
use prometheus::TextEncoder;
use std::task::{Context, Poll};
use tonic::body::BoxBody;
use tonic::codegen::{BoxFuture, StdError};
use tonic::transport::NamedService;
use tower_service::Service;

#[derive(Debug, Clone)]
pub struct GrpcMeter<S> {
    inner: S,
    total: UpDownCounter<i64>,
    // exporter: PrometheusExporter,
}

impl<S> GrpcMeter<S> {
    pub(crate) fn new(inner: S) -> Self {
        // let exporter = opentelemetry_prometheus::exporter().init();
        GrpcMeter {
            // exporter,
            inner,
            total: global::meter("lambda.demo")
                .i64_up_down_counter("pipeline_total")
                .with_description("the number of call")
                .init(),
        }
    }
}

pub fn run() -> Server {
    let metrics_exporter = opentelemetry_prometheus::exporter().init();
    let request_metrics = actix_web_opentelemetry::RequestMetrics::new(
        opentelemetry::global::meter("actix_http_tracing"),
        Some(|req: &actix_web::dev::ServiceRequest| {
            req.path() == "/metrics" && req.method() == actix_web::http::Method::GET
        }),
        Some(metrics_exporter),
    );
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(RequestTracing::new())
            .wrap(request_metrics.clone())
        // .service(web::resource("/users/{username}").to(index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
}

impl<S> Service<Request<Body>> for GrpcMeter<S>
where
    S: Service<Request<Body>, Response = Response<BoxBody>> + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<StdError> + Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let fut = self.inner.call(req);

        println!("call meter...");
        let res = Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        });
        self.total.add(1, &[KeyValue::new("key", "value")]);

        res
    }
}

impl<S: NamedService> NamedService for GrpcMeter<S> {
    const NAME: &'static str = S::NAME;
}
