# Phase 5: Integration Testing

**Status**: Planned
**Duration**: ~1 day
**Depends on**: Phases 2-4

## Overview

Wire up real providers to the API server, implement provider selection logic, add fallback behavior, and create end-to-end integration tests that verify the complete research flow with real search results.

## Tasks

| Task | Description |
|------|-------------|
| API server wiring | Replace mock provider with real providers |
| Provider selection | Select provider based on config and query |
| Fallback logic | Try next provider on failure |
| E2E tests | Test full research flow with real APIs |
| Documentation | Update README with setup instructions |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-api/src/main.rs` | Initialize real providers from config |
| `crates/gorkd-api/src/state.rs` | Support multiple providers via registry |
| `crates/gorkd-core/src/pipeline/executor.rs` | Add fallback logic |
| `crates/gorkd-api/tests/integration.rs` | Add real provider tests |
| `.env.example` | Add search provider env vars |
| `README.md` | Update configuration section |

## Provider Selection Logic

```
1. Check query content_type
   - News → prefer Tavily (topic: news)
   - Academic → prefer Exa (neural search)
   - General → use default

2. Check configured providers
   - Use first available from: [Tavily, Exa, SearXNG]

3. On failure
   - If retryable error → retry same provider (1x)
   - If non-retryable → try next provider
   - If all fail → return error
```

## Fallback Behavior

| Primary Failure | Fallback To |
|-----------------|-------------|
| Tavily rate limited | Exa |
| Exa unavailable | SearXNG |
| SearXNG timeout | Tavily (retry) |
| All providers fail | Return error to user |

## E2E Test Scenarios

| Test | Description |
|------|-------------|
| Basic search | Query → real results → sources returned |
| Filter application | Recency filter produces recent results |
| Provider fallback | Mock primary failure → fallback works |
| No providers | Graceful error when nothing configured |

## Environment Setup for Tests

```bash
# For integration tests (optional)
export TAVILY_API_KEY=tvly-test-xxx
export EXA_API_KEY=exa-test-xxx
export SEARXNG_URL=https://searx.be

# Run integration tests
cargo test -p gorkd-api --features integration
```

## Key Considerations

- Integration tests should be feature-gated to not require API keys in CI
- Mock provider should remain default for `cargo run` without config
- Fallback should log which provider is being tried
- Provider selection should be observable (include in job metadata)

## Deliverables

- [ ] API server initializes providers from environment
- [ ] Provider registry integrated into AppState
- [ ] Executor supports provider fallback
- [ ] E2E tests pass with real providers
- [ ] E2E tests pass with mock (no API keys)
- [ ] `.env.example` documents all search config
- [ ] README updated with provider setup
- [ ] `cargo test` passes (unit tests, no API keys)
- [ ] `cargo test --features integration` passes (with API keys)
