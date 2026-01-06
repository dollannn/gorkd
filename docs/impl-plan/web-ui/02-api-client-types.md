# Phase 2: API Client & Types

**Status**: Done
**Duration**: ~0.5-1 day
**Depends on**: Phase 1 (Project Scaffolding)

## Overview

Create TypeScript types mirroring the Rust API and a type-safe fetch client. All API interactions go through this layer.

## Tasks

| Task | Description |
|------|-------------|
| Define API types | ResearchJob, Source, Citation, Answer, etc. |
| Define error types | ApiError with code/message structure |
| Create fetch client | Base client with error handling |
| Add research endpoints | POST /research, GET /jobs/:id |
| Add sources endpoint | GET /jobs/:id/sources |
| Add health endpoint | GET /health |
| Add request/response logging | Dev-only console logging |
| Write unit tests | Client error handling tests |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `web/src/lib/api/types.ts` | **New file** - All API types |
| `web/src/lib/api/client.ts` | **New file** - Fetch client |
| `web/src/lib/api/errors.ts` | **New file** - Error types and handling |
| `web/src/lib/api/index.ts` | **New file** - Public exports |
| `web/src/lib/api/client.test.ts` | **New file** - Client tests |

## Type Definitions

### Core Types (mirror Rust)

```
JobStatus: 'pending' | 'planning' | 'searching' | 'fetching' | 'synthesizing' | 'completed' | 'failed'
Confidence: 'high' | 'medium' | 'low' | 'insufficient'
```

### Request/Response Types

```
ResearchRequest: { query: string }
ResearchResponse: { job_id, status, stream_url }
JobResponse: { job_id, status, query, answer?, citations?, sources?, metadata? }
SourcesResponse: { sources: Source[] }
HealthResponse: { status, version, uptime_seconds }
```

### Error Types

```
ApiError: { code: string, message: string, details?: Record<string, unknown> }
ApiErrorCode: 'INVALID_QUERY' | 'JOB_NOT_FOUND' | 'RATE_LIMITED' | 'SEARCH_FAILED' | 'LLM_FAILED' | 'INTERNAL_ERROR'
```

## Client API

```typescript
// Usage pattern
const client = createApiClient({ baseUrl: PUBLIC_API_URL })

const { job_id } = await client.startResearch({ query: '...' })
const job = await client.getJob(job_id)
const sources = await client.getSources(job_id)
```

## Key Considerations

- All timestamps parsed as Date objects
- Client throws typed ApiError on failure
- Request timeout: 30s default
- No retry logic in client (handled at call site)
- Zod for runtime validation (optional, adds safety)

## Deliverables

- [x] All API types defined and exported
- [x] Client can start research job
- [x] Client can fetch job status
- [x] Client can fetch sources
- [x] Error responses parsed into ApiError
- [x] Unit tests pass for error handling
- [x] Types match `docs/interfaces/http-api.md` exactly
