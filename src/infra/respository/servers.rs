use diesel::*;
use diesel::{deserialize::Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domains::models::server::ServerModel;
use crate::infra::errors::{adapt_infra_error, InfraError};
use crate::schema::servers;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = servers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ServerDB {
    pub id: Uuid,
    pub ip: String,
    pub name: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = servers)]
pub struct NewServerDB {
    pub name: String,
    pub ip: String,
}

#[derive(Deserialize)]
pub struct ServersFilter {
    name_contains: Option<String>,
    ip_contains: Option<String>,
}

#[derive(Serialize)]
pub struct AddOneNewServer {
    pub name: String,
    pub ip: String,
}

#[derive(Serialize)]
pub struct DeleteByName {
    pub name: String,
}

#[derive(Serialize)]
pub struct UpdateServer {
    pub name: String,
    pub ip: String,
}

#[derive(Serialize)]
pub struct DeleteServer {
    pub id: i32,
}

pub async fn insert(
    pool: &deadpool_diesel::postgres::Pool,
    new_server: NewServerDB,
) -> Result<ServerModel, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(servers::table)
                .values(new_server)
                .returning(ServerDB::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_server_db_to_server(res))
}

pub async fn get_all(
    pool: &deadpool_diesel::postgres::Pool,
    filter: ServersFilter,
) -> Result<Vec<ServerModel>, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let res = conn
        .interact(move |conn| {
            let mut query = servers::table.into_boxed::<diesel::pg::Pg>();

            if let Some(name_contains) = filter.name_contains {
                query = query.filter(servers::name.like(format!("%{}%", name_contains)));
            }

            if let Some(ip_contains) = filter.ip_contains {
                query = query.filter(servers::ip.like(format!("%{}%", ip_contains)));
            }

            query.select(ServerDB::as_select()).load::<ServerDB>(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    let servers: Vec<ServerModel> = res.into_iter().map(adapt_server_db_to_server).collect();
    Ok(servers)
}

fn adapt_server_db_to_server(server_db: ServerDB) -> ServerModel {
    ServerModel {
        id: server_db.id.to_string(),
        name: server_db.name,
        ip: server_db.ip,
    }
}

pub async fn delete(pool: &deadpool_diesel::postgres::Pool, id: Uuid) -> Result<(), InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let _ = conn
        .interact(move |conn| {
            diesel::delete(servers::table.filter(servers::id.eq(id))).execute(conn)
        })
        .await
        .map_err(adapt_infra_error)?;

    Ok(())
}
