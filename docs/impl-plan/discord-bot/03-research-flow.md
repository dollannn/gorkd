# Phase 3: Research Flow

**Status**: Planned
**Duration**: ~1-2 days
**Depends on**: Phase 2

## Overview

Connect event handlers to the gorkd API. When a query is received, create a research job, poll for progress updates, and edit the Discord message as the job progresses through stages. Handle completion, errors, and edge cases.

## Tasks

| Task | Description |
|------|-------------|
| Job creation | POST to /v1/research, get job_id |
| Progress polling | GET /v1/jobs/:id every 2 seconds |
| Message updates | Edit message with current status |
| Completion handling | Format final answer into embed |
| Error handling | Map API errors to user-friendly messages |
| Timeout handling | Cancel polling after 60 seconds |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-bot-discord/src/research.rs` | **New file** - ResearchManager |
| `crates/gorkd-bot-discord/src/api.rs` | Add create_research, get_job, get_sources methods |
| `crates/gorkd-bot-discord/src/handler.rs` | Call ResearchManager from event handlers |
| `crates/gorkd-bot-discord/src/dto.rs` | **New file** - Mirror API DTOs |

## Implementation Details

### Research Flow Sequence

```
1. User triggers query
2. Send initial message: "Researching..."
3. POST /v1/research { query }
4. Store message_id -> job_id mapping
5. Start polling loop:
   a. GET /v1/jobs/{job_id}
   b. If status changed, edit message
   c. If terminal (completed/failed), break
   d. Sleep 2 seconds
6. Fetch sources if completed
7. Format final embed
8. Edit message with result
9. Remove from active_jobs map
```

### Status to Message Mapping

| JobStatus | Discord Message |
|-----------|-----------------|
| pending | "Researching your question..." |
| planning | "Researching your question..." |
| searching | "Searching sources..." |
| fetching | "Searching sources... Found N results" |
| synthesizing | "Analyzing sources..." |
| completed | [Final embed with answer] |
| failed | [Error embed with message] |

### ResearchManager API

```
ResearchManager
├── new(api_client, http) -> Self
├── start_research(ctx, query, message_id) -> Result<()>
│   └── Spawns async task, returns immediately
├── poll_job(job_id, message_id) -> JoinHandle
│   └── Background task that polls and updates
└── format_progress(status, sources_count) -> String
```

### DTO Types (mirror gorkd-api)

```rust
struct CreateResearchRequest { query: String }
struct CreateResearchResponse { job_id: String, status: String }
struct JobResponse { 
    id: String, 
    status: String, 
    answer: Option<AnswerResponse>,
    error_message: Option<String>
}
struct AnswerResponse {
    summary: String,
    confidence: String,
    citations: Vec<CitationResponse>
}
```

## Key Considerations

- Spawn polling as background task to not block event loop
- Track active jobs to prevent duplicate polling
- Respect Discord rate limits (5 edits per 5 seconds per channel)
- If bot restarts mid-research, job continues in API but message won't update
- Consider exponential backoff if API is slow/unavailable

## Deliverables

- [ ] Research job created via API on query
- [ ] Message updates as job progresses
- [ ] Final answer displayed on completion
- [ ] Error message shown on failure
- [ ] Polling stops after 60 second timeout
- [ ] Multiple concurrent queries work correctly
- [ ] Bot handles API being temporarily unavailable
