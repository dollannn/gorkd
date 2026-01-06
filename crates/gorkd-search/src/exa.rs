//! Exa search provider implementation.
//!
//! Exa offers semantic/neural search capabilities for understanding query intent
//! and finding conceptually relevant results. API docs: <https://docs.exa.ai/reference/search>

use async_trait::async_trait;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument, warn};

use crate::client::HttpClient;
use gorkd_core::{Recency, SearchQuery};
use gorkd_core::{SearchError, SearchProvider, SearchResult};

const EXA_API_URL: &str = "https://api.exa.ai/search";
const PROVIDER_ID: &str = "exa";

/// Exa search provider.
///
/// Implements the `SearchProvider` trait for Exa's semantic search API.
/// Supports date filtering, domain filtering, and neural/keyword search modes.
#[derive(Clone)]
pub struct ExaProvider {
    api_key: String,
    client: HttpClient,
    search_type: SearchType,
}

impl ExaProvider {
    /// Creates a new Exa provider with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: HttpClient::default(),
            search_type: SearchType::Auto,
        }
    }

    /// Creates a new Exa provider with a custom HTTP client.
    pub fn with_client(api_key: impl Into<String>, client: HttpClient) -> Self {
        Self {
            api_key: api_key.into(),
            client,
            search_type: SearchType::Auto,
        }
    }

    /// Sets the search type for queries.
    ///
    /// - `Auto` (default): Intelligently combines neural and other methods
    /// - `Neural`: Uses embeddings-based model for semantic search
    /// - `Deep`: Comprehensive search with query expansion
    pub fn with_search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = search_type;
        self
    }

    fn build_request(&self, query: &SearchQuery) -> ExaRequest {
        let mut request = ExaRequest {
            query: query.text.clone(),
            search_type: self.search_type,
            num_results: 10,
            include_domains: None,
            exclude_domains: None,
            start_published_date: None,
            end_published_date: None,
            text: true,
        };

        // Map recency filter to date range
        if let Some(ref recency) = query.filters.recency {
            request.start_published_date = Some(recency_to_start_date(recency));
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

        request
    }
}

#[async_trait]
impl SearchProvider for ExaProvider {
    #[instrument(skip(self), fields(provider = PROVIDER_ID))]
    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, SearchError> {
        let request = self.build_request(query);

        debug!(
            query = %request.query,
            search_type = ?request.search_type,
            "executing exa search"
        );

        let response = self
            .client
            .post(EXA_API_URL)
            .header("x-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| map_reqwest_error(e, self.client.timeout().as_secs()))?;

        let status = response.status();

        if !status.is_success() {
            return Err(map_http_error(status));
        }

        let exa_response: ExaResponse = response.json().await.map_err(|e| {
            warn!(error = %e, "failed to parse exa response");
            SearchError::Provider(format!("failed to parse response: {}", e))
        })?;

        debug!(
            result_count = exa_response.results.len(),
            request_id = %exa_response.request_id.as_deref().unwrap_or("unknown"),
            "exa search completed"
        );

        let results = exa_response
            .results
            .into_iter()
            .map(|r| {
                let snippet = r.text.unwrap_or_default();
                let normalized_score = normalize_score(r.score);
                SearchResult::new(r.url, r.title, snippet).with_score(normalized_score)
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

/// Search type controls the search algorithm used.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchType {
    /// Intelligently combines neural and other search methods.
    #[default]
    Auto,
    /// Uses embeddings-based model for semantic understanding.
    Neural,
    /// Streamlined version for faster results.
    Fast,
    /// Comprehensive search with query expansion.
    Deep,
}

/// Request body for Exa search API.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ExaRequest {
    query: String,
    #[serde(rename = "type")]
    search_type: SearchType,
    num_results: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_published_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_published_date: Option<String>,
    /// Request text content in results.
    text: bool,
}

/// Response from Exa search API.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExaResponse {
    request_id: Option<String>,
    results: Vec<ExaResult>,
}

/// Individual search result from Exa.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExaResult {
    title: String,
    url: String,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    score: Option<f32>,
    #[serde(default)]
    #[allow(dead_code)]
    published_date: Option<String>,
}

// ============================================================================
// Mapping Functions
// ============================================================================

/// Converts a Recency enum to an ISO 8601 start date string.
fn recency_to_start_date(recency: &Recency) -> String {
    let now = Utc::now();
    let start = match recency {
        Recency::Day => now - Duration::days(1),
        Recency::Week => now - Duration::days(7),
        Recency::Month => now - Duration::days(30),
        Recency::Year => now - Duration::days(365),
        Recency::Any => return now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        _ => now - Duration::days(365), // Default to year for unknown variants
    };
    start.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

/// Normalizes Exa scores to a 0.0-1.0 range.
///
/// Exa scores can vary widely. Based on observed behavior:
/// - Scores typically range from 0.1 to 0.4 for good matches
/// - Higher scores indicate better semantic relevance
///
/// We normalize using a scaling factor.
fn normalize_score(score: Option<f32>) -> f32 {
    match score {
        Some(s) if s > 0.0 => {
            // Exa scores are often in 0.1-0.5 range for good results
            // Map to 0.0-1.0 using a scaling factor
            (s * 2.5).clamp(0.0, 1.0)
        }
        _ => 0.0,
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
        let provider = ExaProvider::new("test-api-key");
        assert_eq!(provider.provider_id(), "exa");
        assert!(provider.supports_recency_filter());
        assert!(provider.supports_domain_filter());
    }

    #[test]
    fn builds_basic_request() {
        let provider = ExaProvider::new("test-key");
        let query = SearchQuery::new("test query");

        let request = provider.build_request(&query);

        assert_eq!(request.query, "test query");
        assert!(matches!(request.search_type, SearchType::Auto));
        assert_eq!(request.num_results, 10);
        assert!(request.include_domains.is_none());
        assert!(request.exclude_domains.is_none());
        assert!(request.start_published_date.is_none());
        assert!(request.text);
    }

    #[test]
    fn builds_request_with_recency_filter() {
        let provider = ExaProvider::new("test-key");
        let query =
            SearchQuery::new("test").with_filters(SearchFilters::new().with_recency(Recency::Week));

        let request = provider.build_request(&query);

        assert!(request.start_published_date.is_some());
        // Verify it's a valid ISO 8601 date
        let date_str = request.start_published_date.unwrap();
        assert!(date_str.contains("T"));
        assert!(date_str.ends_with("Z"));
    }

    #[test]
    fn builds_request_with_domain_filters() {
        let provider = ExaProvider::new("test-key");
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
    fn configures_search_type() {
        let provider = ExaProvider::new("key").with_search_type(SearchType::Neural);
        let query = SearchQuery::new("test");

        let request = provider.build_request(&query);

        assert!(matches!(request.search_type, SearchType::Neural));
    }

    #[test]
    fn normalizes_scores() {
        // Score of 0.4 should map to 1.0 (clamped)
        assert_eq!(normalize_score(Some(0.4)), 1.0);

        // Score of 0.2 should map to 0.5
        assert_eq!(normalize_score(Some(0.2)), 0.5);

        // Score of 0.0 should map to 0.0
        assert_eq!(normalize_score(Some(0.0)), 0.0);

        // None should map to 0.0
        assert_eq!(normalize_score(None), 0.0);

        // Negative scores should clamp to 0.0
        assert_eq!(normalize_score(Some(-0.5)), 0.0);
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
        let request = ExaRequest {
            query: "test".to_string(),
            search_type: SearchType::Neural,
            num_results: 5,
            include_domains: Some(vec!["example.com".to_string()]),
            exclude_domains: None,
            start_published_date: Some("2024-01-01T00:00:00.000Z".to_string()),
            end_published_date: None,
            text: true,
        };

        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"query\":\"test\""));
        assert!(json.contains("\"type\":\"neural\""));
        assert!(json.contains("\"numResults\":5"));
        assert!(json.contains("\"includeDomains\":[\"example.com\"]"));
        assert!(json.contains("\"startPublishedDate\":\"2024-01-01T00:00:00.000Z\""));
        assert!(json.contains("\"text\":true"));
        // excludeDomains and endPublishedDate should be skipped when None
        assert!(!json.contains("excludeDomains"));
        assert!(!json.contains("endPublishedDate"));
    }

    #[test]
    fn deserializes_response() {
        let json = r#"{
            "requestId": "abc123",
            "results": [
                {
                    "title": "Test Title",
                    "url": "https://example.com",
                    "text": "Test content snippet",
                    "score": 0.35,
                    "publishedDate": "2024-06-15T00:00:00Z"
                }
            ]
        }"#;

        let response: ExaResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.request_id, Some("abc123".to_string()));
        assert_eq!(response.results.len(), 1);
        assert_eq!(response.results[0].title, "Test Title");
        assert_eq!(response.results[0].url, "https://example.com");
        assert_eq!(
            response.results[0].text,
            Some("Test content snippet".to_string())
        );
        assert_eq!(response.results[0].score, Some(0.35));
        assert_eq!(
            response.results[0].published_date,
            Some("2024-06-15T00:00:00Z".to_string())
        );
    }

    #[test]
    fn deserializes_minimal_response() {
        let json = r#"{
            "results": [
                {
                    "title": "Title",
                    "url": "https://example.com"
                }
            ]
        }"#;

        let response: ExaResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.request_id, None);
        assert_eq!(response.results[0].text, None);
        assert_eq!(response.results[0].score, None);
        assert_eq!(response.results[0].published_date, None);
    }

    #[test]
    fn recency_produces_valid_dates() {
        let recencies = [
            Recency::Day,
            Recency::Week,
            Recency::Month,
            Recency::Year,
            Recency::Any,
        ];

        for recency in recencies {
            let date_str = recency_to_start_date(&recency);
            // Should be valid ISO 8601
            assert!(date_str.contains("T"));
            assert!(date_str.ends_with("Z"));
            // Should be parseable
            assert!(chrono::DateTime::parse_from_rfc3339(&date_str).is_ok());
        }
    }

    #[test]
    fn serializes_all_search_types() {
        assert_eq!(
            serde_json::to_string(&SearchType::Auto).unwrap(),
            "\"auto\""
        );
        assert_eq!(
            serde_json::to_string(&SearchType::Neural).unwrap(),
            "\"neural\""
        );
        assert_eq!(
            serde_json::to_string(&SearchType::Fast).unwrap(),
            "\"fast\""
        );
        assert_eq!(
            serde_json::to_string(&SearchType::Deep).unwrap(),
            "\"deep\""
        );
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn searches_with_real_api() {
        let api_key = std::env::var("EXA_API_KEY").expect("EXA_API_KEY must be set");
        let provider = ExaProvider::new(api_key);
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

    #[tokio::test]
    async fn searches_with_filters() {
        let api_key = std::env::var("EXA_API_KEY").expect("EXA_API_KEY must be set");
        let provider = ExaProvider::new(api_key);
        let query = SearchQuery::new("Rust async programming").with_filters(
            SearchFilters::new()
                .with_recency(Recency::Month)
                .include_domains(["rust-lang.org", "docs.rs"]),
        );

        let results = provider
            .search(&query)
            .await
            .expect("search should succeed");

        // Results should be from included domains
        for result in &results {
            let url_lower = result.url.to_lowercase();
            assert!(
                url_lower.contains("rust-lang.org") || url_lower.contains("docs.rs"),
                "result URL should be from included domains: {}",
                result.url
            );
        }
    }
}
