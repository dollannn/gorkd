use async_trait::async_trait;

use crate::search::SearchQuery;
use crate::source::Source;
use crate::traits::errors::SearchError;

#[derive(Clone, Debug)]
pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub snippet: String,
    pub score: f32,
}

impl SearchResult {
    pub fn new(
        url: impl Into<String>,
        title: impl Into<String>,
        snippet: impl Into<String>,
    ) -> Self {
        Self {
            url: url.into(),
            title: title.into(),
            snippet: snippet.into(),
            score: 0.0,
        }
    }

    pub fn with_score(mut self, score: f32) -> Self {
        self.score = score.clamp(0.0, 1.0);
        self
    }

    pub fn into_source(self, content: String) -> Source {
        Source::new(self.url, self.title, content).with_relevance_score(self.score)
    }
}

#[async_trait]
pub trait SearchProvider: Send + Sync {
    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, SearchError>;

    fn provider_id(&self) -> &str;

    fn supports_recency_filter(&self) -> bool {
        false
    }

    fn supports_domain_filter(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_result_clamps_score() {
        let result = SearchResult::new("https://example.com", "Title", "Snippet").with_score(1.5);
        assert_eq!(result.score, 1.0);

        let result = SearchResult::new("https://example.com", "Title", "Snippet").with_score(-0.5);
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn search_result_converts_to_source() {
        let result = SearchResult::new("https://example.com", "Title", "Snippet").with_score(0.8);
        let source = result.into_source("Full content here".to_string());

        assert_eq!(source.url, "https://example.com");
        assert_eq!(source.title, "Title");
        assert_eq!(source.content, "Full content here");
        assert_eq!(source.relevance_score, 0.8);
    }
}
