# Phase 4: API Skeleton

**Status**: Planned
**Duration**: ~1-2 days
**Depends on**: Phase 3

## Overview

Implement Axum HTTP server with all endpoints from http-api.md. Handlers return stubbed responses initially, wired to mock providers.

## Tasks

| Task | Description |
|------|-------------|
| Setup Axum app | Router, state, middleware |
| Implement POST /v1/research | Create job, return job_id |
| Implement GET /v1/jobs/:id | Return job status/result |
| Implement GET /v1/jobs/:id/stream | SSE endpoint (stubbed) |
| Implement GET /v1/jobs/:id/sources | Return sources for job |
| Implement GET /health | Health check endpoint |
| Add error handling | Structured error responses |
| Add request validation | Query length, format |
| Add app state | Provider instances, config |
| Add basic logging | tracing + tracing-subscriber |
| Add graceful shutdown | SIGTERM handling |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-api/src/main.rs` | Server bootstrap, signal handling |
| `crates/gorkd-api/src/lib.rs` | **New file** - App construction |
| `crates/gorkd-api/src/routes/mod.rs` | **New file** - Router setup |
| `crates/gorkd-api/src/routes/research.rs` | **New file** - POST /research handler |
| `crates/gorkd-api/src/routes/jobs.rs` | **New file** - GET /jobs handlers |
| `crates/gorkd-api/src/routes/health.rs` | **New file** - Health endpoint |
| `crates/gorkd-api/src/error.rs` | **New file** - API error types |
| `crates/gorkd-api/src/state.rs` | **New file** - AppState definition |
| `crates/gorkd-api/src/dto.rs` | **New file** - Request/response DTOs |
| `crates/gorkd-api/Cargo.toml` | Add dependencies |

## Endpoint Specifications

### POST /v1/research
- Input: `{ "query": "..." }`
- Validate: non-empty, < 2000 chars
- Create ResearchJob with mock store
- Return: `{ "job_id": "job_xxx", "status": "pending", "stream_url": "..." }`

### GET /v1/jobs/:id
- Lookup job in store
- 404 if not found
- Return full job state (stubbed answer if completed)

### GET /v1/jobs/:id/stream
- Return SSE stream
- For now: single "complete" event after 1 second delay
- Real implementation in later phase

### GET /health
- Return: `{ "status": "healthy", "version": "0.1.0" }`

## App State Structure

```rust
pub struct AppState {
    pub search: Arc<dyn SearchProvider>,
    pub llm: Arc<dyn LlmProvider>,
    pub store: Arc<dyn Store>,
    pub config: Config,
}
```

## Key Considerations

- Use `axum::extract::State<Arc<AppState>>` pattern
- JSON errors must match spec format: `{ "error": { "code": "...", "message": "..." } }`
- Add `X-Request-Id` header for tracing
- Bind to `0.0.0.0:4000` by default, configurable via `PORT` env

## Deliverables

- [ ] `cargo run -p gorkd-api` starts server on port 4000
- [ ] `curl localhost:4000/health` returns healthy
- [ ] `curl -X POST localhost:4000/v1/research -d '{"query":"test"}'` returns job_id
- [ ] `curl localhost:4000/v1/jobs/job_xxx` returns job (or 404)
- [ ] Invalid requests return proper error format
- [ ] Logs show request/response info
