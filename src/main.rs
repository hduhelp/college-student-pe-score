//! 大学牲体育成绩计算器

use axum::{
    routing::{get, post},
    Router,
};

mod api;

use api::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", post(calculate_pe_score))
        .route("/ping", get(pong));

    let addr = "127.0.0.1:9991".parse().unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
