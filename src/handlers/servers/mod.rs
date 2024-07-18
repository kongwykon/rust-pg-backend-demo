use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod create_server;
pub mod list_servers;
pub mod delete_server;

// req & res

#[derive(Deserialize, Debug)]
pub struct CreateServerQuery {
    ip: String,
    name: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ServerResponse {
    id:String,
    ip: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListServersResponse {
    servers: Vec<ServerResponse>,
}

#[derive(Deserialize)]
pub struct DeleteServerQuery {
    id: Uuid,
}