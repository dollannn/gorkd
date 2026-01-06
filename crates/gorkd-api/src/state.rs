use std::sync::Arc;
use std::time::Instant;

use gorkd_core::{LlmProvider, Pipeline, SearchProvider, Store};
use gorkd_llm::LlmRegistry;
use gorkd_search::{FallbackSearchProvider, ProviderRegistry};

pub struct AppState {
    pub store: Arc<dyn Store>,
    pub search_provider: Arc<dyn SearchProvider>,
    pub llm_registry: LlmRegistry,
    pub search_registry: ProviderRegistry,
    pub started_at: Instant,
}

impl AppState {
    pub fn new(
        store: Arc<dyn Store>,
        search_provider: Arc<dyn SearchProvider>,
        llm_provider: Arc<dyn LlmProvider>,
    ) -> Self {
        let mut llm_registry = LlmRegistry::new();
        let model_id = llm_provider.model_id().to_string();
        llm_registry.register(&model_id, llm_provider);
        llm_registry.set_default(&model_id);

        Self {
            store,
            search_provider,
            llm_registry,
            search_registry: ProviderRegistry::new(),
            started_at: Instant::now(),
        }
    }

    pub fn with_registries(
        store: Arc<dyn Store>,
        search_registry: ProviderRegistry,
        llm_registry: LlmRegistry,
    ) -> Self {
        let fallback = FallbackSearchProvider::from_registry(&search_registry);

        Self {
            store,
            search_provider: Arc::new(fallback),
            llm_registry,
            search_registry,
            started_at: Instant::now(),
        }
    }

    pub fn uptime_seconds(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }

    pub fn available_search_providers(&self) -> Vec<String> {
        self.search_registry.list()
    }

    pub fn available_llm_models(&self) -> Vec<String> {
        self.llm_registry.available_models()
    }

    pub fn default_llm_provider(&self) -> Option<Arc<dyn LlmProvider>> {
        self.llm_registry.default()
    }

    pub fn pipeline(&self) -> Pipeline {
        let llm_provider = self
            .llm_registry
            .default()
            .expect("LlmRegistry must have a default provider");

        Pipeline::new(
            Arc::clone(&self.store),
            Arc::clone(&self.search_provider),
            llm_provider,
        )
    }
}
