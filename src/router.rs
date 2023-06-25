use std::{fs::File, net::SocketAddr};

use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

use crate::api::{calculate_pe_score, pong};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    listen: String,
}

fn readin_config() -> Config {
    debug!("reading config file: config.yaml");
    let f = File::open("./config.yaml").expect("couldn't open config.yaml");
    serde_yaml::from_reader::<_, Config>(&f).unwrap()
}

pub fn listen_addr() -> SocketAddr {
    let config = readin_config();
    info!("will listen at: {}", config.listen);
    config.listen.parse().unwrap()
}

pub fn route() -> Router {
    info!("start service");
    Router::new()
        .route("/", post(calculate_pe_score))
        .route("/ping", get(pong))
}
