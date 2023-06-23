//! 大学牲体育成绩计算器

use axum::{
    routing::{get, post},
    Router,
};

#[macro_use]
extern crate tracing;

mod api;

use api::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", post(calculate_pe_score))
        .route("/ping", get(pong));

    let addr = "127.0.0.1:9991".parse().unwrap();

    info!("be ready to serve");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
