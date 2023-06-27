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

impl Default for Config {
    fn default() -> Self {
        Self {
            listen: String::from("0.0.0.0:9991"),
        }
    }
}

fn readin_config() -> Config {
    debug!("reading config file: config.yaml");
    if let Ok(f) = File::open("./config.yaml") {
        serde_yaml::from_reader::<_, Config>(&f).unwrap()
    } else {
        let defalut_config = Config::default();
        warn!(
            "`{{config.yaml}}` not found, using default config, listening at {}",
            defalut_config.listen
        );
        defalut_config
    }
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
