use gorkd_core::LlmError;
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use tracing::instrument;

use crate::config::AnthropicConfig;
use crate::error::map_anthropic_error;

use super::types::{AnthropicMessage, MessagesRequest, MessagesResponse, ANTHROPIC_VERSION};

pub struct AnthropicClient {
    http: Client,
    api_key: SecretString,
    base_url: String,
}

impl AnthropicClient {
    pub fn new(http: Client, config: &AnthropicConfig) -> Self {
        Self {
            http,
            api_key: config.api_key.clone(),
            base_url: config.base_url.clone(),
        }
    }

    #[instrument(skip(self, system, messages), fields(model = %model))]
    pub async fn send_message(
        &self,
        model: &str,
        system: &str,
        messages: Vec<AnthropicMessage>,
        max_tokens: usize,
    ) -> Result<MessagesResponse, LlmError> {
        let request = MessagesRequest::new(model, messages)
            .with_system(system)
            .with_max_tokens(max_tokens);

        let url = format!("{}/v1/messages", self.base_url);

        let response = self
            .http
            .post(&url)
            .header("x-api-key", self.api_key.expose_secret())
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(crate::error::map_reqwest_error)?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(crate::error::map_reqwest_error)?;

        if !status.is_success() {
            return Err(map_anthropic_error(status, &body));
        }

        serde_json::from_str(&body).map_err(|e| LlmError::Provider(format!("parse error: {}", e)))
    }
}

impl std::fmt::Debug for AnthropicClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnthropicClient")
            .field("base_url", &self.base_url)
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_redacts_api_key() {
        let config = AnthropicConfig {
            api_key: SecretString::from("sk-secret-key"),
            base_url: "https://api.anthropic.com".to_string(),
        };
        let client = AnthropicClient::new(Client::new(), &config);

        let debug_str = format!("{:?}", client);
        assert!(!debug_str.contains("sk-secret-key"));
        assert!(debug_str.contains("[REDACTED]"));
    }
}
