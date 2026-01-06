#![allow(missing_docs)]

use std::env;
use std::time::Duration;

use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConfigError {
    #[error("no search providers configured - set at least one of: TAVILY_API_KEY, EXA_API_KEY, or SEARXNG_URL")]
    NoProvidersConfigured,

    #[error("invalid SEARXNG_URL: {0}")]
    InvalidSearxngUrl(String),

    #[error("invalid {name}: {reason}")]
    InvalidValue { name: String, reason: String },
}

const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_MAX_RESULTS: usize = 10;

#[derive(Clone, Debug)]
pub struct SearchConfig {
    pub tavily_api_key: Option<String>,
    pub exa_api_key: Option<String>,
    pub searxng_url: Option<String>,
    pub timeout: Duration,
    pub max_results: usize,
}

impl SearchConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let tavily_api_key = env::var("TAVILY_API_KEY").ok().filter(|s| !s.is_empty());
        let exa_api_key = env::var("EXA_API_KEY").ok().filter(|s| !s.is_empty());
        let searxng_url = env::var("SEARXNG_URL").ok().filter(|s| !s.is_empty());

        if tavily_api_key.is_none() && exa_api_key.is_none() && searxng_url.is_none() {
            return Err(ConfigError::NoProvidersConfigured);
        }

        if let Some(ref url) = searxng_url {
            if url::Url::parse(url).is_err() {
                return Err(ConfigError::InvalidSearxngUrl(url.clone()));
            }
        }

        let timeout_secs = env::var("SEARCH_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(DEFAULT_TIMEOUT_SECS);

        let max_results = env::var("SEARCH_MAX_RESULTS")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(DEFAULT_MAX_RESULTS);

        Ok(Self {
            tavily_api_key,
            exa_api_key,
            searxng_url,
            timeout: Duration::from_secs(timeout_secs),
            max_results,
        })
    }

    pub fn has_tavily(&self) -> bool {
        self.tavily_api_key.is_some()
    }

    pub fn has_exa(&self) -> bool {
        self.exa_api_key.is_some()
    }

    pub fn has_searxng(&self) -> bool {
        self.searxng_url.is_some()
    }

    pub fn available_providers(&self) -> Vec<&'static str> {
        let mut providers = Vec::new();
        if self.has_tavily() {
            providers.push("tavily");
        }
        if self.has_exa() {
            providers.push("exa");
        }
        if self.has_searxng() {
            providers.push("searxng");
        }
        providers
    }
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            tavily_api_key: None,
            exa_api_key: None,
            searxng_url: None,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            max_results: DEFAULT_MAX_RESULTS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn clear_env() {
        env::remove_var("TAVILY_API_KEY");
        env::remove_var("EXA_API_KEY");
        env::remove_var("SEARXNG_URL");
        env::remove_var("SEARCH_TIMEOUT_SECS");
        env::remove_var("SEARCH_MAX_RESULTS");
    }

    #[test]
    fn fails_when_no_providers_configured() {
        clear_env();
        let result = SearchConfig::from_env();
        assert!(matches!(result, Err(ConfigError::NoProvidersConfigured)));
    }

    #[test]
    fn loads_tavily_config() {
        clear_env();
        env::set_var("TAVILY_API_KEY", "tvly-test-key");

        let config = SearchConfig::from_env().unwrap();
        assert!(config.has_tavily());
        assert!(!config.has_exa());
        assert!(!config.has_searxng());
        assert_eq!(config.tavily_api_key.as_deref(), Some("tvly-test-key"));
    }

    #[test]
    fn loads_exa_config() {
        clear_env();
        env::set_var("EXA_API_KEY", "exa-test-key");

        let config = SearchConfig::from_env().unwrap();
        assert!(!config.has_tavily());
        assert!(config.has_exa());
        assert_eq!(config.exa_api_key.as_deref(), Some("exa-test-key"));
    }

    #[test]
    fn loads_searxng_config() {
        clear_env();
        env::set_var("SEARXNG_URL", "http://localhost:8080");

        let config = SearchConfig::from_env().unwrap();
        assert!(config.has_searxng());
        assert_eq!(config.searxng_url.as_deref(), Some("http://localhost:8080"));
    }

    #[test]
    fn rejects_invalid_searxng_url() {
        clear_env();
        env::set_var("SEARXNG_URL", "not-a-valid-url");

        let result = SearchConfig::from_env();
        assert!(matches!(result, Err(ConfigError::InvalidSearxngUrl(_))));
    }

    #[test]
    fn uses_default_timeout_and_max_results() {
        clear_env();
        env::set_var("TAVILY_API_KEY", "test");

        let config = SearchConfig::from_env().unwrap();
        assert_eq!(config.timeout, Duration::from_secs(DEFAULT_TIMEOUT_SECS));
        assert_eq!(config.max_results, DEFAULT_MAX_RESULTS);
    }

    #[test]
    fn loads_custom_timeout_and_max_results() {
        clear_env();
        env::set_var("TAVILY_API_KEY", "test");
        env::set_var("SEARCH_TIMEOUT_SECS", "60");
        env::set_var("SEARCH_MAX_RESULTS", "25");

        let config = SearchConfig::from_env().unwrap();
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.max_results, 25);
    }

    #[test]
    fn ignores_empty_env_vars() {
        clear_env();
        env::set_var("TAVILY_API_KEY", "");
        env::set_var("EXA_API_KEY", "valid-key");

        let config = SearchConfig::from_env().unwrap();
        assert!(!config.has_tavily());
        assert!(config.has_exa());
    }

    #[test]
    fn lists_available_providers() {
        clear_env();
        env::set_var("TAVILY_API_KEY", "key1");
        env::set_var("EXA_API_KEY", "key2");

        let config = SearchConfig::from_env().unwrap();
        let providers = config.available_providers();
        assert_eq!(providers, vec!["tavily", "exa"]);
    }

    #[test]
    fn default_config_has_no_providers() {
        let config = SearchConfig::default();
        assert!(!config.has_tavily());
        assert!(!config.has_exa());
        assert!(!config.has_searxng());
        assert!(config.available_providers().is_empty());
    }
}
