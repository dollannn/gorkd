use gorkd_core::LlmError;
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use tracing::instrument;

use crate::config::OpenAiConfig;
use crate::error::map_openai_error;

use super::types::{ChatCompletionRequest, ChatCompletionResponse, ChatMessage};

pub struct OpenAiClient {
    http: Client,
    api_key: SecretString,
    base_url: String,
}

impl OpenAiClient {
    pub fn new(http: Client, config: &OpenAiConfig) -> Self {
        Self {
            http,
            api_key: config.api_key.clone(),
            base_url: config.base_url.clone(),
        }
    }

    #[instrument(skip(self, messages), fields(model = %model))]
    pub async fn send_chat_completion(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
        max_tokens: usize,
        json_mode: bool,
    ) -> Result<ChatCompletionResponse, LlmError> {
        let mut request = ChatCompletionRequest::new(model, messages).with_max_tokens(max_tokens);

        if json_mode {
            request = request.with_json_mode();
        }

        let url = format!("{}/v1/chat/completions", self.base_url);

        let response = self
            .http
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.api_key.expose_secret()),
            )
            .header("Content-Type", "application/json")
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
            return Err(map_openai_error(status, &body));
        }

        serde_json::from_str(&body).map_err(|e| LlmError::Provider(format!("parse error: {}", e)))
    }
}

impl std::fmt::Debug for OpenAiClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAiClient")
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
        let config = OpenAiConfig {
            api_key: SecretString::from("sk-secret-key-12345"),
            base_url: "https://api.openai.com".to_string(),
        };
        let client = OpenAiClient::new(Client::new(), &config);

        let debug_str = format!("{:?}", client);
        assert!(!debug_str.contains("sk-secret-key"));
        assert!(debug_str.contains("[REDACTED]"));
    }
}
