use std::sync::Arc;

use axum::{
    body::Body, extract::{Query, State}, http::{HeaderName, HeaderValue, Response}, response::IntoResponse, Json
};

use super::{CreateServerQuery, ServerResponse};
use crate::infra::respository;
use crate::{domains::models::server::ServerError, utils::JsonExtractor, AppState};

pub async fn create_server(
    State(state): State<Arc<AppState>>,
    JsonExtractor(new_serser): JsonExtractor<CreateServerQuery>,
) -> Result<impl IntoResponse, ServerError> {
    tracing::info!("Creating a new server: {:?}", new_serser);
    let new_server_db = respository::servers::NewServerDB {
        ip: new_serser.ip,
        name: new_serser.name,
    };

    let created_server = respository::servers::insert(&state.pool, new_server_db)
        .await
        .map_err(ServerError::InfraError)?;

    let server_response = ServerResponse {
        id: created_server.id.to_string(),
        ip: created_server.ip,
        name: created_server.name,
    };

    let response = Response::builder()
        .header("HX-Trigger", "AddServerDone")
        .body(Body::from(serde_json::to_string(&server_response).unwrap()))
        .unwrap();

    Ok(response)
}
