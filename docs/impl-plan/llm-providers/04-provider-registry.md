# Phase 4: Provider Registry

**Status**: Complete
**Duration**: ~1 day
**Depends on**: Phase 2, Phase 3

## Overview

Create a provider registry that manages multiple LLM providers and enables model selection at runtime. This allows the pipeline to use different models for different tasks and provides automatic fallback when the primary provider fails.

## Tasks

| Task | Description |
|------|-------------|
| Create registry struct | Hold multiple providers, track default |
| Implement provider lookup | Get provider by model ID |
| Add fallback logic | Try fallback on retryable errors |
| Create builder pattern | Fluent API for registry construction |
| Update AppState | Use registry instead of single provider |
| Update Pipeline | Support model selection per-request |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-llm/src/registry.rs` | **New file** - LlmRegistry implementation |
| `crates/gorkd-llm/src/lib.rs` | Export registry, update public API |
| `crates/gorkd-core/src/traits/llm.rs` | Add `provider_name()` to trait |
| `crates/gorkd-api/src/state.rs` | Use LlmRegistry instead of single provider |
| `crates/gorkd-api/src/main.rs` | Initialize registry with configured providers |

## Registry Design

```
LlmRegistry
├── providers: HashMap<ModelId, Arc<dyn LlmProvider>>
├── default_model: ModelId
├── fallback_model: Option<ModelId>
│
├── get(&self, model_id) -> Option<Arc<dyn LlmProvider>>
├── default(&self) -> Arc<dyn LlmProvider>
├── synthesize_with_fallback(...) -> Result<ResearchAnswer, LlmError>
└── available_models(&self) -> Vec<ModelId>
```

## Fallback Behavior

```
Request → Primary Provider
           │
           ├─ Success → Return answer
           │
           └─ Error (retryable) → Fallback Provider
                                    │
                                    ├─ Success → Return answer
                                    │
                                    └─ Error → Return error
```

**Retryable errors** (trigger fallback):
- `LlmError::RateLimited`
- `LlmError::ModelUnavailable`
- `LlmError::Timeout`
- `LlmError::Network`

**Non-retryable errors** (fail immediately):
- `LlmError::ContentFiltered`
- `LlmError::ContextLengthExceeded`

## Key Considerations

- Registry should be `Clone` + `Send` + `Sync` for use in Axum state
- Model IDs should be validated at registration time
- Fallback should log when triggered (for observability)
- Consider adding metrics for fallback rate
- Keep backward compatibility with single-provider API

## Deliverables

- [x] LlmRegistry struct with provider management
- [x] Model lookup by ID
- [x] Automatic fallback on retryable errors
- [x] Builder pattern for easy construction
- [x] AppState updated to use registry
- [x] API can specify model (optional parameter)
- [x] Unit tests for fallback logic
- [x] `cargo test -p gorkd-llm` passes
- [x] `cargo test -p gorkd-api` passes
