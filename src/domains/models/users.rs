use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub pw_hash: String,
}
