use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{domains::models::server::ServerError, infra::respository, AppState};

pub async fn delete_server(
    State(state): State<Arc<AppState>>,
    Path(server_id): Path<Uuid>,
) -> Result<(), ServerError> {
    let delete_id = server_id;
    let _ = respository::servers::delete(&state.pool, delete_id)
        .await
        .map_err(ServerError::InfraError)?;

    return Ok(());
}
