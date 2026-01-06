//! Search execution for research pipeline.

use std::collections::HashSet;
use std::sync::Arc;

use crate::search::SearchPlan;
use crate::source::Source;
use crate::traits::{SearchError, SearchProvider};

#[derive(Clone, Debug)]
pub struct ExecutorConfig {
    pub max_sources: usize,
    pub min_score: f32,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            max_sources: 10,
            min_score: 0.0,
        }
    }
}

pub struct Executor {
    provider: Arc<dyn SearchProvider>,
    config: ExecutorConfig,
}

impl Executor {
    pub fn new(provider: Arc<dyn SearchProvider>, config: ExecutorConfig) -> Self {
        Self { provider, config }
    }

    pub async fn execute(&self, plan: &SearchPlan) -> Result<Vec<Source>, SearchError> {
        let mut all_sources = Vec::new();
        let mut seen_urls = HashSet::new();

        for query in &plan.queries {
            let results = self.provider.search(query).await?;

            for result in results {
                if seen_urls.contains(&result.url) {
                    continue;
                }
                seen_urls.insert(result.url.clone());

                if result.score < self.config.min_score {
                    continue;
                }

                let source = result.into_source(format!(
                    "Content fetched from source. Query: {}",
                    query.text
                ));

                all_sources.push(source);
            }
        }

        all_sources.sort_by(|a, b| {
            b.relevance_score
                .partial_cmp(&a.relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        all_sources.truncate(self.config.max_sources);

        Ok(all_sources)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockSearchProvider;
    use crate::search::SearchQuery;
    use crate::traits::SearchResult;

    #[tokio::test]
    async fn executor_returns_sources() {
        let provider = Arc::new(MockSearchProvider::new("mock"));
        let executor = Executor::new(provider, ExecutorConfig::default());

        let plan = SearchPlan::new(
            vec![SearchQuery::new("test")],
            vec![crate::search::ProviderId::new("mock")],
        );

        let sources = executor.execute(&plan).await.unwrap();
        assert!(!sources.is_empty());
    }

    #[tokio::test]
    async fn executor_deduplicates_by_url() {
        let results = vec![
            SearchResult::new("https://example.com/1", "Title 1", "Snippet 1").with_score(0.9),
            SearchResult::new("https://example.com/1", "Title 1 Dup", "Snippet 1 Dup")
                .with_score(0.8),
            SearchResult::new("https://example.com/2", "Title 2", "Snippet 2").with_score(0.7),
        ];

        let provider = Arc::new(MockSearchProvider::new("mock").with_results(results));
        let executor = Executor::new(provider, ExecutorConfig::default());

        let plan = SearchPlan::new(
            vec![SearchQuery::new("test")],
            vec![crate::search::ProviderId::new("mock")],
        );

        let sources = executor.execute(&plan).await.unwrap();
        assert_eq!(sources.len(), 2);
    }

    #[tokio::test]
    async fn executor_limits_results() {
        let provider = Arc::new(MockSearchProvider::new("mock"));
        let config = ExecutorConfig {
            max_sources: 2,
            min_score: 0.0,
        };
        let executor = Executor::new(provider, config);

        let plan = SearchPlan::new(
            vec![SearchQuery::new("test")],
            vec![crate::search::ProviderId::new("mock")],
        );

        let sources = executor.execute(&plan).await.unwrap();
        assert!(sources.len() <= 2);
    }

    #[tokio::test]
    async fn executor_filters_by_min_score() {
        let results = vec![
            SearchResult::new("https://example.com/1", "Title 1", "Snippet 1").with_score(0.9),
            SearchResult::new("https://example.com/2", "Title 2", "Snippet 2").with_score(0.3),
        ];

        let provider = Arc::new(MockSearchProvider::new("mock").with_results(results));
        let config = ExecutorConfig {
            max_sources: 10,
            min_score: 0.5,
        };
        let executor = Executor::new(provider, config);

        let plan = SearchPlan::new(
            vec![SearchQuery::new("test")],
            vec![crate::search::ProviderId::new("mock")],
        );

        let sources = executor.execute(&plan).await.unwrap();
        assert_eq!(sources.len(), 1);
        assert!(sources[0].relevance_score >= 0.5);
    }

    #[tokio::test]
    async fn executor_sorts_by_relevance() {
        let results = vec![
            SearchResult::new("https://example.com/1", "Title 1", "Snippet 1").with_score(0.5),
            SearchResult::new("https://example.com/2", "Title 2", "Snippet 2").with_score(0.9),
            SearchResult::new("https://example.com/3", "Title 3", "Snippet 3").with_score(0.7),
        ];

        let provider = Arc::new(MockSearchProvider::new("mock").with_results(results));
        let executor = Executor::new(provider, ExecutorConfig::default());

        let plan = SearchPlan::new(
            vec![SearchQuery::new("test")],
            vec![crate::search::ProviderId::new("mock")],
        );

        let sources = executor.execute(&plan).await.unwrap();
        assert_eq!(sources[0].relevance_score, 0.9);
        assert_eq!(sources[1].relevance_score, 0.7);
        assert_eq!(sources[2].relevance_score, 0.5);
    }
}
