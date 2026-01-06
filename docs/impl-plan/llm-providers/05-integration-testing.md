# Phase 5: Integration Testing

**Status**: Complete
**Duration**: ~1 day
**Depends on**: Phase 4

## Overview

Validate the complete LLM integration with real API calls. Create integration tests that exercise the full synthesis flow, verify citation extraction, and ensure proper error handling. Tests requiring API keys should be skipped in CI unless credentials are provided.

## Tasks

| Task | Description |
|------|-------------|
| Create test fixtures | Sample sources, queries for testing |
| Anthropic integration test | Real API call with Claude Sonnet 4.5 |
| OpenAI integration test | Real API call with GPT-4o |
| Fallback integration test | Verify fallback behavior |
| Citation validation | Ensure citations reference valid sources |
| Error handling tests | Rate limits, invalid keys, timeouts |
| Update E2E test | Full pipeline with real providers |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-llm/tests/integration.rs` | **New file** - Provider integration tests |
| `crates/gorkd-llm/tests/fixtures/mod.rs` | **New file** - Test data fixtures |
| `crates/gorkd-api/tests/integration.rs` | Update E2E test to use real providers |
| `.env.example` | Document all LLM environment variables |
| `.github/workflows/test.yml` | Add integration test job (optional) |

## Test Categories

### Unit Tests (always run)
- Config parsing
- Request serialization
- Response parsing with mock data
- Error mapping
- Registry logic

### Integration Tests (require API keys)
```rust
#[tokio::test]
#[ignore = "requires ANTHROPIC_API_KEY"]
async fn anthropic_synthesizes_answer() {
    // Skip if no API key
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .expect("Set ANTHROPIC_API_KEY to run this test");
    // ...
}
```

## Test Scenarios

| Scenario | Provider | Expected Result |
|----------|----------|-----------------|
| Simple query | Anthropic | Answer with citations |
| Simple query | OpenAI | Answer with citations |
| Empty sources | Both | Confidence::Insufficient |
| Many sources (10+) | Both | Top sources used, no context overflow |
| Fallback trigger | Mock primary + real fallback | Fallback used successfully |
| Invalid API key | Both | LlmError::Provider with auth message |

## Key Considerations

- Integration tests should be `#[ignore]` by default
- Run with `cargo test -- --ignored` when API keys available
- Tests should clean up any state they create
- Use small, focused prompts to minimize API costs
- Add `--test-threads=1` for rate limit safety
- Log API responses in debug mode for troubleshooting

## Deliverables

- [x] Test fixtures with sample sources
- [x] Anthropic integration test passing
- [x] OpenAI integration test passing
- [x] Fallback behavior verified
- [x] Citation extraction validated
- [x] Error scenarios covered
- [x] `.env.example` updated with all variables
- [x] Documentation for running integration tests
- [x] `cargo test -p gorkd-llm` passes (unit tests)
- [x] `cargo test -p gorkd-llm -- --ignored` passes (with keys)
