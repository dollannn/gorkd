//! OpenAI API request and response types.
//!
//! These types map directly to the OpenAI Chat Completions API.
//! See: https://platform.openai.com/docs/api-reference/chat

use serde::{Deserialize, Serialize};

/// GPT-4o model ID (primary fallback model).
pub const MODEL_GPT_4O: &str = "gpt-4o";

/// GPT-4o-mini model ID (cheap/fast model for simple tasks).
pub const MODEL_GPT_4O_MINI: &str = "gpt-4o-mini";

/// Context window size for GPT-4o models (128K tokens).
pub const CONTEXT_WINDOW_TOKENS: usize = 128_000;

/// Default max tokens for GPT responses.
pub const DEFAULT_MAX_TOKENS: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::System,
            content: content.into(),
        }
    }

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
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub format_type: String,
}

impl ResponseFormat {
    pub fn json() -> Self {
        Self {
            format_type: "json_object".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

impl ChatCompletionRequest {
    pub fn new(model: impl Into<String>, messages: Vec<ChatMessage>) -> Self {
        Self {
            model: model.into(),
            messages,
            max_tokens: None,
            temperature: None,
            response_format: None,
        }
    }

    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature.clamp(0.0, 2.0));
        self
    }

    pub fn with_json_mode(mut self) -> Self {
        self.response_format = Some(ResponseFormat::json());
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

impl Usage {
    pub fn total(&self) -> usize {
        self.total_tokens
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChoiceMessage {
    pub role: MessageRole,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    pub index: usize,
    pub message: ChoiceMessage,
    pub finish_reason: Option<FinishReason>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

impl ChatCompletionResponse {
    /// Extract the text content from the first choice.
    pub fn text_content(&self) -> String {
        self.choices
            .first()
            .and_then(|c| c.message.content.as_ref())
            .cloned()
            .unwrap_or_default()
    }

    /// Get the finish reason from the first choice.
    pub fn finish_reason(&self) -> Option<&FinishReason> {
        self.choices.first().and_then(|c| c.finish_reason.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_system_message() {
        let msg = ChatMessage::system("You are helpful.");
        assert_eq!(msg.role, MessageRole::System);
        assert_eq!(msg.content, "You are helpful.");
    }

    #[test]
    fn creates_user_message() {
        let msg = ChatMessage::user("Hello");
        assert_eq!(msg.role, MessageRole::User);
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn creates_assistant_message() {
        let msg = ChatMessage::assistant("Hi there!");
        assert_eq!(msg.role, MessageRole::Assistant);
        assert_eq!(msg.content, "Hi there!");
    }

    #[test]
    fn serializes_request() {
        let request = ChatCompletionRequest::new(
            MODEL_GPT_4O,
            vec![
                ChatMessage::system("You are helpful."),
                ChatMessage::user("Hello"),
            ],
        )
        .with_max_tokens(1000)
        .with_temperature(0.7);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("gpt-4o"));
        assert!(json.contains("\"role\":\"system\""));
        assert!(json.contains("\"max_tokens\":1000"));
        assert!(json.contains("0.7"));
    }

    #[test]
    fn serializes_request_with_json_mode() {
        let request = ChatCompletionRequest::new(MODEL_GPT_4O, vec![ChatMessage::user("Hello")])
            .with_json_mode();

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"response_format\""));
        assert!(json.contains("\"type\":\"json_object\""));
    }

    #[test]
    fn clamps_temperature() {
        let request = ChatCompletionRequest::new(MODEL_GPT_4O, vec![]).with_temperature(3.0);
        assert_eq!(request.temperature, Some(2.0));

        let request = ChatCompletionRequest::new(MODEL_GPT_4O, vec![]).with_temperature(-1.0);
        assert_eq!(request.temperature, Some(0.0));
    }

    #[test]
    fn deserializes_response() {
        let json = r#"{
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "created": 1677652288,
            "model": "gpt-4o",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "Hello! How can I help you?"
                },
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 8,
                "total_tokens": 18
            }
        }"#;

        let response: ChatCompletionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "chatcmpl-123");
        assert_eq!(response.text_content(), "Hello! How can I help you?");
        assert_eq!(response.finish_reason(), Some(&FinishReason::Stop));
        assert_eq!(response.usage.total(), 18);
    }

    #[test]
    fn deserializes_length_finish_reason() {
        let json = r#"{
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "created": 1677652288,
            "model": "gpt-4o",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "Truncated..."
                },
                "finish_reason": "length"
            }],
            "usage": {
                "prompt_tokens": 100,
                "completion_tokens": 4096,
                "total_tokens": 4196
            }
        }"#;

        let response: ChatCompletionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.finish_reason(), Some(&FinishReason::Length));
    }

    #[test]
    fn deserializes_content_filter_finish_reason() {
        let json = r#"{
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "created": 1677652288,
            "model": "gpt-4o",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": null
                },
                "finish_reason": "content_filter"
            }],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 0,
                "total_tokens": 10
            }
        }"#;

        let response: ChatCompletionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.finish_reason(), Some(&FinishReason::ContentFilter));
        assert_eq!(response.text_content(), "");
    }

    #[test]
    fn handles_null_content() {
        let json = r#"{
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "created": 1677652288,
            "model": "gpt-4o",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": null
                },
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 0,
                "total_tokens": 10
            }
        }"#;

        let response: ChatCompletionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.text_content(), "");
    }

    #[test]
    fn model_constants_defined() {
        assert_eq!(MODEL_GPT_4O, "gpt-4o");
        assert_eq!(MODEL_GPT_4O_MINI, "gpt-4o-mini");
        assert_eq!(CONTEXT_WINDOW_TOKENS, 128_000);
    }
}
