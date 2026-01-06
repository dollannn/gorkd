//! Tavily search provider implementation.
//!
//! Tavily offers high-quality web search with relevance scoring, recency filters,
//! and domain filtering. API docs: <https://docs.tavily.com/documentation/api-reference/endpoint/search>

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument, warn};

use crate::client::HttpClient;
use gorkd_core::{ContentType, Recency, SearchQuery};
use gorkd_core::{SearchError, SearchProvider, SearchResult};

const TAVILY_API_URL: &str = "https://api.tavily.com/search";
const PROVIDER_ID: &str = "tavily";

/// Tavily search provider.
///
/// Implements the `SearchProvider` trait for Tavily's web search API.
/// Supports recency filtering, domain filtering, and content type filtering.
#[derive(Clone)]
pub struct TavilyProvider {
    api_key: String,
    client: HttpClient,
    search_depth: SearchDepth,
}

impl TavilyProvider {
    /// Creates a new Tavily provider with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: HttpClient::default(),
            search_depth: SearchDepth::Basic,
        }
    }

    /// Creates a new Tavily provider with a custom HTTP client.
    pub fn with_client(api_key: impl Into<String>, client: HttpClient) -> Self {
        Self {
            api_key: api_key.into(),
            client,
            search_depth: SearchDepth::Basic,
        }
    }

    /// Sets the search depth for queries.
    ///
    /// - `Basic`: Balanced option for relevance and latency (1 credit)
    /// - `Advanced`: Highest relevance with increased latency (2 credits)
    pub fn with_search_depth(mut self, depth: SearchDepth) -> Self {
        self.search_depth = depth;
        self
    }

    fn build_request(&self, query: &SearchQuery) -> TavilyRequest {
        let mut request = TavilyRequest {
            query: query.text.clone(),
            search_depth: self.search_depth,
            max_results: 10,
            topic: None,
            time_range: None,
            include_domains: None,
            exclude_domains: None,
            include_answer: false,
            include_raw_content: false,
        };

        // Map recency filter
        if let Some(ref recency) = query.filters.recency {
            request.time_range = Some(map_recency(recency));
        }

        // Map domain filters
        if let Some(ref domains) = query.filters.include_domains {
            if !domains.is_empty() {
                request.include_domains = Some(domains.clone());
            }
        }

        if let Some(ref domains) = query.filters.exclude_domains {
            if !domains.is_empty() {
                request.exclude_domains = Some(domains.clone());
            }
        }

        // Map content type to topic
        if let Some(ref content_type) = query.filters.content_type {
            request.topic = Some(map_content_type(content_type));
        }

        request
    }
}

#[async_trait]
impl SearchProvider for TavilyProvider {
    #[instrument(skip(self), fields(provider = PROVIDER_ID))]
    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, SearchError> {
        let request = self.build_request(query);

        debug!(
            query = %request.query,
            depth = ?request.search_depth,
            "executing tavily search"
        );

        let response = self
            .client
            .post(TAVILY_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| map_reqwest_error(e, self.client.timeout().as_secs()))?;

        let status = response.status();

        if !status.is_success() {
            return Err(map_http_error(status));
        }

        let tavily_response: TavilyResponse = response.json().await.map_err(|e| {
            warn!(error = %e, "failed to parse tavily response");
            SearchError::Provider(format!("failed to parse response: {}", e))
        })?;

        debug!(
            result_count = tavily_response.results.len(),
            response_time = %tavily_response.response_time,
            "tavily search completed"
        );

        let results = tavily_response
            .results
            .into_iter()
            .map(|r| {
                SearchResult::new(r.url, r.title, r.content).with_score(r.score.unwrap_or(0.0))
            })
            .collect();

        Ok(results)
    }

    fn provider_id(&self) -> &str {
        PROVIDER_ID
    }

    fn supports_recency_filter(&self) -> bool {
        true
    }

    fn supports_domain_filter(&self) -> bool {
        true
    }
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Search depth controls latency vs relevance tradeoff.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchDepth {
    /// Balanced option for relevance and latency (1 credit).
    #[default]
    Basic,
    /// Highest relevance with increased latency (2 credits).
    Advanced,
    /// Prioritizes lower latency (1 credit).
    Fast,
}

/// Topic category for the search.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Topic {
    General,
    News,
    Finance,
}

/// Time range for filtering results.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TimeRange {
    Day,
    Week,
    Month,
    Year,
}

/// Request body for Tavily search API.
#[derive(Debug, Serialize)]
struct TavilyRequest {
    query: String,
    search_depth: SearchDepth,
    max_results: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<Topic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_range: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_domains: Option<Vec<String>>,
    include_answer: bool,
    include_raw_content: bool,
}

/// Response from Tavily search API.
#[derive(Debug, Deserialize)]
struct TavilyResponse {
    #[allow(dead_code)]
    query: String,
    results: Vec<TavilyResult>,
    response_time: String,
}

/// Individual search result from Tavily.
#[derive(Debug, Deserialize)]
struct TavilyResult {
    title: String,
    url: String,
    content: String,
    score: Option<f32>,
}

// ============================================================================
// Mapping Functions
// ============================================================================

fn map_recency(recency: &Recency) -> TimeRange {
    match recency {
        Recency::Day => TimeRange::Day,
        Recency::Week => TimeRange::Week,
        Recency::Month => TimeRange::Month,
        Recency::Year | Recency::Any => TimeRange::Year,
        _ => TimeRange::Year,
    }
}

fn map_content_type(content_type: &ContentType) -> Topic {
    match content_type {
        ContentType::News => Topic::News,
        _ => Topic::General,
    }
}

fn map_reqwest_error(error: reqwest::Error, timeout_secs: u64) -> SearchError {
    if error.is_timeout() {
        SearchError::Timeout { timeout_secs }
    } else if error.is_connect() {
        SearchError::Network(format!("connection failed: {}", error))
    } else {
        SearchError::Network(error.to_string())
    }
}

fn map_http_error(status: reqwest::StatusCode) -> SearchError {
    match status.as_u16() {
        401 => SearchError::ProviderUnavailable {
            provider: PROVIDER_ID.to_string(),
        },
        429 => SearchError::RateLimited {
            provider: PROVIDER_ID.to_string(),
        },
        400 => SearchError::InvalidQuery {
            reason: "bad request".to_string(),
        },
        _ => SearchError::Provider(format!("HTTP {}", status)),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use gorkd_core::SearchFilters;

    #[test]
    fn creates_provider() {
        let provider = TavilyProvider::new("test-api-key");
        assert_eq!(provider.provider_id(), "tavily");
        assert!(provider.supports_recency_filter());
        assert!(provider.supports_domain_filter());
    }

    #[test]
    fn builds_basic_request() {
        let provider = TavilyProvider::new("test-key");
        let query = SearchQuery::new("test query");

        let request = provider.build_request(&query);

        assert_eq!(request.query, "test query");
        assert!(matches!(request.search_depth, SearchDepth::Basic));
        assert_eq!(request.max_results, 10);
        assert!(request.topic.is_none());
        assert!(request.time_range.is_none());
    }

    #[test]
    fn builds_request_with_recency_filter() {
        let provider = TavilyProvider::new("test-key");
        let query =
            SearchQuery::new("test").with_filters(SearchFilters::new().with_recency(Recency::Week));

        let request = provider.build_request(&query);

        assert!(matches!(request.time_range, Some(TimeRange::Week)));
    }

    #[test]
    fn builds_request_with_domain_filters() {
        let provider = TavilyProvider::new("test-key");
        let query = SearchQuery::new("test").with_filters(
            SearchFilters::new()
                .include_domains(["rust-lang.org", "crates.io"])
                .exclude_domains(["spam.com"]),
        );

        let request = provider.build_request(&query);

        assert_eq!(
            request.include_domains,
            Some(vec!["rust-lang.org".to_string(), "crates.io".to_string()])
        );
        assert_eq!(request.exclude_domains, Some(vec!["spam.com".to_string()]));
    }

    #[test]
    fn builds_request_with_news_topic() {
        let provider = TavilyProvider::new("test-key");
        let query = SearchQuery::new("test")
            .with_filters(SearchFilters::new().with_content_type(ContentType::News));

        let request = provider.build_request(&query);

        assert!(matches!(request.topic, Some(Topic::News)));
    }

    #[test]
    fn maps_all_recency_values() {
        assert!(matches!(map_recency(&Recency::Day), TimeRange::Day));
        assert!(matches!(map_recency(&Recency::Week), TimeRange::Week));
        assert!(matches!(map_recency(&Recency::Month), TimeRange::Month));
        assert!(matches!(map_recency(&Recency::Year), TimeRange::Year));
        assert!(matches!(map_recency(&Recency::Any), TimeRange::Year));
    }

    #[test]
    fn maps_content_types_to_topics() {
        assert!(matches!(map_content_type(&ContentType::News), Topic::News));
        assert!(matches!(
            map_content_type(&ContentType::General),
            Topic::General
        ));
        assert!(matches!(
            map_content_type(&ContentType::Academic),
            Topic::General
        ));
    }

    #[test]
    fn maps_http_errors() {
        assert!(matches!(
            map_http_error(reqwest::StatusCode::UNAUTHORIZED),
            SearchError::ProviderUnavailable { .. }
        ));
        assert!(matches!(
            map_http_error(reqwest::StatusCode::TOO_MANY_REQUESTS),
            SearchError::RateLimited { .. }
        ));
        assert!(matches!(
            map_http_error(reqwest::StatusCode::BAD_REQUEST),
            SearchError::InvalidQuery { .. }
        ));
        assert!(matches!(
            map_http_error(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
            SearchError::Provider(_)
        ));
    }

    #[test]
    fn serializes_request_correctly() {
        let request = TavilyRequest {
            query: "test".to_string(),
            search_depth: SearchDepth::Basic,
            max_results: 5,
            topic: Some(Topic::News),
            time_range: Some(TimeRange::Week),
            include_domains: Some(vec!["example.com".to_string()]),
            exclude_domains: None,
            include_answer: false,
            include_raw_content: false,
        };

        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"query\":\"test\""));
        assert!(json.contains("\"search_depth\":\"basic\""));
        assert!(json.contains("\"topic\":\"news\""));
        assert!(json.contains("\"time_range\":\"week\""));
        assert!(json.contains("\"include_domains\":[\"example.com\"]"));
        // exclude_domains should be skipped when None
        assert!(!json.contains("exclude_domains"));
    }

    #[test]
    fn deserializes_response() {
        let json = r#"{
            "query": "test query",
            "results": [
                {
                    "title": "Test Title",
                    "url": "https://example.com",
                    "content": "Test content snippet",
                    "score": 0.85
                }
            ],
            "response_time": "1.23"
        }"#;

        let response: TavilyResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.query, "test query");
        assert_eq!(response.results.len(), 1);
        assert_eq!(response.results[0].title, "Test Title");
        assert_eq!(response.results[0].url, "https://example.com");
        assert_eq!(response.results[0].content, "Test content snippet");
        assert_eq!(response.results[0].score, Some(0.85));
        assert_eq!(response.response_time, "1.23");
    }

    #[test]
    fn deserializes_response_without_score() {
        let json = r#"{
            "query": "test",
            "results": [
                {
                    "title": "Title",
                    "url": "https://example.com",
                    "content": "Content"
                }
            ],
            "response_time": "1.0"
        }"#;

        let response: TavilyResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.results[0].score, None);
    }

    #[test]
    fn configures_search_depth() {
        let provider = TavilyProvider::new("key").with_search_depth(SearchDepth::Advanced);
        let query = SearchQuery::new("test");

        let request = provider.build_request(&query);

        assert!(matches!(request.search_depth, SearchDepth::Advanced));
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn searches_with_real_api() {
        let api_key = std::env::var("TAVILY_API_KEY").expect("TAVILY_API_KEY must be set");
        let provider = TavilyProvider::new(api_key);
        let query = SearchQuery::new("What is Rust programming language?");

        let results = provider
            .search(&query)
            .await
            .expect("search should succeed");

        assert!(!results.is_empty(), "should return results");
        assert!(results[0].score > 0.0, "results should have scores");
        assert!(!results[0].url.is_empty(), "results should have URLs");
        assert!(!results[0].title.is_empty(), "results should have titles");
    }
}
