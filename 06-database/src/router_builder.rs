use axum::{routing::get, routing::post, Router};
use std::sync::Arc;

use crate::handlers;
use crate::repo::Repo;
use crate::service::Service;

pub fn app() -> Router {
    let repo = Arc::new(Repo::new());
    let service = Arc::new(Service::new(repo));
    Router::new()
        .route("/items", post(handlers::create_item))
        .route("/items/{id}", get(handlers::get_item))
        .fallback(handlers::fallback)
        .with_state(service)
}
