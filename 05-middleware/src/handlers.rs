use axum::response::IntoResponse;
use tracing::info;

pub async fn hello() -> impl IntoResponse {
    info!("hello handler");
    "Hello, middleware"
}

pub async fn slow() -> impl IntoResponse {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    "Slow response"
}

pub async fn fallback() -> &'static str {
    "Not found"
}
