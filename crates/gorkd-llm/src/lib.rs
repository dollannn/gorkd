#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! LLM provider implementations (OpenAI, Anthropic).

/// Anthropic (Claude) provider.
pub mod anthropic;
/// OpenAI (GPT) provider.
pub mod openai;
