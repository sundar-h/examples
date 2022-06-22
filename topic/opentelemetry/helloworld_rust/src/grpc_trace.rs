use crate::trace::with_http_span;
use http::{HeaderValue, Method, Request, Response, StatusCode, Version};
use hyper::Body;
use std::task::{Context, Poll};
use tonic::body::BoxBody;
use tonic::codegen::{BoxFuture, StdError};
use tonic::transport::NamedService;
use tonic::IntoRequest;
use tower_service::Service;

#[derive(Debug, Clone)]
pub struct GrpcTrace<S> {
    inner: S,
}

impl<S> GrpcTrace<S> {
    pub(crate) fn new(inner: S) -> Self {
        GrpcTrace { inner }
    }
}

impl<S> Service<Request<Body>> for GrpcTrace<S>
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
        // 头部信息都是有的，在这里不起作用
        let headers = req.headers().clone();
        let fut = self.inner.call(req);

        println!("call trace...");
        let res = Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        });
        with_http_span(&headers);
        res
    }
}

impl<S: NamedService> NamedService for GrpcTrace<S> {
    const NAME: &'static str = S::NAME;
}
