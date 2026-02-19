//! Service layer: business logic, uses repo. Handlers call service, not repo directly.

use crate::dto::{CreateItemDto, ItemDto};
use crate::repo::Repo;
use std::sync::Arc;

pub struct Service {
    repo: Arc<Repo>,
}

impl Service {
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }

    pub fn create_item(&self, dto: CreateItemDto) -> ItemDto {
        self.repo.create(dto.name)
    }

    pub fn get_item(&self, id: u32) -> Option<ItemDto> {
        self.repo.get(id)
    }
}
