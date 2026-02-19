//! DTOs: request/response shapes, separate from domain or DB models.

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateItemDto {
    pub name: String,
}

#[derive(Clone, Serialize)]
pub struct ItemDto {
    pub id: u32,
    pub name: String,
}
