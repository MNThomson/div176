use axum::response::IntoResponse;
use components::{Layout, PageUnderConstruction};
use hypertext::*;

use crate::auth::AuthUser;

#[tracing::instrument(skip(_ctx))]
pub async fn home(AuthUser(_ctx): AuthUser) -> impl IntoResponse {
    Layout(PageUnderConstruction()).render()
}
