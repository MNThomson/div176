use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use db::{Database, DB};
use rsx::*;
use telemetry::{info, otel_tracing, tracing_init};
extern crate rsx as hypertext;

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
        .route("/", get(rsx!(<h1>div176</h1>).render()))
        .layer(otel_tracing())
        .route("/health", get(healthcheck))
        .route("/version", get(|| async { env!("GIT_HASH") }))
        .with_state(state);

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
