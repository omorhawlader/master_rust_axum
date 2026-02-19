//! Handlers: extract, call service, return response. No DB or repo access here.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::dto::CreateItemDto;
use crate::service::Service;

pub async fn create_item(
    State(svc): State<Arc<Service>>,
    Json(dto): Json<CreateItemDto>,
) -> impl IntoResponse {
    let item = svc.create_item(dto);
    (StatusCode::CREATED, Json(item))
}

pub async fn get_item(
    State(svc): State<Arc<Service>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    match svc.get_item(id) {
        Some(item) => (StatusCode::OK, Json(item)).into_response(),
        None => (StatusCode::NOT_FOUND, "not found").into_response(),
    }
}

pub async fn fallback() -> &'static str {
    "Not found"
}
