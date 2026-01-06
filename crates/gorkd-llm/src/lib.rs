#![forbid(unsafe_code)]

pub mod anthropic;
pub mod client;
pub mod config;
pub mod error;
pub mod openai;
pub mod prompt;
pub mod registry;
pub mod types;

pub use anthropic::AnthropicProvider;
pub use client::{build_http_client, build_http_client_with_timeout, default_http_client};
pub use config::{
    AnthropicConfig, LlmConfig, OpenAiConfig, DEFAULT_MAX_RETRIES, DEFAULT_TIMEOUT_SECS,
};
pub use error::{map_anthropic_error, map_openai_error, map_reqwest_error};
pub use openai::OpenAiProvider;
pub use prompt::{
    build_synthesis_messages, estimate_messages_tokens, estimate_token_count,
    SYNTHESIS_SYSTEM_PROMPT,
};
pub use registry::{LlmRegistry, LlmRegistryBuilder};
pub use types::{ChatRequest, ChatResponse, FinishReason, Message, Role, TokenUsage};
