# HTTP API

RESTful API for research operations. Built with Axum.

## Base URL

```
http://localhost:4000/v1
```

## Authentication

**Initial**: None (local development)
**Future**: API key via `Authorization: Bearer <key>` header

## Endpoints

### POST /research

Start a new research job.

**Request**
```json
{
  "query": "What caused the 2024 CrowdStrike outage?"
}
```

**Response** `202 Accepted`
```json
{
  "job_id": "job_abc123xyz",
  "status": "pending",
  "stream_url": "/v1/jobs/job_abc123xyz/stream"
}
```

**Errors**
- `400` - Invalid query (empty, too long, malformed)
- `429` - Rate limited
- `500` - Internal error

---

### GET /jobs/:id

Get job status and results.

**Response** `200 OK` (completed)
```json
{
  "job_id": "job_abc123xyz",
  "status": "completed",
  "query": "What caused the 2024 CrowdStrike outage?",
  "answer": {
    "summary": "The outage was caused by a faulty update to CrowdStrike's Falcon sensor software...",
    "detail": "On July 19, 2024, CrowdStrike released a content update...",
    "confidence": "high",
    "limitations": [
      "Technical details still being investigated",
      "Full impact assessment ongoing"
    ]
  },
  "citations": [
    {
      "claim": "The outage affected approximately 8.5 million Windows devices",
      "source_id": "src_001",
      "quote": "Microsoft estimates that 8.5 million Windows devices were affected"
    }
  ],
  "sources": [
    {
      "id": "src_001",
      "url": "https://blogs.microsoft.com/...",
      "title": "Helping our customers through the CrowdStrike outage",
      "domain": "microsoft.com",
      "published_at": "2024-07-20T00:00:00Z"
    }
  ],
  "metadata": {
    "created_at": "2024-07-25T10:30:00Z",
    "completed_at": "2024-07-25T10:30:14Z",
    "duration_ms": 14230,
    "sources_considered": 12,
    "cached": false
  }
}
```

**Response** `200 OK` (pending)
```json
{
  "job_id": "job_abc123xyz",
  "status": "searching",
  "query": "What caused the 2024 CrowdStrike outage?",
  "progress": {
    "stage": "search",
    "message": "Fetching sources...",
    "sources_found": 8
  }
}
```

**Errors**
- `404` - Job not found
- `500` - Internal error

---

### GET /jobs/:id/stream

Server-Sent Events stream for real-time updates.

**Event Types**

```
event: status
data: {"stage": "planning", "message": "Analyzing query..."}

event: status
data: {"stage": "searching", "message": "Searching Tavily...", "progress": 0.3}

event: source
data: {"id": "src_001", "url": "...", "title": "...", "relevance": 0.92}

event: status
data: {"stage": "synthesizing", "message": "Generating answer...", "sources_count": 8}

event: answer
data: {"summary": "...", "confidence": "high"}

event: complete
data: {"job_id": "job_abc123xyz", "duration_ms": 14230}
```

**Connection**
- Keep-alive: 30 seconds
- Timeout: 120 seconds
- Reconnect: Client should reconnect on disconnect

---

### GET /jobs/:id/sources

Get detailed source information for a job.

**Response** `200 OK`
```json
{
  "sources": [
    {
      "id": "src_001",
      "url": "https://...",
      "title": "...",
      "domain": "microsoft.com",
      "content_preview": "First 500 characters...",
      "published_at": "2024-07-20T00:00:00Z",
      "relevance_score": 0.92,
      "used_in_citations": true
    }
  ]
}
```

---

### GET /health

Health check endpoint.

**Response** `200 OK`
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "uptime_seconds": 3600
}
```

## Data Types

### JobStatus

```
pending     - Job created, not started
planning    - Analyzing query
searching   - Executing searches
fetching    - Retrieving page content
synthesizing - LLM generating answer
completed   - Done, results available
failed      - Error occurred
```

### Confidence

```
high        - Multiple corroborating authoritative sources
medium      - Good sources but limited corroboration
low         - Weak sources or conflicting information
insufficient - Cannot answer from available sources
```

## Error Format

All errors follow this structure:

```json
{
  "error": {
    "code": "INVALID_QUERY",
    "message": "Query cannot be empty",
    "details": {}
  }
}
```

### Error Codes

| Code | HTTP | Description |
|------|------|-------------|
| `INVALID_QUERY` | 400 | Query validation failed |
| `JOB_NOT_FOUND` | 404 | Job ID doesn't exist |
| `RATE_LIMITED` | 429 | Too many requests |
| `SEARCH_FAILED` | 502 | Search providers unavailable |
| `LLM_FAILED` | 502 | LLM provider error |
| `INTERNAL_ERROR` | 500 | Unexpected error |

## Rate Limits

| Endpoint | Limit |
|----------|-------|
| POST /research | 10/minute per IP |
| GET /jobs/* | 60/minute per IP |
| GET /stream | 5 concurrent per IP |

Headers included in response:
```
X-RateLimit-Limit: 10
X-RateLimit-Remaining: 7
X-RateLimit-Reset: 1627849200
```

## Conventions

- All timestamps in ISO 8601 UTC
- All IDs are prefixed (`job_`, `src_`)
- Pagination via `?cursor=` (when implemented)
- Request bodies are JSON (`Content-Type: application/json`)
- UTF-8 encoding throughout
