use axum::response::IntoResponse;
use tracing::info;

pub async fn health() -> impl IntoResponse {
    info!("health check");
    "ok"
}

pub async fn fallback() -> &'static str {
    "Not found"
}
