use std::sync::atomic::{AtomicUsize, Ordering};

use async_trait::async_trait;

use crate::search::SearchQuery;
use crate::traits::{SearchError, SearchProvider, SearchResult};

pub struct MockSearchProvider {
    provider_id: String,
    results: Vec<SearchResult>,
    call_count: AtomicUsize,
    fail_after: Option<usize>,
}

impl MockSearchProvider {
    pub fn new(provider_id: impl Into<String>) -> Self {
        Self {
            provider_id: provider_id.into(),
            results: Self::default_results(),
            call_count: AtomicUsize::new(0),
            fail_after: None,
        }
    }

    pub fn with_results(mut self, results: Vec<SearchResult>) -> Self {
        self.results = results;
        self
    }

    pub fn fail_after(mut self, n: usize) -> Self {
        self.fail_after = Some(n);
        self
    }

    pub fn call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }

    fn default_results() -> Vec<SearchResult> {
        vec![
            SearchResult::new(
                "https://example.com/article-1",
                "Example Article 1",
                "This is the first example article about the topic.",
            )
            .with_score(0.95),
            SearchResult::new(
                "https://example.com/article-2",
                "Example Article 2",
                "Another relevant article discussing the subject.",
            )
            .with_score(0.85),
            SearchResult::new(
                "https://example.org/resource",
                "Research Resource",
                "A comprehensive resource with detailed information.",
            )
            .with_score(0.75),
        ]
    }
}

#[async_trait]
impl SearchProvider for MockSearchProvider {
    async fn search(&self, _query: &SearchQuery) -> Result<Vec<SearchResult>, SearchError> {
        let count = self.call_count.fetch_add(1, Ordering::SeqCst);

        if let Some(fail_after) = self.fail_after {
            if count >= fail_after {
                return Err(SearchError::RateLimited {
                    provider: self.provider_id.clone(),
                });
            }
        }

        Ok(self.results.clone())
    }

    fn provider_id(&self) -> &str {
        &self.provider_id
    }

    fn supports_recency_filter(&self) -> bool {
        true
    }

    fn supports_domain_filter(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_search_returns_results() {
        let provider = MockSearchProvider::new("mock");
        let query = SearchQuery::new("test query");

        let results = provider.search(&query).await.unwrap();

        assert_eq!(results.len(), 3);
        assert!(results[0].score > results[1].score);
    }

    #[tokio::test]
    async fn mock_search_tracks_call_count() {
        let provider = MockSearchProvider::new("mock");
        let query = SearchQuery::new("test");

        assert_eq!(provider.call_count(), 0);

        provider.search(&query).await.unwrap();
        assert_eq!(provider.call_count(), 1);

        provider.search(&query).await.unwrap();
        assert_eq!(provider.call_count(), 2);
    }

    #[tokio::test]
    async fn mock_search_fails_after_n_calls() {
        let provider = MockSearchProvider::new("mock").fail_after(2);
        let query = SearchQuery::new("test");

        assert!(provider.search(&query).await.is_ok());
        assert!(provider.search(&query).await.is_ok());
        assert!(provider.search(&query).await.is_err());
    }

    #[tokio::test]
    async fn mock_search_with_custom_results() {
        let custom_results = vec![SearchResult::new(
            "https://custom.com",
            "Custom Result",
            "Custom snippet",
        )];

        let provider = MockSearchProvider::new("mock").with_results(custom_results);
        let query = SearchQuery::new("test");

        let results = provider.search(&query).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].url, "https://custom.com");
    }
}
