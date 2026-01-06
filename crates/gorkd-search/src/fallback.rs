//! Fallback search provider that tries multiple providers in order.

use std::sync::Arc;

use async_trait::async_trait;
use tracing::{debug, info, warn};

use gorkd_core::traits::{SearchError, SearchProvider, SearchResult};
use gorkd_core::SearchQuery;

use crate::ProviderRegistry;

/// A search provider that tries multiple providers in priority order.
pub struct FallbackSearchProvider {
    providers: Vec<Arc<dyn SearchProvider>>,
}

impl FallbackSearchProvider {
    /// Creates a new fallback provider from a list of providers.
    pub fn new(providers: Vec<Arc<dyn SearchProvider>>) -> Self {
        Self { providers }
    }

    /// Creates a fallback provider from a registry using its priority order.
    pub fn from_registry(registry: &ProviderRegistry) -> Self {
        Self::new(registry.providers_in_order())
    }
}

#[async_trait]
impl SearchProvider for FallbackSearchProvider {
    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, SearchError> {
        if self.providers.is_empty() {
            return Err(SearchError::ProviderUnavailable {
                provider: "none".to_string(),
            });
        }

        let mut last_error: Option<SearchError> = None;

        for provider in &self.providers {
            let provider_id = provider.provider_id();
            debug!(provider = %provider_id, query = %query.text, "attempting search");

            match provider.search(query).await {
                Ok(results) => {
                    info!(
                        provider = %provider_id,
                        results = results.len(),
                        "search succeeded"
                    );
                    return Ok(results);
                }
                Err(e) => {
                    warn!(
                        provider = %provider_id,
                        error = %e,
                        retryable = e.is_retryable(),
                        "search failed"
                    );

                    if e.is_retryable() {
                        last_error = Some(e);
                        continue;
                    }

                    return Err(e);
                }
            }
        }

        Err(
            last_error.unwrap_or_else(|| SearchError::ProviderUnavailable {
                provider: "all".to_string(),
            }),
        )
    }

    fn provider_id(&self) -> &str {
        "fallback"
    }

    fn supports_recency_filter(&self) -> bool {
        self.providers
            .first()
            .map(|p| p.supports_recency_filter())
            .unwrap_or(false)
    }

    fn supports_domain_filter(&self) -> bool {
        self.providers
            .first()
            .map(|p| p.supports_domain_filter())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct FailingProvider {
        id: String,
        error: SearchError,
        call_count: AtomicUsize,
    }

    impl FailingProvider {
        fn new(id: &str, error: SearchError) -> Self {
            Self {
                id: id.to_string(),
                error,
                call_count: AtomicUsize::new(0),
            }
        }

        fn calls(&self) -> usize {
            self.call_count.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl SearchProvider for FailingProvider {
        async fn search(&self, _query: &SearchQuery) -> Result<Vec<SearchResult>, SearchError> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            Err(self.error.clone())
        }

        fn provider_id(&self) -> &str {
            &self.id
        }
    }

    struct SuccessProvider {
        id: String,
        results: Vec<SearchResult>,
        call_count: AtomicUsize,
    }

    impl SuccessProvider {
        fn new(id: &str) -> Self {
            Self {
                id: id.to_string(),
                results: vec![SearchResult::new("https://example.com", "Test", "Content")],
                call_count: AtomicUsize::new(0),
            }
        }

        fn calls(&self) -> usize {
            self.call_count.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl SearchProvider for SuccessProvider {
        async fn search(&self, _query: &SearchQuery) -> Result<Vec<SearchResult>, SearchError> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            Ok(self.results.clone())
        }

        fn provider_id(&self) -> &str {
            &self.id
        }
    }

    #[tokio::test]
    async fn returns_error_when_no_providers() {
        let fallback = FallbackSearchProvider::new(vec![]);
        let query = SearchQuery::new("test");

        let result = fallback.search(&query).await;
        assert!(matches!(
            result,
            Err(SearchError::ProviderUnavailable { .. })
        ));
    }

    #[tokio::test]
    async fn uses_first_provider_on_success() {
        let first = Arc::new(SuccessProvider::new("first"));
        let second = Arc::new(SuccessProvider::new("second"));

        let fallback =
            FallbackSearchProvider::new(vec![Arc::clone(&first) as _, Arc::clone(&second) as _]);
        let query = SearchQuery::new("test");

        let result = fallback.search(&query).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(first.calls(), 1);
        assert_eq!(second.calls(), 0);
    }

    #[tokio::test]
    async fn falls_back_on_retryable_error() {
        let first = Arc::new(FailingProvider::new(
            "first",
            SearchError::RateLimited {
                provider: "first".to_string(),
            },
        ));
        let second = Arc::new(SuccessProvider::new("second"));

        let fallback =
            FallbackSearchProvider::new(vec![Arc::clone(&first) as _, Arc::clone(&second) as _]);
        let query = SearchQuery::new("test");

        let result = fallback.search(&query).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(first.calls(), 1);
        assert_eq!(second.calls(), 1);
    }

    #[tokio::test]
    async fn stops_on_non_retryable_error() {
        let first = Arc::new(FailingProvider::new(
            "first",
            SearchError::InvalidQuery {
                reason: "bad query".to_string(),
            },
        ));
        let second = Arc::new(SuccessProvider::new("second"));

        let fallback =
            FallbackSearchProvider::new(vec![Arc::clone(&first) as _, Arc::clone(&second) as _]);
        let query = SearchQuery::new("test");

        let result = fallback.search(&query).await;
        assert!(matches!(result, Err(SearchError::InvalidQuery { .. })));
        assert_eq!(first.calls(), 1);
        assert_eq!(second.calls(), 0);
    }

    #[tokio::test]
    async fn returns_last_error_when_all_fail() {
        let first = Arc::new(FailingProvider::new(
            "first",
            SearchError::Timeout { timeout_secs: 30 },
        ));
        let second = Arc::new(FailingProvider::new(
            "second",
            SearchError::RateLimited {
                provider: "second".to_string(),
            },
        ));

        let fallback =
            FallbackSearchProvider::new(vec![Arc::clone(&first) as _, Arc::clone(&second) as _]);
        let query = SearchQuery::new("test");

        let result = fallback.search(&query).await;
        assert!(matches!(result, Err(SearchError::RateLimited { .. })));
        assert_eq!(first.calls(), 1);
        assert_eq!(second.calls(), 1);
    }

    #[test]
    fn provider_id_is_fallback() {
        let fallback = FallbackSearchProvider::new(vec![]);
        assert_eq!(fallback.provider_id(), "fallback");
    }
}
