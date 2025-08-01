use std::any::Any;

use auth::login_page;
use axum::{
    Router,
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
};
use components::Layout;
use db::{DB, Database, embedded_db};
use hypertext::*;
use telemetry::{otel_tracing, tracing_init};
use tokio::signal;
use tower_http::{catch_panic::CatchPanicLayer, compression::CompressionLayer};
use tracing::{error, info};

use crate::{auth::AuthUser, r#static::static_handler};

mod auth;
mod r#static;

#[derive(Clone)]
struct AppState {
    db: DB,
}

#[tokio::main]
async fn main() {
    tracing_init(env!("CARGO_PKG_NAME"), env!("GIT_HASH"));

    let (url, _pg) = embedded_db().await;

    let state = AppState {
        db: DB::init(&url).await.unwrap(),
    };

    let app = Router::new()
        .route("/", get(Layout(rsx!(<div>div176</div>)).render()))
        .route("/login", get(login_page))
        .route("/static/*file", get(static_handler))
        .route("/protected", get(protected))
        .layer(otel_tracing())
        .route("/health", get(healthcheck))
        .route("/version", get(|| async { env!("GIT_HASH") }))
        .route(
            "/robots.txt",
            get(|| async { "User-agent: *\nAllow: /$\nDisallow: /" }),
        )
        .fallback(fallback_404)
        .with_state(state);

    #[cfg(debug_assertions)]
    let app = app.layer(
        tower_livereload::LiveReloadLayer::new()
            //.request_predicate(|req: &Request<_>| !req.headers().contains_key("hx-request"))
            .reload_interval(std::time::Duration::from_millis(100)),
    );

    let app = app
        .layer(CompressionLayer::new())
        .layer(CatchPanicLayer::custom(handle_panic));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

pub async fn fallback_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Layout(rsx!(
            <div class="items-center px-2 text-center mt-8">
                <h1 class="text-8xl font-extrabold text-red">404</h1>
                <h1 class="text-4xl font-extrabold text-red">Page Not Found</h1>
                <p class="text-xl mt-4">"Oops. It seems like the page you're looking for does not exist"</p>
                <p class="mt-2">"(Have you checked the back of Dussault's Jeep?)"</p>
            </div>
        )).render())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tracing::instrument(skip(user))]
async fn protected(AuthUser(user): AuthUser) -> impl IntoResponse {
    user.session_token
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
