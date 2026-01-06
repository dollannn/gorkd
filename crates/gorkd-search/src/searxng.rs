//! SearXNG search provider implementation.
//!
//! SearXNG is a privacy-respecting metasearch engine that aggregates results from
//! multiple sources. No API key required, but needs a running instance with JSON
//! format enabled. API docs: <https://docs.searxng.org/dev/search_api.html>

use async_trait::async_trait;
use serde::Deserialize;
use tracing::{debug, instrument, warn};
use url::Url;

use crate::client::HttpClient;
use gorkd_core::{ContentType, Recency, SearchQuery};
use gorkd_core::{SearchError, SearchProvider, SearchResult};

const PROVIDER_ID: &str = "searxng";
const DEFAULT_INSTANCE_URL: &str = "https://searx.be";

/// SearXNG search provider.
///
/// Implements the `SearchProvider` trait for SearXNG's JSON search API.
/// Supports recency filtering via `time_range`, content type filtering via
/// `categories`, and domain filtering via query syntax (`site:domain.com`).
#[derive(Clone)]
pub struct SearxngProvider {
    instance_url: String,
    client: HttpClient,
}

impl SearxngProvider {
    /// Creates a new SearXNG provider with the given instance URL.
    ///
    /// # Example
    ///
    /// ```
    /// use gorkd_search::SearxngProvider;
    ///
    /// let provider = SearxngProvider::new("https://searx.example.org");
    /// ```
    pub fn new(instance_url: impl Into<String>) -> Self {
        let mut url = instance_url.into();
        if url.ends_with('/') {
            url.pop();
        }
        Self {
            instance_url: url,
            client: HttpClient::default(),
        }
    }

    /// Creates a new SearXNG provider from the `SEARXNG_URL` environment variable.
    ///
    /// Falls back to a default public instance if the variable is not set.
    pub fn from_env() -> Self {
        let url = std::env::var("SEARXNG_URL").unwrap_or_else(|_| DEFAULT_INSTANCE_URL.to_string());
        Self::new(url)
    }

    /// Creates a new SearXNG provider with a custom HTTP client.
    pub fn with_client(instance_url: impl Into<String>, client: HttpClient) -> Self {
        let mut url = instance_url.into();
        if url.ends_with('/') {
            url.pop();
        }
        Self {
            instance_url: url,
            client,
        }
    }

    /// Returns the instance URL.
    pub fn instance_url(&self) -> &str {
        &self.instance_url
    }

    fn build_url(&self, query: &SearchQuery) -> Result<Url, SearchError> {
        let mut url = Url::parse(&format!("{}/search", self.instance_url))
            .map_err(|e| SearchError::Provider(format!("invalid instance URL: {}", e)))?;

        let query_text = build_query_with_domains(&query.text, &query.filters.include_domains);

        {
            let mut params = url.query_pairs_mut();
            params.append_pair("q", &query_text);
            params.append_pair("format", "json");

            if let Some(ref recency) = query.filters.recency {
                if let Some(time_range) = map_recency(recency) {
                    params.append_pair("time_range", time_range);
                }
            }

            if let Some(ref content_type) = query.filters.content_type {
                params.append_pair("categories", map_content_type(content_type));
            }
        }

        Ok(url)
    }
}

#[async_trait]
impl SearchProvider for SearxngProvider {
    #[instrument(skip(self), fields(provider = PROVIDER_ID, instance = %self.instance_url))]
    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, SearchError> {
        let url = self.build_url(query)?;

        debug!(url = %url, "executing searxng search");

        let response = self
            .client
            .get(url.as_str())
            .send()
            .await
            .map_err(|e| map_reqwest_error(e, self.client.timeout().as_secs()))?;

        let status = response.status();

        if !status.is_success() {
            return Err(map_http_error(status));
        }

        let searxng_response: SearxngResponse = response.json().await.map_err(|e| {
            warn!(error = %e, "failed to parse searxng response");
            SearchError::Provider(format!("failed to parse response: {}", e))
        })?;

        debug!(
            result_count = searxng_response.results.len(),
            "searxng search completed"
        );

        let results = searxng_response
            .results
            .into_iter()
            .map(|r| {
                let snippet = r.content.unwrap_or_default();
                let score = normalize_score(r.score);
                SearchResult::new(r.url, r.title, snippet).with_score(score)
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
// Response Types
// ============================================================================

/// Response from SearXNG search API.
#[derive(Debug, Deserialize)]
struct SearxngResponse {
    #[allow(dead_code)]
    query: Option<String>,
    results: Vec<SearxngResult>,
}

/// Individual search result from SearXNG.
#[derive(Debug, Deserialize)]
struct SearxngResult {
    title: String,
    url: String,
    #[serde(default)]
    content: Option<String>,
    /// The primary engine that returned this result.
    #[serde(default)]
    #[allow(dead_code)]
    engine: Option<String>,
    /// All engines that returned this result.
    #[serde(default)]
    #[allow(dead_code)]
    engines: Vec<String>,
    /// Relevance score (can be missing or inconsistent across engines).
    #[serde(default)]
    score: Option<f32>,
}

// ============================================================================
// Mapping Functions
// ============================================================================

/// Maps Recency to SearXNG time_range parameter.
fn map_recency(recency: &Recency) -> Option<&'static str> {
    match recency {
        Recency::Day => Some("day"),
        Recency::Week => Some("week"),
        Recency::Month => Some("month"),
        Recency::Year => Some("year"),
        Recency::Any => None,
        _ => None,
    }
}

/// Maps ContentType to SearXNG categories parameter.
fn map_content_type(content_type: &ContentType) -> &'static str {
    match content_type {
        ContentType::News => "news",
        ContentType::Academic => "science",
        ContentType::General => "general",
        ContentType::Blog => "general",
        ContentType::Forum => "general",
        _ => "general",
    }
}

/// Builds query string with site: operators for domain filtering.
///
/// SearXNG doesn't have an API parameter for domain filtering, so we use
/// the `site:domain.com` query syntax.
fn build_query_with_domains(query: &str, include_domains: &Option<Vec<String>>) -> String {
    match include_domains {
        Some(domains) if !domains.is_empty() => {
            let site_filter = if domains.len() == 1 {
                format!("site:{}", domains[0])
            } else {
                let sites: Vec<String> = domains.iter().map(|d| format!("site:{}", d)).collect();
                format!("({})", sites.join(" OR "))
            };
            format!("{} {}", query, site_filter)
        }
        _ => query.to_string(),
    }
}

/// Normalizes SearXNG scores to a 0.0-1.0 range.
///
/// SearXNG scores can vary widely or be missing entirely. We normalize using
/// a reasonable default and clamping.
fn normalize_score(score: Option<f32>) -> f32 {
    match score {
        Some(s) if s > 0.0 => (s / 5.0).clamp(0.0, 1.0),
        _ => 0.5,
    }
}

fn map_reqwest_error(error: reqwest::Error, timeout_secs: u64) -> SearchError {
    if error.is_timeout() {
        SearchError::Timeout { timeout_secs }
    } else if error.is_connect() {
        SearchError::ProviderUnavailable {
            provider: PROVIDER_ID.to_string(),
        }
    } else {
        SearchError::Network(error.to_string())
    }
}

fn map_http_error(status: reqwest::StatusCode) -> SearchError {
    match status.as_u16() {
        403 => SearchError::ProviderUnavailable {
            provider: PROVIDER_ID.to_string(),
        },
        429 => SearchError::RateLimited {
            provider: PROVIDER_ID.to_string(),
        },
        400 => SearchError::InvalidQuery {
            reason: "bad request".to_string(),
        },
        502..=504 => SearchError::ProviderUnavailable {
            provider: PROVIDER_ID.to_string(),
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
        let provider = SearxngProvider::new("https://searx.example.org");
        assert_eq!(provider.provider_id(), "searxng");
        assert_eq!(provider.instance_url(), "https://searx.example.org");
        assert!(provider.supports_recency_filter());
        assert!(provider.supports_domain_filter());
    }

    #[test]
    fn creates_provider_strips_trailing_slash() {
        let provider = SearxngProvider::new("https://searx.example.org/");
        assert_eq!(provider.instance_url(), "https://searx.example.org");
    }

    #[test]
    fn creates_provider_from_env() {
        std::env::remove_var("SEARXNG_URL");
        let provider = SearxngProvider::from_env();
        assert_eq!(provider.instance_url(), DEFAULT_INSTANCE_URL);
    }

    #[test]
    fn builds_basic_url() {
        let provider = SearxngProvider::new("https://searx.example.org");
        let query = SearchQuery::new("test query");

        let url = provider.build_url(&query).unwrap();

        assert!(url.as_str().starts_with("https://searx.example.org/search"));
        assert!(url.as_str().contains("q=test+query"));
        assert!(url.as_str().contains("format=json"));
    }

    #[test]
    fn builds_url_with_recency_filter() {
        let provider = SearxngProvider::new("https://searx.example.org");
        let query =
            SearchQuery::new("test").with_filters(SearchFilters::new().with_recency(Recency::Week));

        let url = provider.build_url(&query).unwrap();

        assert!(url.as_str().contains("time_range=week"));
    }

    #[test]
    fn builds_url_with_content_type() {
        let provider = SearxngProvider::new("https://searx.example.org");
        let query = SearchQuery::new("test")
            .with_filters(SearchFilters::new().with_content_type(ContentType::News));

        let url = provider.build_url(&query).unwrap();

        assert!(url.as_str().contains("categories=news"));
    }

    #[test]
    fn builds_url_with_single_domain_filter() {
        let provider = SearxngProvider::new("https://searx.example.org");
        let query = SearchQuery::new("rust programming")
            .with_filters(SearchFilters::new().include_domains(["rust-lang.org"]));

        let url = provider.build_url(&query).unwrap();

        assert!(url.as_str().contains("site%3Arust-lang.org"));
    }

    #[test]
    fn builds_url_with_multiple_domain_filters() {
        let provider = SearxngProvider::new("https://searx.example.org");
        let query = SearchQuery::new("test")
            .with_filters(SearchFilters::new().include_domains(["example.com", "test.com"]));

        let url = provider.build_url(&query).unwrap();
        let url_str = url.as_str();

        assert!(url_str.contains("site%3Aexample.com"));
        assert!(url_str.contains("site%3Atest.com"));
        assert!(url_str.contains("+OR+"));
    }

    #[test]
    fn maps_all_recency_values() {
        assert_eq!(map_recency(&Recency::Day), Some("day"));
        assert_eq!(map_recency(&Recency::Week), Some("week"));
        assert_eq!(map_recency(&Recency::Month), Some("month"));
        assert_eq!(map_recency(&Recency::Year), Some("year"));
        assert_eq!(map_recency(&Recency::Any), None);
    }

    #[test]
    fn maps_all_content_types() {
        assert_eq!(map_content_type(&ContentType::News), "news");
        assert_eq!(map_content_type(&ContentType::Academic), "science");
        assert_eq!(map_content_type(&ContentType::General), "general");
        assert_eq!(map_content_type(&ContentType::Blog), "general");
        assert_eq!(map_content_type(&ContentType::Forum), "general");
    }

    #[test]
    fn builds_query_with_no_domains() {
        let result = build_query_with_domains("test query", &None);
        assert_eq!(result, "test query");
    }

    #[test]
    fn builds_query_with_empty_domains() {
        let result = build_query_with_domains("test query", &Some(vec![]));
        assert_eq!(result, "test query");
    }

    #[test]
    fn builds_query_with_single_domain() {
        let result = build_query_with_domains("test", &Some(vec!["example.com".to_string()]));
        assert_eq!(result, "test site:example.com");
    }

    #[test]
    fn builds_query_with_multiple_domains() {
        let result = build_query_with_domains(
            "test",
            &Some(vec!["a.com".to_string(), "b.com".to_string()]),
        );
        assert_eq!(result, "test (site:a.com OR site:b.com)");
    }

    #[test]
    fn normalizes_scores() {
        assert_eq!(normalize_score(Some(5.0)), 1.0);
        assert_eq!(normalize_score(Some(2.5)), 0.5);
        assert_eq!(normalize_score(Some(0.0)), 0.5);
        assert_eq!(normalize_score(None), 0.5);
        assert_eq!(normalize_score(Some(10.0)), 1.0);
    }

    #[test]
    fn maps_http_errors() {
        assert!(matches!(
            map_http_error(reqwest::StatusCode::FORBIDDEN),
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
            map_http_error(reqwest::StatusCode::BAD_GATEWAY),
            SearchError::ProviderUnavailable { .. }
        ));
        assert!(matches!(
            map_http_error(reqwest::StatusCode::SERVICE_UNAVAILABLE),
            SearchError::ProviderUnavailable { .. }
        ));
        assert!(matches!(
            map_http_error(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
            SearchError::Provider(_)
        ));
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
                    "engine": "google",
                    "engines": ["google", "bing"],
                    "score": 2.5
                }
            ]
        }"#;

        let response: SearxngResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.query, Some("test query".to_string()));
        assert_eq!(response.results.len(), 1);
        assert_eq!(response.results[0].title, "Test Title");
        assert_eq!(response.results[0].url, "https://example.com");
        assert_eq!(
            response.results[0].content,
            Some("Test content snippet".to_string())
        );
        assert_eq!(response.results[0].engine, Some("google".to_string()));
        assert_eq!(response.results[0].engines, vec!["google", "bing"]);
        assert_eq!(response.results[0].score, Some(2.5));
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

        let response: SearxngResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.query, None);
        assert_eq!(response.results[0].content, None);
        assert_eq!(response.results[0].engine, None);
        assert!(response.results[0].engines.is_empty());
        assert_eq!(response.results[0].score, None);
    }

    #[test]
    fn deserializes_empty_results() {
        let json = r#"{"results": []}"#;

        let response: SearxngResponse = serde_json::from_str(json).unwrap();

        assert!(response.results.is_empty());
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn searches_with_public_instance() {
        let instance_url =
            std::env::var("SEARXNG_URL").unwrap_or_else(|_| "https://searx.be".to_string());
        let provider = SearxngProvider::new(instance_url);
        let query = SearchQuery::new("What is Rust programming language?");

        let results = provider.search(&query).await;

        match results {
            Ok(results) => {
                assert!(!results.is_empty(), "should return results");
                assert!(!results[0].url.is_empty(), "results should have URLs");
                assert!(!results[0].title.is_empty(), "results should have titles");
            }
            Err(SearchError::ProviderUnavailable { .. }) => {
                eprintln!("Public SearXNG instance unavailable, skipping assertion");
            }
            Err(e) => panic!("unexpected error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn searches_with_filters() {
        let instance_url =
            std::env::var("SEARXNG_URL").unwrap_or_else(|_| "https://searx.be".to_string());
        let provider = SearxngProvider::new(instance_url);
        let query = SearchQuery::new("technology news").with_filters(
            SearchFilters::new()
                .with_recency(Recency::Week)
                .with_content_type(ContentType::News),
        );

        let results = provider.search(&query).await;

        match results {
            Ok(results) => {
                println!("Got {} results with filters", results.len());
            }
            Err(SearchError::ProviderUnavailable { .. }) => {
                eprintln!("Public SearXNG instance unavailable, skipping");
            }
            Err(e) => panic!("unexpected error: {:?}", e),
        }
    }
}
