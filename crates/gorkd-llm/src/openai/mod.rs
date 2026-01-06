mod client;
mod parser;
pub mod types;

use std::time::Instant;

use async_trait::async_trait;
use gorkd_core::{LlmError, LlmProvider, ResearchAnswer, Source};
use reqwest::Client;
use tracing::instrument;

use crate::config::OpenAiConfig;
use crate::prompt::build_synthesis_messages;

use client::OpenAiClient;
pub use parser::ParseError;
use types::{ChatMessage, FinishReason, CONTEXT_WINDOW_TOKENS, DEFAULT_MAX_TOKENS};

pub struct OpenAiProvider {
    client: OpenAiClient,
    model: String,
    max_tokens: usize,
}

impl OpenAiProvider {
    pub fn new(http: Client, config: &OpenAiConfig, model: impl Into<String>) -> Self {
        Self {
            client: OpenAiClient::new(http, config),
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
impl LlmProvider for OpenAiProvider {
    #[instrument(skip(self, sources), fields(model = %self.model, source_count = sources.len()))]
    async fn synthesize(
        &self,
        query: &str,
        sources: &[Source],
    ) -> Result<ResearchAnswer, LlmError> {
        let start = Instant::now();

        let messages = build_synthesis_messages(query, sources);
        let openai_messages: Vec<ChatMessage> = messages
            .iter()
            .map(|m| match m.role {
                crate::types::Role::System => ChatMessage::system(&m.content),
                crate::types::Role::User => ChatMessage::user(&m.content),
                crate::types::Role::Assistant => ChatMessage::assistant(&m.content),
            })
            .collect();

        let response = self
            .client
            .send_chat_completion(&self.model, openai_messages, self.max_tokens, true)
            .await?;

        if response.finish_reason() == Some(&FinishReason::Length) {
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

    fn max_context_tokens(&self) -> usize {
        CONTEXT_WINDOW_TOKENS
    }

    fn supports_streaming(&self) -> bool {
        false
    }
}

impl std::fmt::Debug for OpenAiProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAiProvider")
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
        assert_eq!(CONTEXT_WINDOW_TOKENS, 128_000);
    }

    #[test]
    fn provider_model_constants_exist() {
        use types::{MODEL_GPT_4O, MODEL_GPT_4O_MINI};
        assert!(MODEL_GPT_4O.contains("gpt-4o"));
        assert!(MODEL_GPT_4O_MINI.contains("gpt-4o-mini"));
    }
}
