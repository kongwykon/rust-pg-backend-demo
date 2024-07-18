use axum::response::IntoResponse;
use serde::Serialize;

use crate::infra::errors::InfraError;

#[derive(Debug, Clone)]
pub struct ServerModel {
    pub id: String,
    pub name: String,
    pub ip: String,
}

pub enum ServerError {
    InfraError(InfraError),
    InternalServerError,
    NotFound,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound => (
                axum::http::StatusCode::NOT_FOUND,
                format!("ServerModel with id has not been found"),
            ),
            Self::InfraError(err) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", err),
            ),
            _ => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };
        (
            status,
            axum::Json(
                serde_json::json!({"resource":"ServerModel", "message": err_msg, "happened_at" : chrono::Utc::now() }),
            ),
        )
            .into_response()
    }
}
