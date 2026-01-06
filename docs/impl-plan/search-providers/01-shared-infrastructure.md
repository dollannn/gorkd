# Phase 1: Shared Infrastructure

**Status**: Planned
**Duration**: ~1 day
**Depends on**: None

## Overview

Set up shared configuration, HTTP client wrapper, and provider registry that all search providers will use. This foundation enables consistent error handling, timeout management, and runtime provider selection.

## Tasks

| Task | Description |
|------|-------------|
| Config module | Environment-based configuration for API keys and settings |
| HTTP client wrapper | Shared reqwest client with timeouts and user-agent |
| Provider registry | HashMap-based registry for runtime provider lookup |
| Re-export trait | Re-export SearchProvider from gorkd-search for convenience |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-search/src/lib.rs` | Add modules, re-exports, registry |
| `crates/gorkd-search/src/config.rs` | **New file** - SearchConfig struct |
| `crates/gorkd-search/src/client.rs` | **New file** - Shared HTTP client |
| `crates/gorkd-search/src/registry.rs` | **New file** - ProviderRegistry |
| `crates/gorkd-search/Cargo.toml` | Add url crate dependency |

## Implementation Details

### Config Structure

```
SearchConfig
├── tavily_api_key: Option<String>
├── exa_api_key: Option<String>
├── searxng_url: Option<String>
├── timeout_secs: u64
└── max_results: usize
```

### Provider Registry

```
ProviderRegistry
├── providers: HashMap<String, Arc<dyn SearchProvider>>
├── register(id, provider)
├── get(id) -> Option<Arc<dyn SearchProvider>>
├── list() -> Vec<String>
└── from_config(SearchConfig) -> Self  // Auto-register available providers
```

## Key Considerations

- Config should fail fast if no providers are configured
- HTTP client should set reasonable timeouts (30s default)
- User-Agent header should identify gorkd for API providers
- Registry should be thread-safe (uses Arc internally)

## Deliverables

- [ ] SearchConfig loads from environment
- [ ] Shared HTTP client with configurable timeout
- [ ] ProviderRegistry with register/get/list
- [ ] Unit tests for config parsing
- [ ] `cargo build -p gorkd-search` passes
