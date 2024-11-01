use axum::{routing::get, Router};
use telemetry::{info, tracing_init};

#[tokio::main]
async fn main() {
    tracing_init();

    let app = Router::new().route("/", get("div176"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
