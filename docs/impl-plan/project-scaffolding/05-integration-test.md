# Phase 5: Integration Smoke Test

**Status**: Planned
**Duration**: ~1 day
**Depends on**: Phase 4

## Overview

Wire everything together: submit query via API, mock providers execute pipeline, return synthesized result. Verify the full flow works end-to-end with tests.

## Tasks

| Task | Description |
|------|-------------|
| Implement research pipeline | Orchestrate: plan -> search -> synthesize |
| Wire API to pipeline | POST /research triggers full flow |
| Add pipeline module to core | Stateless orchestration logic |
| Update job status | Progress through states during execution |
| Add integration tests | Full HTTP request/response cycle |
| Add docker-compose.yml | Postgres stub for future (optional) |
| Update README | Quickstart instructions that work |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-core/src/pipeline/mod.rs` | **New file** - Pipeline orchestrator |
| `crates/gorkd-core/src/pipeline/planner.rs` | **New file** - Query planning logic |
| `crates/gorkd-core/src/pipeline/executor.rs` | **New file** - Search execution |
| `crates/gorkd-core/src/pipeline/synthesizer.rs` | **New file** - Synthesis coordination |
| `crates/gorkd-api/src/routes/research.rs` | Wire to pipeline |
| `crates/gorkd-api/tests/integration.rs` | **New file** - API integration tests |
| `docker-compose.yml` | **New file** - Local infrastructure |
| `README.md` | Update quickstart section |

## Pipeline Flow

```
1. API receives query
2. Create ResearchJob (status: pending)
3. Pipeline.run(job, providers):
   a. Planner: generate search queries (status: planning)
   b. Executor: call search provider (status: searching)
   c. Synthesizer: call LLM provider (status: synthesizing)
   d. Update job with answer (status: completed)
4. Return job_id
5. Client polls GET /jobs/:id for result
```

## Integration Test Scenarios

| Test | Description |
|------|-------------|
| happy_path | Submit query, poll until complete, verify answer structure |
| invalid_query_empty | Empty query returns 400 |
| invalid_query_too_long | >2000 char query returns 400 |
| job_not_found | Unknown job_id returns 404 |
| health_check | /health returns 200 with version |

## Key Considerations

- Pipeline runs synchronously in handler for now (async background job later)
- Mock providers return instantly (no artificial delays in tests)
- Integration tests use `axum::test` helpers
- Keep pipeline logic in gorkd-core (no HTTP concerns)

## Test Structure

```rust
#[tokio::test]
async fn test_research_happy_path() {
    let app = create_test_app().await;
    
    // Submit research request
    let response = app
        .post("/v1/research")
        .json(&json!({"query": "What is Rust?"}))
        .await;
    
    assert_eq!(response.status(), 202);
    let body: Value = response.json().await;
    let job_id = body["job_id"].as_str().unwrap();
    
    // Poll for completion
    let response = app.get(&format!("/v1/jobs/{}", job_id)).await;
    assert_eq!(response.status(), 200);
    
    let job: Value = response.json().await;
    assert_eq!(job["status"], "completed");
    assert!(job["answer"]["summary"].as_str().is_some());
}
```

## Deliverables

- [ ] Full query -> answer flow works via API
- [ ] `cargo test -p gorkd-api` integration tests pass
- [ ] Job progresses through all status states
- [ ] Mock providers are easily swappable
- [ ] README quickstart works for new developer
- [ ] `cargo build --release` succeeds
