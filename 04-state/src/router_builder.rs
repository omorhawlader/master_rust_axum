use axum::{routing::get, Router};
use std::sync::Arc;

use crate::handlers;
use crate::state::AppState;

pub fn app() -> Router {
    let state = Arc::new(AppState::new("day4".into()));
    Router::new()
        .route("/config", get(handlers::config))
        .fallback(handlers::fallback)
        .with_state(state)
}
