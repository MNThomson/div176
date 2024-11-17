mod axum_otel;
mod tracing_otel;

pub use axum_otel::otel_tracing;
pub use tracing::{debug, error, info, trace, warn};
pub use tracing_otel::tracing_init;
