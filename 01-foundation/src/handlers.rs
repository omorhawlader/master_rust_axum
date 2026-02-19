//! Handler system: return types, IntoResponse, compile-time validation.

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;

pub async fn root() -> &'static str {
    "Hello from Foundation"
}

pub async fn get_user(Path(id): Path<u32>) -> impl IntoResponse {
    (StatusCode::OK, format!("user id: {}", id))
}

#[derive(Serialize)]
pub struct HealthBody {
    pub status: &'static str,
    pub version: &'static str,
}

pub async fn health() -> (StatusCode, Json<HealthBody>) {
    (
        StatusCode::OK,
        Json(HealthBody {
            status: "ok",
            version: "day1",
        }),
    )
}

pub struct PlainText(pub String);

impl IntoResponse for PlainText {
    fn into_response(self) -> Response {
        (StatusCode::OK, self.0).into_response()
    }
}

pub async fn custom_response() -> PlainText {
    PlainText("custom IntoResponse".to_string())
}

pub async fn status_only() -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn fallback() -> &'static str {
    "Not found (fallback)"
}
