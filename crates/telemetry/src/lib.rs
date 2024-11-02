use std::time::Duration;

use axum::response::Response;
use gethostname::gethostname;
use http::Request;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use rustc_version;
use tower_http::{
    classify::{ServerErrorsAsFailures, ServerErrorsFailureClass, SharedClassifier},
    trace::{MakeSpan, OnBodyChunk, OnEos, OnFailure, OnRequest, OnResponse, TraceLayer},
};
use tracing::Span;
pub use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn tracing_init(crate_name: &str) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                return format!(
                    "none,{}=debug,{}=debug,axum::rejection=trace",
                    crate_name,
                    env!("CARGO_PKG_NAME")
                )
                .into();
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_opentelemetry::layer().with_tracer(
                opentelemetry_otlp::new_pipeline()
                    .tracing()
                    .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_endpoint(""))
                    .with_trace_config(sdktrace::config().with_resource(Resource::new(vec![
                            KeyValue::new("service.name", env!("CARGO_PKG_NAME")),
                            KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
                            KeyValue::new("process.runtime.name", "rustc"),
                            KeyValue::new(
                                "process.runtime.version",
                                rustc_version::version().unwrap().to_string(),
                            ),
                            KeyValue::new("process.command", std::env::args().next().unwrap()),
                            KeyValue::new(
                                "process.command_line",
                                std::env::args().collect::<Vec<_>>().join(" "),
                            ),
                            KeyValue::new(
                                "process.executable.name",
                                std::env::current_exe()
                                    .unwrap()
                                    .file_name()
                                    .unwrap()
                                    .to_string_lossy()
                                    .into_owned(),
                            ),
                            KeyValue::new(
                                "process.executable.path",
                                std::env::current_exe()
                                    .unwrap()
                                    .display()
                                    .to_string(),
                            ),
                            KeyValue::new("process.pid", std::process::id() as i64),
                            KeyValue::new("host.arch", std::env::consts::ARCH),
                            KeyValue::new("host.name", gethostname().into_string().unwrap()),
                        ])))
                    .install_batch(runtime::Tokio)
                    .unwrap(),
            ),
        )
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
