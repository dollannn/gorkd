mod client;
mod parser;
pub mod types;

use std::time::Instant;

use async_trait::async_trait;
use gorkd_core::{LlmError, LlmProvider, ResearchAnswer, Source};
use reqwest::Client;
use tracing::instrument;

use crate::config::AnthropicConfig;
use crate::prompt::{build_synthesis_messages, SYNTHESIS_SYSTEM_PROMPT};

use client::AnthropicClient;
pub use parser::ParseError;
use types::{AnthropicMessage, StopReason, CONTEXT_WINDOW_TOKENS, DEFAULT_MAX_TOKENS};

pub struct AnthropicProvider {
    client: AnthropicClient,
    model: String,
    max_tokens: usize,
}

impl AnthropicProvider {
    pub fn new(http: Client, config: &AnthropicConfig, model: impl Into<String>) -> Self {
        Self {
            client: AnthropicClient::new(http, config),
            model: model.into(),
            max_tokens: DEFAULT_MAX_TOKENS,
        }
    }

    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = max_tokens;
        self
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    #[instrument(skip(self, sources), fields(model = %self.model, source_count = sources.len()))]
    async fn synthesize(
        &self,
        query: &str,
        sources: &[Source],
    ) -> Result<ResearchAnswer, LlmError> {
        let start = Instant::now();

        let messages = build_synthesis_messages(query, sources);
        let anthropic_messages: Vec<AnthropicMessage> = messages
            .iter()
            .filter(|m| !matches!(m.role, crate::types::Role::System))
            .map(|m| match m.role {
                crate::types::Role::User => AnthropicMessage::user(&m.content),
                crate::types::Role::Assistant => AnthropicMessage::assistant(&m.content),
                crate::types::Role::System => unreachable!(),
            })
            .collect();

        let response = self
            .client
            .send_message(
                &self.model,
                SYNTHESIS_SYSTEM_PROMPT,
                anthropic_messages,
                self.max_tokens,
            )
            .await?;

        if response.stop_reason == Some(StopReason::MaxTokens) {
            tracing::warn!("response truncated due to max_tokens limit");
        }

        let text = response.text_content();
        let tokens_used = response.usage.total();

        let mut answer = parser::parse_synthesis_response(&text, sources, &self.model, tokens_used)
            .map_err(|e| {
                LlmError::Provider(format!("failed to parse synthesis response: {}", e))
            })?;

        answer.synthesis_metadata.synthesis_duration = start.elapsed();

        Ok(answer)
    }

    fn model_id(&self) -> &str {
        &self.model
    }

    fn provider_name(&self) -> &str {
        "anthropic"
    }

    fn max_context_tokens(&self) -> usize {
        CONTEXT_WINDOW_TOKENS
    }

    fn supports_streaming(&self) -> bool {
        false
    }
}

impl std::fmt::Debug for AnthropicProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnthropicProvider")
            .field("model", &self.model)
            .field("max_tokens", &self.max_tokens)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_has_correct_context_window() {
        assert_eq!(CONTEXT_WINDOW_TOKENS, 200_000);
    }

    #[test]
    fn provider_model_constants_exist() {
        use types::{MODEL_CLAUDE_HAIKU_35, MODEL_CLAUDE_SONNET_4};
        assert!(MODEL_CLAUDE_SONNET_4.contains("sonnet"));
        assert!(MODEL_CLAUDE_HAIKU_35.contains("haiku"));
    }
}
