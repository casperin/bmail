use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;

pub(crate) struct UserCookie {
    pub user_id: String,
    pub name: String,
}

impl<S: Send + Sync> FromRequestParts<S> for UserCookie {
    type Rejection = &'static str;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);

        let user_id = jar
            .get("user_id")
            .ok_or("Missing 'user_id' cookie")?
            .value()
            .to_string();

        let name = jar
            .get("name")
            .ok_or("Missing 'name' cookie")?
            .value()
            .to_string();

        Ok(UserCookie { user_id, name })
    }
}
