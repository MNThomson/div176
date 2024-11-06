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
    tracing_init(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let state = AppState { db: DB {} };

    let app = Router::new()
        .route("/", get(rsx!(<h1>div176</h1>).render()))
        .layer(otel_tracing())
        .route("/health", get(healthcheck))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn healthcheck(State(state): State<AppState>) -> impl IntoResponse {
    let health = state.db.healthcheck().map_or("unhealthy", |_| "healthy");
    let statuscode = state
        .db
        .healthcheck()
        .map_or(StatusCode::INTERNAL_SERVER_ERROR, |_| StatusCode::OK);
    (statuscode, rsx!(<p>database: {health}</p>).render())
}
