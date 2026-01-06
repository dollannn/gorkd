use std::sync::Arc;
use std::time::Instant;

use gorkd_core::Store;

pub struct AppState {
    pub store: Arc<dyn Store>,
    pub started_at: Instant,
}

impl AppState {
    pub fn new(store: Arc<dyn Store>) -> Self {
        Self {
            store,
            started_at: Instant::now(),
        }
    }

    pub fn uptime_seconds(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }
}
