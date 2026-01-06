# Phase 3: Provider Traits

**Status**: Planned
**Duration**: ~1 day
**Depends on**: Phase 2

## Overview

Define async traits in gorkd-core for external integrations. Implement mock versions for testing. Real implementations live in gorkd-search, gorkd-llm, gorkd-store.

## Tasks

| Task | Description |
|------|-------------|
| Define SearchProvider trait | `search(query) -> Vec<SearchResult>` |
| Define LlmProvider trait | `synthesize(sources, query) -> Answer` |
| Define Store trait | Job CRUD, source storage, cache operations |
| Define provider error types | SearchError, LlmError, StoreError |
| Implement MockSearchProvider | Returns canned results |
| Implement MockLlmProvider | Returns templated answers |
| Implement MockStore | In-memory HashMap storage |
| Add trait tests | Verify mock implementations |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-core/src/lib.rs` | Add trait module exports |
| `crates/gorkd-core/src/traits/mod.rs` | **New file** - Module declarations |
| `crates/gorkd-core/src/traits/search.rs` | **New file** - SearchProvider trait |
| `crates/gorkd-core/src/traits/llm.rs` | **New file** - LlmProvider trait |
| `crates/gorkd-core/src/traits/store.rs` | **New file** - Store trait |
| `crates/gorkd-core/src/mock/mod.rs` | **New file** - Mock module |
| `crates/gorkd-core/src/mock/search.rs` | **New file** - MockSearchProvider |
| `crates/gorkd-core/src/mock/llm.rs` | **New file** - MockLlmProvider |
| `crates/gorkd-core/src/mock/store.rs` | **New file** - MockStore |

## Trait Definitions

### SearchProvider
```rust
#[async_trait]
pub trait SearchProvider: Send + Sync {
    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, SearchError>;
    fn provider_id(&self) -> &str;
}
```

### LlmProvider
```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn synthesize(
        &self,
        query: &str,
        sources: &[Source],
    ) -> Result<ResearchAnswer, LlmError>;
    fn model_id(&self) -> &str;
}
```

### Store
```rust
#[async_trait]
pub trait Store: Send + Sync {
    // Jobs
    async fn create_job(&self, job: &ResearchJob) -> Result<(), StoreError>;
    async fn get_job(&self, id: &JobId) -> Result<Option<ResearchJob>, StoreError>;
    async fn update_job(&self, job: &ResearchJob) -> Result<(), StoreError>;
    
    // Sources
    async fn store_sources(&self, job_id: &JobId, sources: &[Source]) -> Result<(), StoreError>;
    async fn get_sources(&self, job_id: &JobId) -> Result<Vec<Source>, StoreError>;
    
    // Cache
    async fn find_similar(&self, embedding: &[f32], threshold: f32) -> Result<Option<JobId>, StoreError>;
}
```

## Key Considerations

- Traits in gorkd-core use `async_trait` crate (stable async traits limited)
- Mock implementations should be deterministic for testing
- Error types should be convertible to API error responses
- Consider `#[cfg(test)]` vs separate mock module (prefer separate for reuse)

## Deliverables

- [ ] All three provider traits defined
- [ ] Mock implementations compile and pass tests
- [ ] Error types have proper `Display` and `Error` impls
- [ ] Traits are object-safe (can use `dyn Trait`)
- [ ] Documentation with usage examples
