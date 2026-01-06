#![allow(missing_docs)]

use std::collections::HashMap;
use std::sync::Arc;

use gorkd_core::traits::SearchProvider;
use tracing::info;

use crate::config::SearchConfig;
use crate::exa::ExaProvider;
use crate::searxng::SearxngProvider;
use crate::tavily::TavilyProvider;

/// Order of providers for fallback (highest priority first).
pub const PROVIDER_ORDER: &[&str] = &["tavily", "exa", "searxng"];

#[derive(Clone, Default)]
pub struct ProviderRegistry {
    providers: HashMap<String, Arc<dyn SearchProvider>>,
    /// Provider IDs in priority order for fallback.
    order: Vec<String>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            order: Vec::new(),
        }
    }

    pub fn register(&mut self, id: impl Into<String>, provider: Arc<dyn SearchProvider>) {
        let id = id.into();
        if !self.providers.contains_key(&id) {
            self.order.push(id.clone());
        }
        self.providers.insert(id, provider);
    }

    pub fn get(&self, id: &str) -> Option<Arc<dyn SearchProvider>> {
        self.providers.get(id).cloned()
    }

    /// Returns the default (highest priority) provider.
    pub fn default_provider(&self) -> Option<Arc<dyn SearchProvider>> {
        self.order.first().and_then(|id| self.get(id))
    }

    /// Returns providers in priority order for fallback iteration.
    pub fn providers_in_order(&self) -> Vec<Arc<dyn SearchProvider>> {
        self.order.iter().filter_map(|id| self.get(id)).collect()
    }

    pub fn list(&self) -> Vec<String> {
        self.order.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Creates a registry from configuration, initializing all available providers.
    ///
    /// Providers are registered in priority order: Tavily, Exa, SearXNG.
    /// Only providers with valid credentials/URLs are registered.
    pub fn from_config(config: &SearchConfig) -> Self {
        let mut registry = Self::new();

        if let Some(ref api_key) = config.tavily_api_key {
            let provider = TavilyProvider::new(api_key);
            registry.register("tavily", Arc::new(provider));
            info!(provider = "tavily", "registered search provider");
        }

        if let Some(ref api_key) = config.exa_api_key {
            let provider = ExaProvider::new(api_key);
            registry.register("exa", Arc::new(provider));
            info!(provider = "exa", "registered search provider");
        }

        if let Some(ref url) = config.searxng_url {
            let provider = SearxngProvider::new(url);
            registry.register("searxng", Arc::new(provider));
            info!(provider = "searxng", url = %url, "registered search provider");
        }

        registry
    }
}

impl std::fmt::Debug for ProviderRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderRegistry")
            .field("providers", &self.list())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use gorkd_core::traits::{SearchError, SearchResult};
    use gorkd_core::SearchQuery;

    struct MockProvider {
        id: String,
    }

    impl MockProvider {
        fn new(id: impl Into<String>) -> Self {
            Self { id: id.into() }
        }
    }

    #[async_trait]
    impl SearchProvider for MockProvider {
        async fn search(&self, _query: &SearchQuery) -> Result<Vec<SearchResult>, SearchError> {
            Ok(vec![])
        }

        fn provider_id(&self) -> &str {
            &self.id
        }
    }

    #[test]
    fn creates_empty_registry() {
        let registry = ProviderRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn registers_and_retrieves_provider() {
        let mut registry = ProviderRegistry::new();
        let provider = Arc::new(MockProvider::new("test"));
        registry.register("test", provider);

        assert!(!registry.is_empty());
        assert_eq!(registry.len(), 1);
        assert!(registry.get("test").is_some());
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn lists_providers_in_registration_order() {
        let mut registry = ProviderRegistry::new();
        registry.register("zebra", Arc::new(MockProvider::new("zebra")));
        registry.register("alpha", Arc::new(MockProvider::new("alpha")));
        registry.register("middle", Arc::new(MockProvider::new("middle")));

        assert_eq!(registry.list(), vec!["zebra", "alpha", "middle"]);
    }

    #[test]
    fn overwrites_existing_provider_keeps_order() {
        let mut registry = ProviderRegistry::new();
        registry.register("test", Arc::new(MockProvider::new("old")));
        registry.register("test", Arc::new(MockProvider::new("new")));

        assert_eq!(registry.len(), 1);
        let provider = registry.get("test").unwrap();
        assert_eq!(provider.provider_id(), "new");
        assert_eq!(registry.list(), vec!["test"]);
    }

    #[test]
    fn debug_shows_provider_ids() {
        let mut registry = ProviderRegistry::new();
        registry.register("tavily", Arc::new(MockProvider::new("tavily")));
        registry.register("exa", Arc::new(MockProvider::new("exa")));

        let debug = format!("{:?}", registry);
        assert!(debug.contains("ProviderRegistry"));
        assert!(debug.contains("tavily"));
        assert!(debug.contains("exa"));
    }

    #[test]
    fn from_config_creates_empty_registry_without_credentials() {
        let config = SearchConfig::default();
        let registry = ProviderRegistry::from_config(&config);
        assert!(registry.is_empty());
    }

    #[test]
    fn default_provider_returns_first_registered() {
        let mut registry = ProviderRegistry::new();
        registry.register("first", Arc::new(MockProvider::new("first")));
        registry.register("second", Arc::new(MockProvider::new("second")));

        let default = registry.default_provider().unwrap();
        assert_eq!(default.provider_id(), "first");
    }

    #[test]
    fn default_provider_returns_none_when_empty() {
        let registry = ProviderRegistry::new();
        assert!(registry.default_provider().is_none());
    }

    #[test]
    fn providers_in_order_returns_all_in_registration_order() {
        let mut registry = ProviderRegistry::new();
        registry.register("a", Arc::new(MockProvider::new("a")));
        registry.register("b", Arc::new(MockProvider::new("b")));
        registry.register("c", Arc::new(MockProvider::new("c")));

        let providers = registry.providers_in_order();
        assert_eq!(providers.len(), 3);
        assert_eq!(providers[0].provider_id(), "a");
        assert_eq!(providers[1].provider_id(), "b");
        assert_eq!(providers[2].provider_id(), "c");
    }
}
