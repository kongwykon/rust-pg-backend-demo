use axum::extract::{rejection::JsonRejection, FromRequest};
use tracing::debug;

use crate::errors::AppError;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct JsonExtractor<T>(pub T);

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        debug!("JsonRejection: {:?}", rejection);

        AppError::BodyParsingError(rejection.to_string())
    }
}
