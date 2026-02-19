use axum::{routing::get, Router};

use crate::handlers;

pub fn app() -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .fallback(handlers::fallback)
}
