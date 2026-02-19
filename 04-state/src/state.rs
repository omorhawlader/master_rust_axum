//! Shared app state: Clone + Send + Sync. Wrapped in Arc for cheap cloning.

use std::sync::atomic::{AtomicU64, Ordering};

pub struct AppState {
    pub name: String,
    pub request_count: AtomicU64,
}

impl AppState {
    pub fn new(name: String) -> Self {
        Self {
            name,
            request_count: AtomicU64::new(0),
        }
    }
}
