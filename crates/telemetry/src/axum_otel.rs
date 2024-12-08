use std::time::Duration;

use axum::{extract::MatchedPath, response::Response};
use http::{header, Request, Version};
use opentelemetry::trace::SpanKind;
use tower_http::{
    classify::{ServerErrorsAsFailures, ServerErrorsFailureClass, SharedClassifier},
    trace::{MakeSpan, OnBodyChunk, OnEos, OnFailure, OnRequest, OnResponse, TraceLayer},
};
use tracing::{debug, Span};

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
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let matched_route = request
            .extensions()
            .get::<MatchedPath>()
            .map(MatchedPath::as_str);

        tracing::info_span!(
            "http_request",
            "otel.name" = format!("{} {}", request.method(), matched_route.unwrap_or("UnknownRoute")),
            "otel.kind" = format!("{:?}", SpanKind::Server),
            "error.type" = tracing::field::Empty,

            "http.flavor" = match request.version() {
                Version::HTTP_09 => "0.9",
                Version::HTTP_10 => "1.0",
                Version::HTTP_11 => "1.1",
                Version::HTTP_2 => "2.0",
                Version::HTTP_3 => "3.0",
                _ => "Unknown",
            },
            "http.host" = request.headers() .get(header::HOST).map_or("", |h| h.to_str().unwrap_or("")),
            "http.request.content_length" = request.headers().get(header::CONTENT_LENGTH).and_then(|val| val.to_str().ok()),
            "http.request.method" = ?request.method(),
            "http.response.status_code" = tracing::field::Empty,
            "http.route" = matched_route,

            "url.path" = request.uri().path(),
            "url.query" = request.uri().query(),
            "url.scheme" = request.uri().scheme().map_or("http".to_string(), |s| s.to_string()),
            "user_agent.original" = request.headers().get(header::USER_AGENT).map_or("", |h| h.to_str().unwrap_or("")),
        )
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
    fn on_response(self, response: &Response<B>, _: Duration, span: &Span) {
        span.record(
            "http.response.status_code",
            response.status().as_u16() as i64,
        );
        if response.status().is_server_error() {
            span.record("otel.status_code", "ERROR");
        }
        debug!("Request served");
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
    fn on_failure(&mut self, error: ServerErrorsFailureClass, _: Duration, span: &Span) {
        span.record("otel.status_code", "ERROR");
        span.record("error.type", error.to_string());
        debug!("Request errored");
    }
}
