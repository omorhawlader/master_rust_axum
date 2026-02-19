//! Router: route, nest, merge, fallback, MethodRouter.

use axum::{routing::get, Router};

use crate::handlers;

pub fn app() -> Router {
    let api_routes = Router::new()
        .route("/health", get(handlers::health))
        .route("/users/{id}", get(handlers::get_user))
        .route("/custom", get(handlers::custom_response))
        .route("/status", get(handlers::status_only));

    Router::new()
        .route("/", get(handlers::root))
        .nest("/api", api_routes)
        .fallback(handlers::fallback)
}
