# Phase 1: Shared Infrastructure

**Status**: Complete
**Duration**: ~1 day
**Depends on**: None

## Overview

Set up the foundational infrastructure for LLM providers: configuration loading, HTTP client setup, common request/response types, and the prompt templates used for research synthesis.

## Tasks

| Task | Description |
|------|-------------|
| Add dependencies | reqwest, secrecy (for API keys), backoff (retry) |
| Create config module | Environment-based configuration for all providers |
| Create HTTP client | Shared client with timeout, retry logic |
| Define common types | Request/response structures shared across providers |
| Create prompt module | System prompts and formatting for synthesis |
| Add synthesis prompt | Research-focused prompt with citation instructions |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-llm/Cargo.toml` | Add secrecy, backoff dependencies |
| `crates/gorkd-llm/src/lib.rs` | Export new modules |
| `crates/gorkd-llm/src/config.rs` | **New file** - LlmConfig, provider configs |
| `crates/gorkd-llm/src/client.rs` | **New file** - Shared HTTP client builder |
| `crates/gorkd-llm/src/types.rs` | **New file** - Common types (Message, Role, etc.) |
| `crates/gorkd-llm/src/prompt.rs` | **New file** - Synthesis prompt templates |
| `crates/gorkd-llm/src/error.rs` | **New file** - Provider-specific error mapping |

## Configuration Schema

```
LlmConfig
├── default_model: String
├── fallback_model: Option<String>
├── timeout_secs: u64
├── max_retries: u32
├── anthropic: Option<AnthropicConfig>
│   ├── api_key: SecretString
│   └── base_url: Option<String>
└── openai: Option<OpenAiConfig>
    ├── api_key: SecretString
    └── base_url: Option<String>
```

## Key Considerations

- API keys must never be logged (use `secrecy` crate)
- HTTP client should have reasonable defaults (30s timeout, 2 retries)
- Retry logic should respect `is_retryable()` from `LlmError`
- Base URLs should be configurable for proxy/enterprise deployments
- Prompts should be testable without making API calls

## Deliverables

- [x] Config loads from environment variables
- [x] API keys are protected with `SecretString`
- [x] HTTP client has timeout and retry configured
- [x] Common message types defined
- [x] Synthesis prompt template complete
- [x] Unit tests for config parsing
- [x] `cargo build -p gorkd-llm` passes
