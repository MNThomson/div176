use std::time::Duration;

use axum::response::Response;
use http::Request;
use tower_http::{
    classify::{ServerErrorsAsFailures, ServerErrorsFailureClass, SharedClassifier},
    trace::{MakeSpan, OnBodyChunk, OnEos, OnFailure, OnRequest, OnResponse, TraceLayer},
};
use tracing::Span;
pub use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn tracing_init() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub fn otel_tracing() -> TraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    OtelMakeSpan,
    OtelOnRequest,
    OtelOnResponse,
    OtelOnBodyChunk,
    OtelOnEos,
    OtelOnFailure,
> {
    TraceLayer::new_for_http()
        .make_span_with(OtelMakeSpan)
        .on_request(OtelOnRequest)
        .on_response(OtelOnResponse)
        .on_body_chunk(OtelOnBodyChunk)
        .on_eos(OtelOnEos)
        .on_failure(OtelOnFailure)
}

#[derive(Clone, Copy, Debug)]
pub struct OtelMakeSpan;
impl<B> MakeSpan<B> for OtelMakeSpan {
    fn make_span(&mut self, _req: &Request<B>) -> Span {
        tracing::info_span!("http_request")
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OtelOnRequest;
impl<B> OnRequest<B> for OtelOnRequest {
    fn on_request(&mut self, _: &Request<B>, _: &Span) {}
}

#[derive(Clone, Copy, Debug)]
pub struct OtelOnResponse;
impl<B> OnResponse<B> for OtelOnResponse {
    fn on_response(self, _: &Response<B>, _: Duration, _: &Span) {
        info!("");
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OtelOnBodyChunk;
impl<B> OnBodyChunk<B> for OtelOnBodyChunk {
    fn on_body_chunk(&mut self, _: &B, _: Duration, _: &Span) {}
}

#[derive(Clone, Copy, Debug)]
pub struct OtelOnEos;
impl OnEos for OtelOnEos {
    fn on_eos(self, _: Option<&http::HeaderMap>, _: Duration, _: &Span) {}
}

#[derive(Clone, Copy, Debug)]
pub struct OtelOnFailure;
impl OnFailure<ServerErrorsFailureClass> for OtelOnFailure {
    fn on_failure(&mut self, _: ServerErrorsFailureClass, _: Duration, _: &Span) {}
}
