use axum::response::Redirect;

use crate::user::UserCookie;

pub(crate) async fn handler(user_cookie_result: Result<UserCookie, &'static str>) -> Redirect {
    match user_cookie_result {
        Ok(_) => Redirect::to("/inbox"),
        Err(_) => Redirect::to("/login"),
    }
}
