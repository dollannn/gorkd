use std::sync::Arc;
use std::time::Instant;

use gorkd_core::{LlmProvider, Pipeline, SearchProvider, Store};

pub struct AppState {
    pub store: Arc<dyn Store>,
    pub search_provider: Arc<dyn SearchProvider>,
    pub llm_provider: Arc<dyn LlmProvider>,
    pub started_at: Instant,
}

impl AppState {
    pub fn new(
        store: Arc<dyn Store>,
        search_provider: Arc<dyn SearchProvider>,
        llm_provider: Arc<dyn LlmProvider>,
    ) -> Self {
        Self {
            store,
            search_provider,
            llm_provider,
            started_at: Instant::now(),
        }
    }

    pub fn uptime_seconds(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }

    pub fn pipeline(&self) -> Pipeline {
        Pipeline::new(
            Arc::clone(&self.store),
            Arc::clone(&self.search_provider),
            Arc::clone(&self.llm_provider),
        )
    }
}
