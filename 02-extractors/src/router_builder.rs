use axum::{routing::get, routing::post, Router};

use crate::handlers;

pub fn app() -> Router {
    Router::new()
        .route("/items/{id}", get(handlers::get_item))
        .route("/search", get(handlers::search))
        .route("/items", post(handlers::create_item))
        .fallback(handlers::fallback)
}
