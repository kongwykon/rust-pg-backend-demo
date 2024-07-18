use std::sync::Arc;

use crate::schema::users;
use axum::{
    body::Body,
    extract::State,
    http::{header::SET_COOKIE, HeaderName, HeaderValue, Response, StatusCode},
    response::IntoResponse,
    Json,
};
use bcrypt::BcryptError;
use diesel::deserialize::Queryable;
use diesel::prelude::*;
use headers::SetCookie;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    infra::{errors::InfraError, respository},
    utils::JsonExtractor,
    AppState,
};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = users )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub pw_hash: Vec<u8>,
    pub username: String,
}

#[derive(Clone)]
struct Backend {
    state: Arc<AppState>,
}

#[derive(Serialize, Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SignUpRequestModel {
    pub username: String,
    pub pw_hash: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, Queryable, Selectable, PartialEq)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SignInRequestModel {
    pub username: String,
    pub pw_hash: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub pw: String,
}
#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    pub username: String,
    pub pw: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize)]
pub struct JwtResponse {
    pub token: String,
}

#[derive(Debug)]
pub enum SignUpError {
    InternalServerError,
    NotFound,
    InfraError(InfraError),
    PasswordHashError(BcryptError),
    UsernameAlreadyExists,
}

#[derive(Debug)]
pub enum SignOutError {
    InfraError(InfraError),
}
#[derive(Debug)]
pub enum SignInError {
    InternalServerError,
    NotFound,
    InfraError(InfraError),
    InvalidCredentials(BcryptError),
}

impl IntoResponse for SignOutError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InfraError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
        };
        (
            status,
            Json(
                json!({"resource":"User", "message": err_msg, "happened_at" : chrono::Utc::now() }),
            ),
        )
            .into_response()
    }
}
impl IntoResponse for SignInError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, format!("User has not been found")),
            Self::InfraError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
            Self::InvalidCredentials(err) => {
                (StatusCode::UNAUTHORIZED, format!("Invalid credentials"))
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };
        (
            status,
            Json(
                json!({"resource":"User", "message": err_msg, "happened_at" : chrono::Utc::now() }),
            ),
        )
            .into_response()
    }
}

pub async fn sign_up(
    State(state): State<Arc<AppState>>,
    JsonExtractor(sign_up_json): JsonExtractor<SignUpRequest>,
) -> Result<Response<Body>, SignUpError> {
    let result = respository::user::sign_up(&state.pool, sign_up_json).await?;
    Ok(result.into())
}
pub async fn sign_in(
    State(state): State<Arc<AppState>>,
    JsonExtractor(sign_in_json): JsonExtractor<SignInRequest>,
) -> Result<Response<Body>, SignInError> {
    let resutl = respository::user::sign_in(&state.pool, sign_in_json).await?;
    Ok(resutl.into())
}
pub async fn sign_out(State(_): State<Arc<AppState>>) -> Result<Response<Body>, SignOutError> {
    let cookie_value = "session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly";
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(SET_COOKIE, HeaderValue::from_static(cookie_value))
        .header("HX-Refresh", "true")
        .body(Body::from("Logged out"))
        .unwrap();
    Ok(response)
}

impl IntoResponse for SignUpError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, format!("User has not been found")),
            Self::InfraError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };
        (
            status,
            Json(
                json!({"resource":"User", "message": err_msg, "happened_at" : chrono::Utc::now() }),
            ),
        )
            .into_response()
    }
}
