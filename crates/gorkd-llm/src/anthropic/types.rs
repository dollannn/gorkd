//! Anthropic API request and response types.
//!
//! These types map directly to the Anthropic Messages API.
//! See: https://docs.anthropic.com/en/api/messages

use serde::{Deserialize, Serialize};

/// Anthropic API version header value.
pub const ANTHROPIC_VERSION: &str = "2023-06-01";

/// Default max tokens for Claude responses (required parameter).
pub const DEFAULT_MAX_TOKENS: usize = 4096;

/// Claude Sonnet 4 model ID (primary model).
pub const MODEL_CLAUDE_SONNET_4: &str = "claude-sonnet-4-20250514";

/// Claude Haiku 3.5 model ID (fast/cheap fallback).
pub const MODEL_CLAUDE_HAIKU_35: &str = "claude-3-5-haiku-20241022";

/// Context window size for Claude models (200K tokens).
pub const CONTEXT_WINDOW_TOKENS: usize = 200_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicMessage {
    pub role: MessageRole,
    pub content: String,
}

impl AnthropicMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MessagesRequest {
    pub model: String,
    pub max_tokens: usize,
    pub messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

impl MessagesRequest {
    pub fn new(model: impl Into<String>, messages: Vec<AnthropicMessage>) -> Self {
        Self {
            model: model.into(),
            max_tokens: DEFAULT_MAX_TOKENS,
            messages,
            system: None,
            temperature: None,
        }
    }

    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature.clamp(0.0, 1.0));
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text { text: String },
}

#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    pub input_tokens: usize,
    pub output_tokens: usize,
}

impl Usage {
    pub fn total(&self) -> usize {
        self.input_tokens + self.output_tokens
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    EndTurn,
    MaxTokens,
    StopSequence,
    ToolUse,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessagesResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub response_type: String,
    pub role: MessageRole,
    pub content: Vec<ContentBlock>,
    pub model: String,
    pub stop_reason: Option<StopReason>,
    pub usage: Usage,
}

impl MessagesResponse {
    pub fn text_content(&self) -> String {
        self.content
            .iter()
            .map(|block| match block {
                ContentBlock::Text { text } => text.as_str(),
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_user_message() {
        let msg = AnthropicMessage::user("Hello");
        assert_eq!(msg.role, MessageRole::User);
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn creates_assistant_message() {
        let msg = AnthropicMessage::assistant("Hi there!");
        assert_eq!(msg.role, MessageRole::Assistant);
        assert_eq!(msg.content, "Hi there!");
    }

    #[test]
    fn serializes_request() {
        let request =
            MessagesRequest::new(MODEL_CLAUDE_SONNET_4, vec![AnthropicMessage::user("Hello")])
                .with_system("You are helpful.")
                .with_temperature(0.7);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("claude-sonnet-4"));
        assert!(json.contains("You are helpful."));
        assert!(json.contains("0.7"));
    }

    #[test]
    fn clamps_temperature() {
        let request = MessagesRequest::new(MODEL_CLAUDE_SONNET_4, vec![]).with_temperature(2.0);
        assert_eq!(request.temperature, Some(1.0));

        let request = MessagesRequest::new(MODEL_CLAUDE_SONNET_4, vec![]).with_temperature(-0.5);
        assert_eq!(request.temperature, Some(0.0));
    }

    #[test]
    fn deserializes_response() {
        let json = r#"{
            "id": "msg_123",
            "type": "message",
            "role": "assistant",
            "content": [{"type": "text", "text": "Hello!"}],
            "model": "claude-sonnet-4-20250514",
            "stop_reason": "end_turn",
            "usage": {"input_tokens": 10, "output_tokens": 5}
        }"#;

        let response: MessagesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "msg_123");
        assert_eq!(response.text_content(), "Hello!");
        assert_eq!(response.stop_reason, Some(StopReason::EndTurn));
        assert_eq!(response.usage.total(), 15);
    }

    #[test]
    fn deserializes_max_tokens_stop_reason() {
        let json = r#"{
            "id": "msg_123",
            "type": "message",
            "role": "assistant",
            "content": [{"type": "text", "text": "Truncated..."}],
            "model": "claude-sonnet-4-20250514",
            "stop_reason": "max_tokens",
            "usage": {"input_tokens": 100, "output_tokens": 4096}
        }"#;

        let response: MessagesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.stop_reason, Some(StopReason::MaxTokens));
    }

    #[test]
    fn handles_multiple_content_blocks() {
        let json = r#"{
            "id": "msg_123",
            "type": "message",
            "role": "assistant",
            "content": [
                {"type": "text", "text": "First part. "},
                {"type": "text", "text": "Second part."}
            ],
            "model": "claude-sonnet-4-20250514",
            "stop_reason": "end_turn",
            "usage": {"input_tokens": 10, "output_tokens": 20}
        }"#;

        let response: MessagesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.text_content(), "First part. Second part.");
    }
}
