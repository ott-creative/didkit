#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

use axum::{
    routing::{get, post},
    AddExtensionLayer, Router,
};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod dto;
mod error;
mod extractors;
mod handlers;
mod model;
mod response;
mod service;
mod sql;
mod utils;

pub mod config;

pub fn app(pg_pool: PgPool) -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(AddExtensionLayer::new(pg_pool))
        .into_inner();

    let auth_api = Router::new()
        .route("/login", post(handlers::login))
        .route("/register", post(handlers::register));
    let vc_api = Router::new().route("/template", post(handlers::vc_tplt_create));

    Router::new()
        .nest("/api/:v/auth", auth_api)
        .nest("/api/:v/vc", vc_api)
        .route("/api/:v/echo", get(handlers::echo))
        .layer(middleware_stack)
}
