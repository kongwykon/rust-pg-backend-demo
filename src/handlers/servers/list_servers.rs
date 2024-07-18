use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::Extensions,
    Extension, Json,
};

use crate::{
    domains::models::server::{ServerError, ServerModel},
    infra::{
        middleware::auth_middleware::CurrentUser,
        respository::{self, servers::ServersFilter},
    },
    AppState,
};

use super::{ListServersResponse, ServerResponse};

pub async fn list_servers(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ServersFilter>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Json<ListServersResponse>, ServerError> {
    tracing::info!("list servers : {:?}", current_user);
    let servers = respository::servers::get_all(&state.pool, query)
        .await
        .map_err(|_| ServerError::InternalServerError)?;

    Ok(Json(adapt_servers_to_list_servers_response(servers)))
}

fn adapt_servers_to_list_servers_response(servers: Vec<ServerModel>) -> ListServersResponse {
    let servers_response: Vec<ServerResponse> = servers
        .into_iter()
        .map(adapt_server_to_server_response)
        .collect();

    ListServersResponse {
        servers: servers_response,
    }
}

fn adapt_server_to_server_response(server: ServerModel) -> ServerResponse {
    ServerResponse {
        id: server.id.to_string(),
        ip: server.ip,
        name: server.name,
    }
}
