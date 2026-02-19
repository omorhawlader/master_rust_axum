use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use std::sync::Arc;

use crate::state::AppState;

pub async fn config(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    state.request_count.fetch_add(1, Ordering::Relaxed);
    #[derive(Serialize)]
    struct Config {
        app_name: String,
        requests: u64,
    }
    (
        StatusCode::OK,
        Json(Config {
            app_name: state.name.clone(),
            requests: state.request_count.load(Ordering::Relaxed),
        }),
    )
}

pub async fn fallback() -> &'static str {
    "Not found"
}
