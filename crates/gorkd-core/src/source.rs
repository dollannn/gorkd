use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::id::SourceId;
use crate::search::ProviderId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceMetadata {
    pub domain: String,
    pub published_at: Option<DateTime<Utc>>,
    pub author: Option<String>,
    pub word_count: usize,
}

impl SourceMetadata {
    pub fn new(domain: impl Into<String>) -> Self {
        Self {
            domain: domain.into(),
            published_at: None,
            author: None,
            word_count: 0,
        }
    }

    pub fn with_published_at(mut self, published_at: DateTime<Utc>) -> Self {
        self.published_at = Some(published_at);
        self
    }

    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn with_word_count(mut self, count: usize) -> Self {
        self.word_count = count;
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    pub id: SourceId,
    pub url: String,
    pub title: String,
    pub content: String,
    pub metadata: SourceMetadata,
    pub relevance_score: f32,
}

impl Source {
    pub fn new(
        url: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        let url = url.into();
        let domain = extract_domain(&url).unwrap_or_default();

        Self {
            id: SourceId::new(),
            url,
            title: title.into(),
            content: content.into(),
            metadata: SourceMetadata::new(domain),
            relevance_score: 0.0,
        }
    }

    pub fn with_metadata(mut self, metadata: SourceMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn with_relevance_score(mut self, score: f32) -> Self {
        self.relevance_score = score.clamp(0.0, 1.0);
        self
    }
}

fn extract_domain(url: &str) -> Option<String> {
    let without_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    without_scheme.split('/').next().map(|s| s.to_string())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchMetadata {
    pub queries_executed: Vec<String>,
    pub providers_used: Vec<ProviderId>,
    pub total_results: usize,
    #[serde(with = "duration_millis")]
    pub fetch_duration: Duration,
}

impl SearchMetadata {
    pub fn new() -> Self {
        Self {
            queries_executed: Vec::new(),
            providers_used: Vec::new(),
            total_results: 0,
            fetch_duration: Duration::ZERO,
        }
    }
}

impl Default for SearchMetadata {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceCollection {
    pub sources: Vec<Source>,
    pub search_metadata: SearchMetadata,
}

impl SourceCollection {
    pub fn new(sources: Vec<Source>) -> Self {
        Self {
            sources,
            search_metadata: SearchMetadata::new(),
        }
    }

    pub fn with_metadata(mut self, metadata: SearchMetadata) -> Self {
        self.search_metadata = metadata;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    pub fn len(&self) -> usize {
        self.sources.len()
    }
}

mod duration_millis {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_millis().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_source() {
        let source = Source::new(
            "https://rust-lang.org/about",
            "About Rust",
            "Rust is a systems programming language.",
        );
        assert!(source.id.as_str().starts_with("src_"));
        assert_eq!(source.metadata.domain, "rust-lang.org");
    }

    #[test]
    fn clamps_relevance_score() {
        let source =
            Source::new("https://example.com", "Test", "Content").with_relevance_score(1.5);
        assert_eq!(source.relevance_score, 1.0);

        let source =
            Source::new("https://example.com", "Test", "Content").with_relevance_score(-0.5);
        assert_eq!(source.relevance_score, 0.0);
    }

    #[test]
    fn extracts_domain_from_url() {
        assert_eq!(
            extract_domain("https://rust-lang.org/about"),
            Some("rust-lang.org".to_string())
        );
        assert_eq!(
            extract_domain("http://example.com"),
            Some("example.com".to_string())
        );
    }

    #[test]
    fn source_collection_tracks_count() {
        let sources = vec![
            Source::new("https://a.com", "A", "Content A"),
            Source::new("https://b.com", "B", "Content B"),
        ];
        let collection = SourceCollection::new(sources);
        assert_eq!(collection.len(), 2);
        assert!(!collection.is_empty());
    }

    #[test]
    fn serializes_source() {
        let source = Source::new("https://example.com", "Test", "Content");
        let json = serde_json::to_string(&source).unwrap();
        assert!(json.contains("example.com"));
        assert!(json.contains("src_"));
    }
}
