use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::error::AppError;

// Result in handler: E must implement IntoResponse (here AppError does)
pub async fn get_resource(Path(id): Path<u32>) -> Result<impl IntoResponse, AppError> {
    if id == 0 {
        return Err(AppError::Validation("id must be > 0".into()));
    }
    #[derive(Serialize)]
    struct Resource {
        id: u32,
        name: String,
    }
    Ok((StatusCode::OK, Json(Resource { id, name: "r".into() })))
}

pub async fn fallback() -> &'static str {
    "Not found"
}
