use std::time::Duration;

use reqwest::Client;

use crate::config::{LlmConfig, DEFAULT_TIMEOUT_SECS};

pub fn build_http_client(config: &LlmConfig) -> Result<Client, reqwest::Error> {
    Client::builder()
        .timeout(config.timeout)
        .connect_timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(4)
        .build()
}

pub fn build_http_client_with_timeout(timeout_secs: u64) -> Result<Client, reqwest::Error> {
    Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .connect_timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(4)
        .build()
}

pub fn default_http_client() -> Result<Client, reqwest::Error> {
    build_http_client_with_timeout(DEFAULT_TIMEOUT_SECS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_client_with_default_config() {
        let config = LlmConfig::default();
        let client = build_http_client(&config);
        assert!(client.is_ok());
    }

    #[test]
    fn builds_client_with_custom_timeout() {
        let client = build_http_client_with_timeout(60);
        assert!(client.is_ok());
    }

    #[test]
    fn builds_default_client() {
        let client = default_http_client();
        assert!(client.is_ok());
    }
}
