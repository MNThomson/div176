use anyhow::anyhow;
use argon2::{
    Algorithm, Argon2, Params,
    password_hash::{self, PasswordHash, PasswordVerifier},
};
use axum::{
    Form, async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{HeaderName, StatusCode, header, request::Parts},
    response::{AppendHeaders, IntoResponse},
};
use axum_extra::extract::CookieJar;
use components::Layout;
use hypertext::*;
use rand::{Rng, distr::Alphanumeric};
use serde::{Deserialize, Serialize};
use tracing::{Instrument, info_span};
use types::Error;

use crate::AppState;

pub const AUTH_COOKIE: &str = "authorization";

pub struct Ctx {
    pub user_id: i32,
    //pub user: User,
    //pub permissions,
}

pub struct AuthUser(pub Ctx);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user_id = async move {
            let auth_token = CookieJar::from_headers(&parts.headers)
                .get(AUTH_COOKIE)
                .ok_or(Error::Unauthorized)?
                .value_trimmed()
                .to_string();

            let db = AppState::from_ref(state).db;

            let user_id = db
                .users
                .get_userid_from_session(&auth_token)
                .await?
                .ok_or(Error::Unauthorized)?;
            Ok::<i32, Error>(user_id)
        }
        .instrument(info_span!("AuthUser Extractor"))
        .await?;

        tracing::Span::current().record("user.id", user_id);

        Ok(AuthUser(Ctx { user_id }))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginModel {
    username: String,
    password: String,
}

#[tracing::instrument(skip(state, body))]
pub async fn login(
    State(state): State<AppState>,
    Form(body): Form<LoginModel>,
) -> Result<impl IntoResponse, Error> {
    let (uid, password_hash) = state
        .db
        .users
        .get_userid_password_from_username(&body.username)
        .await?
        .ok_or(Error::Unauthorized)?;

    hasher()
        .verify_password(
            body.password.as_bytes(),
            &PasswordHash::new(&password_hash)
                .map_err(|e| anyhow!("stored password hash to be valid: {}", e))?,
        )
        .map_err(|e| match e {
            password_hash::Error::Password => Error::Unauthorized,
            _ => anyhow!("verifying password failed: {}", e).into(),
        })?;

    let token: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    state.db.users.create_session(uid, &token).await?;

    Ok((
        StatusCode::OK,
        AppendHeaders([
            // TODO: Add HttpOnly/security flags
            (header::SET_COOKIE, format!("{}={}", AUTH_COOKIE, token)),
            (HeaderName::from_static("hx-redirect"), String::from("/")),
        ]),
    ))
}

pub fn hasher() -> Argon2<'static> {
    // HACK: These values are insecure
    Argon2::new(
        Algorithm::Argon2id,
        argon2::Version::V0x13,
        Params::new(1024, 1, 1, Some(Params::DEFAULT_OUTPUT_LEN)).expect("correct Argon2 params"),
    )
}

#[tracing::instrument]
pub async fn login_page() -> impl IntoResponse {
    Layout(rsx!(
            <main class="desktop:flex py-4 mobile:px-5 desktop:px-8 max-w-6xl w-full mx-auto desktop:space-x-12 bg-white">
                <h1 class="desktop:hidden text-xl font-medium text-center mb-4">Division 176 (Victoria)</h1>
                <div id="login" class="flex-none desktop:w-64 mobile:w-full">
                    <h2 class="text-white text-center font-medium p-2 bg-green-light rounded-t-lg">Volunteer Login</h2>
                    <form hx-post="/login" class="p-4 rounded-b-lg space-y-4 border border-green-light">
                        <label
                            for="Username"
                            class="relative block py-1 pl-3 border border-neutral rounded shadow-sm focus-within:border-green focus-within:ring-1 focus-within:ring-green"
                            >
                            <input
                                type="text"
                                id="Username"
                                name="username"
                                class="peer border-none bg-white placeholder-transparent focus:border-transparent focus:outline-none focus:ring-0 placeholder:opacity-0"
                                placeholder="Username"
                                />
                            <span
                                class="pointer-events-none absolute start-2.5 top-0 -translate-y-1/2 bg-white p-0.5 text-xs text-neutral transition-all peer-focus:text-black peer-placeholder-shown:top-1/2 peer-placeholder-shown:text-sm peer-focus:top-0 peer-focus:text-xs">
                                Username
                            </span>
                        </label>
                        <label
                            for="Password"
                            class="relative block py-1 pl-3 border border-neutral rounded shadow-sm focus-within:border-green focus-within:ring-1 focus-within:ring-green"
                            >
                            <input
                                type="password"
                                id="Password"
                                name="password"
                                class="peer border-none bg-white placeholder-transparent focus:border-transparent focus:outline-none focus:ring-0 placeholder:opacity-0"
                                placeholder="Password"
                                />
                            <span
                                class="pointer-events-none absolute start-2.5 top-0 -translate-y-1/2 bg-white p-0.5 text-xs text-neutral peer-focus:text-black transition-all peer-placeholder-shown:top-1/2 peer-placeholder-shown:text-sm peer-focus:top-0 peer-focus:text-xs">
                                Password
                            </span>
                        </label>
                        <div>
                            <input class="bg-green text-white px-2 py-0.5 shadow-sm rounded hover:bg-green-light cursor-pointer" type="submit" value="Login">
                        </div>
                    </form>
                </div>
                <div>
                    <h1 class="mobile:hidden text-xl font-medium text-center">"St. John Ambulance - Division 176 (Victoria)"</h1>
                    <div class="space-y-2 mt-4">
                        <h2 class="font-medium">Need First Aid Coverage for Your Event?</h2>

                        <p>For information on the status of your event request and general inquiries, please <a class="text-green font-medium" href="https://www.cognitoforms.com/StJohnAmbulanceVictoriaBrigade/StJohnAmbulanceVictoriaGeneralInquiryToCommunityServices">send us a message</a>.</p>

                        <p>To submit a request for first aid coverage for your event, please fill out a <a class="text-green font-medium" href="https://www.cognitoforms.com/StJohnAmbulanceVictoriaBrigade/RequestFirstAidCoverageForYourEvent"> Request Form</a>.</p>
                    </div>
                    <div class="space-y-2 mt-4">
                        <h2 class="font-medium">Want to Volunteer with us?</h2>

                        <p>We are excited to hear from you! If you want to volunteer with us please fill out an <a class="text-green font-medium" href="https://www.cognitoforms.com/StJohnAmbulanceVictoriaBrigade/StJohnAmbulanceVictoriaBCVolunteerApplication"> Application Form</a>.</p>

                        <p>For further recruiting inquiries please <a class="text-green font-medium" href="https://www.cognitoforms.com/StJohnAmbulanceVictoriaBrigade/StJohnAmbulanceVictoriaBrigadeGeneralInquiryToRecruitment">send us a message</a>.</p>
                    </div>
                </div>
            </main>
    )).render()
}
