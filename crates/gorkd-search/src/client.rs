#![allow(missing_docs)]

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use thiserror::Error;

const DEFAULT_USER_AGENT: &str = concat!("gorkd/", env!("CARGO_PKG_VERSION"));
const DEFAULT_TIMEOUT_SECS: u64 = 30;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum HttpClientError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("request timeout after {timeout_secs}s")]
    Timeout { timeout_secs: u64 },
}

#[derive(Clone)]
pub struct HttpClient {
    inner: reqwest::Client,
    timeout: Duration,
}

impl HttpClient {
    pub fn new(timeout: Duration) -> Result<Self, HttpClientError> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(timeout)
            .connect_timeout(Duration::from_secs(10))
            .build()?;

        Ok(Self {
            inner: client,
            timeout,
        })
    }

    pub fn with_default_timeout() -> Result<Self, HttpClientError> {
        Self::new(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
    }

    pub fn inner(&self) -> &reqwest::Client {
        &self.inner
    }

    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    pub fn get(&self, url: &str) -> reqwest::RequestBuilder {
        self.inner.get(url)
    }

    pub fn post(&self, url: &str) -> reqwest::RequestBuilder {
        self.inner.post(url)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::with_default_timeout().expect("failed to create default HTTP client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_client_with_custom_timeout() {
        let client = HttpClient::new(Duration::from_secs(60)).unwrap();
        assert_eq!(client.timeout(), Duration::from_secs(60));
    }

    #[test]
    fn creates_client_with_default_timeout() {
        let client = HttpClient::with_default_timeout().unwrap();
        assert_eq!(client.timeout(), Duration::from_secs(DEFAULT_TIMEOUT_SECS));
    }

    #[test]
    fn default_creates_client() {
        let client = HttpClient::default();
        assert_eq!(client.timeout(), Duration::from_secs(DEFAULT_TIMEOUT_SECS));
    }

    #[test]
    fn exposes_inner_client() {
        let client = HttpClient::default();
        let _ = client.inner();
    }

    #[test]
    fn creates_get_request() {
        let client = HttpClient::default();
        let _request = client.get("https://example.com");
    }

    #[test]
    fn creates_post_request() {
        let client = HttpClient::default();
        let _request = client.post("https://example.com");
    }
}
