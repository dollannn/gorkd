use std::env;
use std::time::Duration;

use secrecy::{ExposeSecret, SecretString};

pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
pub const DEFAULT_MAX_RETRIES: u32 = 2;

#[derive(Clone)]
pub struct AnthropicConfig {
    pub api_key: SecretString,
    pub base_url: String,
}

impl AnthropicConfig {
    pub fn from_env() -> Option<Self> {
        let api_key = env::var("ANTHROPIC_API_KEY").ok()?;
        let base_url = env::var("ANTHROPIC_BASE_URL")
            .unwrap_or_else(|_| "https://api.anthropic.com".to_string());

        Some(Self {
            api_key: SecretString::from(api_key),
            base_url,
        })
    }
}

impl std::fmt::Debug for AnthropicConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnthropicConfig")
            .field("api_key", &"[REDACTED]")
            .field("base_url", &self.base_url)
            .finish()
    }
}

#[derive(Clone)]
pub struct OpenAiConfig {
    pub api_key: SecretString,
    pub base_url: String,
}

impl OpenAiConfig {
    pub fn from_env() -> Option<Self> {
        let api_key = env::var("OPENAI_API_KEY").ok()?;
        let base_url =
            env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com".to_string());

        Some(Self {
            api_key: SecretString::from(api_key),
            base_url,
        })
    }
}

impl std::fmt::Debug for OpenAiConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAiConfig")
            .field("api_key", &"[REDACTED]")
            .field("base_url", &self.base_url)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct LlmConfig {
    pub default_model: String,
    pub fallback_model: Option<String>,
    pub timeout: Duration,
    pub max_retries: u32,
    pub anthropic: Option<AnthropicConfig>,
    pub openai: Option<OpenAiConfig>,
}

impl LlmConfig {
    pub fn from_env() -> Self {
        let default_model = env::var("LLM_DEFAULT_MODEL")
            .unwrap_or_else(|_| "claude-sonnet-4-20250514".to_string());
        let fallback_model = env::var("LLM_FALLBACK_MODEL").ok();
        let timeout_secs = env::var("LLM_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_TIMEOUT_SECS);
        let max_retries = env::var("LLM_MAX_RETRIES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_MAX_RETRIES);

        Self {
            default_model,
            fallback_model,
            timeout: Duration::from_secs(timeout_secs),
            max_retries,
            anthropic: AnthropicConfig::from_env(),
            openai: OpenAiConfig::from_env(),
        }
    }

    pub fn has_provider(&self) -> bool {
        self.anthropic.is_some() || self.openai.is_some()
    }

    pub fn anthropic_api_key(&self) -> Option<&str> {
        self.anthropic.as_ref().map(|c| c.api_key.expose_secret())
    }

    pub fn openai_api_key(&self) -> Option<&str> {
        self.openai.as_ref().map(|c| c.api_key.expose_secret())
    }
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            default_model: "claude-sonnet-4-20250514".to_string(),
            fallback_model: None,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            max_retries: DEFAULT_MAX_RETRIES,
            anthropic: None,
            openai: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_reasonable_values() {
        let config = LlmConfig::default();

        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.max_retries, 2);
        assert!(!config.has_provider());
    }

    #[test]
    fn config_debug_redacts_api_keys() {
        let anthropic = AnthropicConfig {
            api_key: SecretString::from("sk-secret-key"),
            base_url: "https://api.anthropic.com".to_string(),
        };

        let debug_str = format!("{:?}", anthropic);
        assert!(!debug_str.contains("sk-secret-key"));
        assert!(debug_str.contains("[REDACTED]"));
    }
}
