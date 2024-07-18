use std::sync::Arc;

use axum::http::{StatusCode, Uri};
use axum::response::{Html, IntoResponse};
use axum::routing::{delete, get, get_service, post};
use axum::{middleware, Router};
use tower_http::trace::DefaultMakeSpan;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use crate::handlers::servers::create_server::create_server;
use crate::handlers::servers::delete_server::delete_server;
use crate::handlers::servers::list_servers::list_servers;
use crate::handlers::user::{sign_in, sign_out, sign_up};
use crate::infra::middleware::auth_middleware::jwt_token_check;
use crate::AppState;

pub fn app_router(state: Arc<AppState>) -> Router {
    Router::new()
        .nest_service("/", routes_static())
        .nest("/v1/servers", servers_routes(state.clone()))
        .nest("/v1/main_user", main_user_routers(state.clone()))
        .fallback(handler_404)
}

fn main_user_routers(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/sign_up", post(sign_up))
        .route("/sign_in", post(sign_in))
        .route("/sign_out", post(sign_out))
        .with_state(state)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
fn servers_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(create_server))
        .route("/", get(list_servers))
        .route("/:id", delete(delete_server))
        .with_state(state)
        .route_layer(middleware::from_fn(jwt_token_check))
}

fn routes_static() -> Router {
    Router::new()
        .nest_service("/", get_service(ServeDir::new("www/")))
        .nest_service("/sign_up", ServeFile::new("www/sign_up.html"))
        .nest_service("/sign_in", ServeFile::new("www/sign_in.html"))
}
