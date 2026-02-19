use axum::{routing::get, Router};
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::handlers;

pub fn app() -> Router {
    Router::new()
        .route("/", get(handlers::hello))
        .route("/slow", get(handlers::slow))
        .fallback(handlers::fallback)
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(Duration::from_secs(1)))
}
