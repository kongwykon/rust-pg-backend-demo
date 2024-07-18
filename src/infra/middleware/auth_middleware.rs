use axum::{
    extract::Request,
    http::{self, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey, Validation};

use crate::{handlers::user::Claims, infra::respository::user::JWT_SECRET, AppState};

pub async fn jwt_token_check(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    tracing::info!("Checking JWT token: {:?}", &req);
    let cookie_jar = CookieJar::from_headers(&req.headers());

    tracing::info!("cookie_jar: {:?}", cookie_jar);
    let session_cookie = cookie_jar.get("session");
    tracing::info!("session cookie: {:?}", session_cookie);

    let jwt_token = if let Some(cookie) = session_cookie {
        cookie.value()
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(current_user) = authorize_current_user(jwt_token).await {
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub user_id: String,
}

async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    let decoding_key = DecodingKey::from_secret(JWT_SECRET.as_ref());
    match decode::<Claims>(&auth_token, &decoding_key, &Validation::default()) {
        Ok(token_data) => Some(CurrentUser {
            user_id: token_data.claims.sub.to_string(),
        }),
        Err(err) => {
            match *err.kind() {
                ErrorKind::InvalidToken => println!("Token is invalid"),
                _ => println!("Some other errors"),
            };
            None
        }
    }
}
