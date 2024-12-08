use axum::{
    http::{header, StatusCode, Uri},
    response::IntoResponse,
};
use mime_guess::from_path;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "src/static/"]
#[prefix = "/static/"]
struct Asset;

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path();

    match Asset::get(path) {
        Some(content) => {
            let mime = from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}
