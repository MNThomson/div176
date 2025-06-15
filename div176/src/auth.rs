use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::CookieJar;
use components::Layout;
use hypertext::*;
use tracing::info_span;

use crate::AppState;

pub const AUTH_COOKIE: &str = "authorization";

pub struct AuthedUser {
    pub session_token: String,
    //pub user: User,
    //pub permissions,
}

pub struct AuthUser(pub AuthedUser);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Redirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let root_span = tracing::Span::current();
        let _span = info_span!("AuthUser Extractor").entered();

        let auth_token = CookieJar::from_headers(&parts.headers)
            .get(AUTH_COOKIE)
            .ok_or(Redirect::temporary("/login"))?
            .value_trimmed()
            .to_string();

        let _db = AppState::from_ref(state).db;

        root_span.record("user.id", "testuser");

        Ok(AuthUser(AuthedUser {
            session_token: auth_token,
        }))
    }
}

pub async fn login_page() -> impl IntoResponse {
    Layout(rsx!(
            <main class="desktop:flex py-4 mobile:px-5 desktop:px-8 max-w-6xl w-full mx-auto desktop:space-x-12 bg-white">
                <h1 class="desktop:hidden text-xl font-medium text-center mb-4">Division 176 (Victoria)</h1>
                <div id="login" class="flex-none desktop:w-64 mobile:w-full">
                    <h2 class="text-white text-center font-medium p-2 bg-green-light rounded-t-lg">Volunteer Login</h2>
                    <form class="p-4 rounded-b-lg space-y-4 border border-green-light">
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
