# Phase 4: API Skeleton

**Status**: Planned
**Duration**: ~1-2 days
**Depends on**: Phase 3

## Overview

Implement Axum HTTP server with all endpoints from http-api.md. Handlers return stubbed responses initially, wired to mock providers. Uses utoipa for automatic OpenAPI spec generation with Swagger UI.

## Tasks

| Task | Description |
|------|-------------|
| Setup Axum app | Router, state, middleware |
| Setup utoipa OpenAPI | Configure OpenApiDoc, info, tags |
| Add Scalar UI | Mount at `/docs`, spec at `/api-doc/openapi.json` |
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
| `crates/gorkd-api/src/lib.rs` | **New file** - App construction, OpenAPI setup |
| `crates/gorkd-api/src/routes/mod.rs` | **New file** - Router setup |
| `crates/gorkd-api/src/routes/research.rs` | **New file** - POST /research handler |
| `crates/gorkd-api/src/routes/jobs.rs` | **New file** - GET /jobs handlers |
| `crates/gorkd-api/src/routes/health.rs` | **New file** - Health endpoint |
| `crates/gorkd-api/src/error.rs` | **New file** - API error types with ToSchema |
| `crates/gorkd-api/src/state.rs` | **New file** - AppState definition |
| `crates/gorkd-api/src/dto.rs` | **New file** - Request/response DTOs with ToSchema |
| `crates/gorkd-api/src/openapi.rs` | **New file** - OpenAPI configuration |
| `crates/gorkd-api/Cargo.toml` | Add dependencies (incl. utoipa, utoipa-axum, utoipa-swagger-ui) |

## Dependencies

```toml
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# OpenAPI
utoipa = { version = "5", features = ["axum_extras"] }
utoipa-axum = "0.2"
utoipa-scalar = { version = "0.3", features = ["axum"] }
```

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

### GET /docs (auto-generated)
- Scalar UI for interactive API exploration
- OpenAPI spec available at `/api-doc/openapi.json`

## App State Structure

```rust
pub struct AppState {
    pub search: Arc<dyn SearchProvider>,
    pub llm: Arc<dyn LlmProvider>,
    pub store: Arc<dyn Store>,
    pub config: Config,
}
```

## OpenAPI Configuration

```rust
// openapi.rs
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "gorkd API",
        version = "0.1.0",
        description = "Truth-seeking research bot API",
        license(name = "MIT")
    ),
    tags(
        (name = "research", description = "Research operations"),
        (name = "jobs", description = "Job management"),
        (name = "health", description = "Health checks")
    )
)]
pub struct ApiDoc;
```

## DTO Examples with ToSchema

```rust
// dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateResearchRequest {
    /// The research query (1-2000 characters)
    #[schema(example = "What caused the 2024 CrowdStrike outage?", min_length = 1, max_length = 2000)]
    pub query: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateResearchResponse {
    /// Unique job identifier
    #[schema(example = "job_abc123")]
    pub job_id: String,
    /// Current job status
    pub status: JobStatus,
    /// URL for SSE streaming updates
    #[schema(example = "/v1/jobs/job_abc123/stream")]
    pub stream_url: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Searching,
    Synthesizing,
    Completed,
    Failed,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiError {
    pub error: ApiErrorBody,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiErrorBody {
    /// Error code (e.g., "validation_error", "not_found")
    #[schema(example = "validation_error")]
    pub code: String,
    /// Human-readable error message
    #[schema(example = "Query cannot be empty")]
    pub message: String,
}
```

## Handler Example with utoipa

```rust
// routes/research.rs
use axum::{extract::State, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

/// Create a new research job
#[utoipa::path(
    post,
    path = "/v1/research",
    tag = "research",
    request_body = CreateResearchRequest,
    responses(
        (status = 201, description = "Job created", body = CreateResearchResponse),
        (status = 400, description = "Invalid request", body = ApiError),
    )
)]
pub async fn create_research(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateResearchRequest>,
) -> Result<Json<CreateResearchResponse>, ApiError> {
    // ...
}

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new().routes(routes!(create_research))
}
```

## Router Setup with utoipa-axum

```rust
// lib.rs
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

pub fn app(state: Arc<AppState>) -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(routes::research::router())
        .merge(routes::jobs::router())
        .merge(routes::health::router())
        .split_for_parts();

    router
        .merge(Scalar::with_url("/docs", api))
        .with_state(state)
}
```

## Key Considerations

- Use `axum::extract::State<Arc<AppState>>` pattern
- JSON errors must match spec format: `{ "error": { "code": "...", "message": "..." } }`
- Add `X-Request-Id` header for tracing
- Bind to `0.0.0.0:4000` by default, configurable via `PORT` env
- All DTOs derive `ToSchema` for OpenAPI spec generation
- Use `utoipa_axum::routes!` macro to auto-register paths
- Use `#[schema(...)]` attributes for examples and constraints

## Deliverables

- [ ] `cargo run -p gorkd-api` starts server on port 4000
- [ ] `curl localhost:4000/health` returns healthy
- [ ] `curl -X POST localhost:4000/v1/research -d '{"query":"test"}'` returns job_id
- [ ] `curl localhost:4000/v1/jobs/job_xxx` returns job (or 404)
- [ ] Invalid requests return proper error format
- [ ] Logs show request/response info
- [ ] `curl localhost:4000/docs/openapi.json` returns valid OpenAPI 3.1 spec
- [ ] Scalar UI accessible at `http://localhost:4000/docs`
- [ ] All endpoints documented with request/response schemas
