use std::any::Any;

use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use components::Layout;
use db::{Database, DB};
use hypertext::*;
use telemetry::{error, info, otel_tracing, tracing_init};
use tower_http::catch_panic::CatchPanicLayer;

#[derive(Clone)]
struct AppState {
    db: DB,
}

#[tokio::main]
async fn main() {
    tracing_init(env!("CARGO_PKG_NAME"), env!("GIT_HASH"));

    let state = AppState {
        db: DB::init().await.unwrap(),
    };

    let app = Router::new()
        .route("/", get(Layout(rsx!(<h1>div176</h1>)).render()))
        .layer(otel_tracing())
        .route("/health", get(healthcheck))
        .route("/version", get(|| async { env!("GIT_HASH") }))
        .with_state(state)
        .layer(CatchPanicLayer::custom(handle_panic));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn healthcheck(State(state): State<AppState>) -> impl IntoResponse {
    let is_healthy = state.db.healthcheck().await.is_ok();
    let health_status = if is_healthy { "healthy" } else { "unhealthy" };
    let statuscode = if is_healthy {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    (statuscode, rsx!(<p>database: {health_status}</p>).render())
}

fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response<Body> {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic message".to_string()
    };
    error!(details);

    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
}
