# Phase 3: OpenAI Provider

**Status**: Done
**Duration**: ~1-2 days
**Depends on**: Phase 1 (Shared Infrastructure)

## Overview

Implement the OpenAI provider for GPT models. This serves as the fallback provider when Anthropic is unavailable. GPT-4o is the primary OpenAI model. The implementation must handle the Chat Completions API with JSON mode for structured output.

## Tasks

| Task | Description |
|------|-------------|
| Implement Chat Completions API | POST /v1/chat/completions |
| Add request serialization | Convert synthesis request to OpenAI format |
| Add response parsing | Parse GPT response, extract citations |
| Implement LlmProvider trait | Full trait implementation for OpenAiProvider |
| Enable JSON mode | Use `response_format: { type: "json_object" }` |
| Handle rate limits | Map 429 responses to LlmError::RateLimited |
| Unit tests | Mock HTTP responses for testing |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-llm/src/openai.rs` | **Rewrite** - Full provider implementation |
| `crates/gorkd-llm/src/openai/mod.rs` | **New file** - Module entry point |
| `crates/gorkd-llm/src/openai/client.rs` | **New file** - HTTP client for OpenAI API |
| `crates/gorkd-llm/src/openai/types.rs` | **New file** - OpenAI-specific request/response |
| `crates/gorkd-llm/src/openai/parser.rs` | **New file** - Response parsing, citation extraction |

## OpenAI API Details

**Endpoint**: `POST https://api.openai.com/v1/chat/completions`

**Required Headers**:
- `Authorization`: `Bearer {api_key}`
- `Content-Type`: `application/json`

**Supported Models**:
- `gpt-4o` (128K context, primary fallback)
- `gpt-4o-mini` (128K context, future use for cheap tasks)

## Key Considerations

- OpenAI has native JSON mode (`response_format`)
- System/User/Assistant message roles
- `max_tokens` is optional (defaults to model max)
- Rate limit headers: `x-ratelimit-*`
- Context window: 128K tokens (smaller than Claude)
- Function calling available but not needed for MVP

## JSON Mode

OpenAI's native JSON mode ensures valid JSON output:
```json
{
  "model": "gpt-4o",
  "messages": [...],
  "response_format": { "type": "json_object" }
}
```

**Important**: When using JSON mode, the system prompt MUST mention "JSON" or the API returns an error.

## Deliverables

- [x] OpenAiProvider implements LlmProvider trait
- [x] Chat Completions API integration working
- [x] JSON mode enabled for structured output
- [x] Proper error mapping (rate limits, auth, network)
- [x] Citation extraction from response
- [x] Token usage tracked in SynthesisMetadata
- [x] Unit tests with mocked HTTP responses
- [ ] Integration test (skipped without API key)
- [x] `cargo test -p gorkd-llm` passes
