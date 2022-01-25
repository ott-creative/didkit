#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

use axum::{
    routing::{get, post},
    AddExtensionLayer, Router, Server,
};
use futures::Future;
use hyper;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use std::net::SocketAddr;

use clap::Parser;

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

fn app(pg_pool: PgPool) -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(AddExtensionLayer::new(pg_pool))
        .into_inner();

    let auth_api = Router::new()
        .route("/login", post(handlers::user::login))
        .route("/register", post(handlers::user::register));
    let vc_api = Router::new().route("/template", post(handlers::vc::vc_tplt_create));
    let did_api = Router::new().route("/create", post(handlers::did::did_create));

    Router::new()
        .route("/api/:v/health_check", get(handlers::health_check))
        .nest("/api/:v/auth", auth_api)
        .nest("/api/:v/vc", vc_api)
        .nest("/api/:v/did", did_api)
        .layer(middleware_stack)
}

pub fn server(pg_pool: PgPool) -> impl Future<Output = hyper::Result<()>> {
    let config = config::env::ServerConfig::parse();
    let addr = SocketAddr::from((config.host, config.port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app(pg_pool).into_make_service())
}
