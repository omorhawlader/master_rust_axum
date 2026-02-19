//! Repository: data access. In production, holds pool and runs queries.
//! Here: in-memory store for structure only.

use crate::dto::ItemDto;
use std::sync::Mutex;

pub struct Repo {
    items: Mutex<Vec<ItemDto>>,
}

impl Repo {
    pub fn new() -> Self {
        Self {
            items: Mutex::new(vec![]),
        }
    }

    pub fn create(&self, name: String) -> ItemDto {
        let mut items = self.items.lock().unwrap();
        let id = items.len() as u32 + 1;
        let item = ItemDto { id, name };
        items.push(item.clone());
        item
    }

    pub fn get(&self, id: u32) -> Option<ItemDto> {
        self.items.lock().unwrap().iter().find(|i| i.id == id).cloned()
    }
}
