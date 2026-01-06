# Phase 2: Anthropic Provider

**Status**: Complete
**Duration**: ~1-2 days
**Depends on**: Phase 1 (Shared Infrastructure)

## Overview

Implement the Anthropic provider for Claude models. This is the primary provider, with Claude Sonnet 4.5 as the default model. The implementation must handle the Messages API, structured JSON output for citations, and proper error mapping.

## Tasks

| Task | Description |
|------|-------------|
| Implement Messages API client | POST /v1/messages with required headers |
| Add request serialization | Convert synthesis request to Anthropic format |
| Add response parsing | Parse Claude response, extract citations |
| Implement LlmProvider trait | Full trait implementation for AnthropicProvider |
| Handle rate limits | Respect 429 responses, map to LlmError::RateLimited |
| Add JSON mode | Use tool_use or system prompt for structured output |
| Unit tests | Mock HTTP responses for testing |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-llm/src/anthropic.rs` | **Rewrite** - Full provider implementation |
| `crates/gorkd-llm/src/anthropic/mod.rs` | **New file** - Module entry point |
| `crates/gorkd-llm/src/anthropic/client.rs` | **New file** - HTTP client for Anthropic API |
| `crates/gorkd-llm/src/anthropic/types.rs` | **New file** - Anthropic-specific request/response |
| `crates/gorkd-llm/src/anthropic/parser.rs` | **New file** - Response parsing, citation extraction |

## Anthropic API Details

**Endpoint**: `POST https://api.anthropic.com/v1/messages`

**Required Headers**:
- `x-api-key`: API key
- `anthropic-version`: `2023-06-01`
- `content-type`: `application/json`

**Supported Models**:
- `claude-sonnet-4-5` (200K context, primary)
- `claude-haiku-3-5` (200K context, future use)

## Key Considerations

- Anthropic has specific prompt formatting (Human/Assistant turns)
- Use `max_tokens` parameter (required, no default)
- JSON output via system prompt instruction (no native JSON mode)
- Rate limit headers: `anthropic-ratelimit-*` for proactive limiting
- Context window: 200K tokens, but response capped at `max_tokens`
- Token counting: Use `usage` field in response for metadata

## Citation Extraction Strategy

Instruct Claude to output structured JSON with claims and source references:
```
{
  "summary": "...",
  "detail": "...",
  "citations": [
    {"claim": "...", "source_id": "src_xxx", "quote": "..."}
  ],
  "confidence": "high|medium|low|insufficient",
  "limitations": ["..."]
}
```

## Deliverables

- [x] AnthropicProvider implements LlmProvider trait
- [x] Messages API integration working
- [x] Proper error mapping (rate limits, auth, network)
- [x] Citation extraction from response
- [x] Token usage tracked in SynthesisMetadata
- [x] Unit tests with mocked HTTP responses
- [ ] Integration test (skipped without API key) - deferred to integration phase
- [x] `cargo test -p gorkd-llm` passes
