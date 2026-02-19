use axum::{routing::get, Router};

use crate::handlers;

pub fn app() -> Router {
    Router::new()
        .route("/resources/{id}", get(handlers::get_resource))
        .fallback(handlers::fallback)
}
