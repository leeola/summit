use crate::uuid::RequestId;
use http::Request;
use hyper::Body;
use std::task::{Context, Poll};
use tower_layer::Layer;
use tower_service::Service;
use tracing::{error_span, Span};

#[derive(Clone, Debug)]
pub struct RequestIdService<S>(S);
impl<B, S> Service<Request<B>> for RequestIdService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        let id = RequestId::new();
        req.extensions_mut().insert(id);
        self.0.call(req)
    }
}

#[derive(Clone, Debug)]
pub struct RequestIdLayer;
impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdService<S>;
    fn layer(&self, s: S) -> Self::Service {
        RequestIdService(s)
    }
}

pub fn trace_layer_span_with(request: &Request<Body>) -> Span {
    let id = request
        .extensions()
        .get::<RequestId>()
        .copied()
        .unwrap_or_default();
    error_span!(
        "request",
        %id,
        method = %request.method(),
        uri = %request.uri(),
    )
}
