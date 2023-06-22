use axum::http::StatusCode;

pub async fn pong() -> (StatusCode, String) {
    (StatusCode::OK, "Pong".to_owned())
}
