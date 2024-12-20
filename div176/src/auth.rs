use axum::{async_trait, extract::FromRequestParts, http::request::Parts, response::Redirect};
use axum_extra::extract::CookieJar;

const AUTH_COOKIE: &str = "authorization";

pub struct User {
    pub session_token: String,
}

pub struct AuthUser(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Redirect;

    #[tracing::instrument(name = "AuthUser Extractor", skip(parts, _state))]
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_token = CookieJar::from_headers(&parts.headers)
            .get(AUTH_COOKIE)
            .ok_or(Redirect::temporary("/login"))?
            .value_trimmed()
            .to_string();

        Ok(AuthUser(User {
            session_token: auth_token,
        }))
    }
}
