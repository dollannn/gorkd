#![allow(missing_docs)]

use std::collections::HashMap;
use std::sync::Arc;

use gorkd_core::traits::SearchProvider;

use crate::config::SearchConfig;

#[derive(Clone, Default)]
pub struct ProviderRegistry {
    providers: HashMap<String, Arc<dyn SearchProvider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, id: impl Into<String>, provider: Arc<dyn SearchProvider>) {
        self.providers.insert(id.into(), provider);
    }

    pub fn get(&self, id: &str) -> Option<Arc<dyn SearchProvider>> {
        self.providers.get(id).cloned()
    }

    pub fn list(&self) -> Vec<String> {
        let mut ids: Vec<_> = self.providers.keys().cloned().collect();
        ids.sort();
        ids
    }

    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    pub fn len(&self) -> usize {
        self.providers.len()
    }

    pub fn from_config(_config: &SearchConfig) -> Self {
        Self::new()
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
    fn lists_providers_sorted() {
        let mut registry = ProviderRegistry::new();
        registry.register("zebra", Arc::new(MockProvider::new("zebra")));
        registry.register("alpha", Arc::new(MockProvider::new("alpha")));
        registry.register("middle", Arc::new(MockProvider::new("middle")));

        assert_eq!(registry.list(), vec!["alpha", "middle", "zebra"]);
    }

    #[test]
    fn overwrites_existing_provider() {
        let mut registry = ProviderRegistry::new();
        registry.register("test", Arc::new(MockProvider::new("old")));
        registry.register("test", Arc::new(MockProvider::new("new")));

        assert_eq!(registry.len(), 1);
        let provider = registry.get("test").unwrap();
        assert_eq!(provider.provider_id(), "new");
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
    fn from_config_creates_registry() {
        let config = SearchConfig::default();
        let registry = ProviderRegistry::from_config(&config);
        assert!(registry.is_empty());
    }
}
