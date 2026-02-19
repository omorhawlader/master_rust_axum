//! Day 2 — Path, Query, Json extractors. Body extractor last; rejection on failure.

use axum::{
    extract::{Path, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

// Path<T> — dynamic segment(s), deserialized
pub async fn get_item(Path(id): Path<u32>) -> impl IntoResponse {
    (StatusCode::OK, format!("item id: {}", id))
}

// Query<T> — query string → struct or HashMap
#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub limit: Option<u32>,
}

pub async fn search(Query(params): Query<SearchQuery>) -> impl IntoResponse {
    let msg = format!("q={:?} limit={:?}", params.q, params.limit);
    (StatusCode::OK, msg)
}

// Json<T> — body, Content-Type application/json, consumes body
#[derive(Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub count: Option<u32>,
}

#[derive(Serialize)]
pub struct ItemCreated {
    pub id: u32,
    pub name: String,
}

pub async fn create_item(Json(body): Json<CreateItem>) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(ItemCreated {
            id: 1,
            name: body.name,
        }),
    )
}

pub async fn fallback() -> &'static str {
    "Not found"
}
