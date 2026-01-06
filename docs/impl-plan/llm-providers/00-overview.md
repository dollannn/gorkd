# LLM Providers - Implementation Plan

**Status**: Planning
**Total Duration**: ~5-7 days
**Priority**: High

## Overview

Implement real LLM provider integrations for gorkd. The system needs to transform search results into cited research answers using Claude Sonnet 4.5 as the primary model, with OpenAI as a fallback option. This enables the core "Search → Synthesize → Cite" workflow with production-quality LLM backends.

## Current State

| Component | Status | Notes |
|-----------|--------|-------|
| `LlmProvider` trait | Done | Single `synthesize` method in gorkd-core |
| `LlmError` enum | Done | Good coverage with `is_retryable()` |
| `MockLlmProvider` | Done | Works for testing |
| `gorkd-llm` crate | Stub | Empty `anthropic.rs` and `openai.rs` |
| Multi-provider support | Missing | Pipeline uses single provider |
| Provider registry | Missing | No way to switch models at runtime |
| Streaming | Missing | Trait has flag but no implementation |

## Phases

| Phase | Description | Duration |
|-------|-------------|----------|
| [1. Shared Infrastructure](./01-shared-infrastructure.md) | Config, HTTP client, common types | 1 day |
| [2. Anthropic Provider](./02-anthropic-provider.md) | Claude Sonnet 4.5 integration (primary) | 1-2 days |
| [3. OpenAI Provider](./03-openai-provider.md) | GPT-4o integration (fallback) | 1-2 days |
| [4. Provider Registry](./04-provider-registry.md) | Multi-model support, routing, fallback | 1 day |
| [5. Integration Testing](./05-integration-testing.md) | E2E tests, real API validation | 1 day |

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Primary model | Claude Sonnet 4.5 | Best quality for research synthesis, user requirement |
| Fallback model | GPT-4o | Reliable alternative when Anthropic unavailable |
| HTTP client | reqwest | Already in workspace, async, well-maintained |
| Config | Environment vars | Standard, works with docker-compose |
| Structured output | JSON mode | Required for reliable citation extraction |
| Streaming | Deferred to later | MVP first, streaming adds complexity |
| Model selection | Registry pattern | Allows runtime model switching, extensible |
| Prompt format | Per-provider | Each provider has optimal prompt structure |

## Model Configurations

| Model ID | Provider | Context Window | Use Case |
|----------|----------|----------------|----------|
| `claude-sonnet-4-5` | Anthropic | 200K tokens | Primary synthesis |
| `claude-haiku-3-5` | Anthropic | 200K tokens | Future: query expansion |
| `gpt-4o` | OpenAI | 128K tokens | Fallback synthesis |
| `gpt-4o-mini` | OpenAI | 128K tokens | Future: cheap tasks |

## Environment Variables

```bash
# Required (at least one)
ANTHROPIC_API_KEY=sk-ant-xxxxx
OPENAI_API_KEY=sk-xxxxx

# Optional configuration
LLM_DEFAULT_MODEL=claude-sonnet-4-5
LLM_FALLBACK_MODEL=gpt-4o
LLM_TIMEOUT_SECS=60
LLM_MAX_RETRIES=2
```

## Dependencies

- API keys: `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`
- No database required for this phase
- Search providers implementation (for E2E testing)

## Related Files

- `crates/gorkd-core/src/traits/llm.rs` - LlmProvider trait
- `crates/gorkd-core/src/traits/errors.rs` - LlmError enum
- `crates/gorkd-core/src/mock/llm.rs` - MockLlmProvider (reference)
- `crates/gorkd-core/src/answer.rs` - ResearchAnswer, Citation types
- `crates/gorkd-llm/src/` - Implementation target
- `crates/gorkd-api/src/state.rs` - Provider injection point
