//! 大学牲体育成绩计算器

#[macro_use]
extern crate tracing;

mod api;
mod router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    axum::Server::bind(&router::listen_addr())
        .serve(router::route().into_make_service())
        .await
        .unwrap();
}
