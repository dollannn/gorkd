# Phase 4: Query Flow

**Status**: Planned
**Duration**: ~1-1.5 days
**Depends on**: Phase 2 (API Client), Phase 3 (Core Layout)

## Overview

Build the query input experience and research state management. User enters query, submits, transitions to researching state.

## Tasks

| Task | Description |
|------|-------------|
| Create research store | State machine for research flow |
| Build QueryInput component | Textarea with character count, submit |
| Build ExampleQueries component | Clickable example queries |
| Build home page | Query-focused landing |
| Handle form submission | Validate, call API, transition state |
| Build ResearchingView | Placeholder for progress (Phase 5) |
| Add URL state | Job ID in URL for shareability |
| Handle navigation | Direct link to job works |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `web/src/lib/stores/research.svelte.ts` | **New file** - Research state machine |
| `web/src/lib/components/QueryInput.svelte` | **New file** - Main query input |
| `web/src/lib/components/ExampleQueries.svelte` | **New file** - Suggested queries |
| `web/src/routes/+page.svelte` | **Modify** - Home page with query input |
| `web/src/routes/research/[id]/+page.svelte` | **New file** - Research job page |
| `web/src/routes/research/[id]/+page.ts` | **New file** - Load function |

## State Machine

```
States:
  idle        -> User hasn't started
  submitting  -> API call in progress
  streaming   -> SSE connection active (Phase 5)
  completed   -> Results ready
  error       -> Something failed

Transitions:
  idle       --submit-->     submitting
  submitting --success-->    streaming
  submitting --failure-->    error
  streaming  --complete-->   completed
  streaming  --failure-->    error
  error      --retry-->      submitting
  completed  --new_query-->  idle
```

## Research Store API

```typescript
// Usage
const research = createResearchStore()

// State
research.state      // 'idle' | 'submitting' | ...
research.job        // ResearchJob | null
research.error      // ApiError | null

// Actions
research.submit(query)
research.retry()
research.reset()
```

## Query Input UX

- Textarea (not input) - multi-line queries
- Character count (max 2000)
- Submit on Ctrl/Cmd + Enter
- Submit button disabled when empty
- Clear button when has content
- Focus on page load

## Example Queries

```typescript
const examples = [
  "What caused the 2024 CrowdStrike outage?",
  "Is coffee good or bad for health?",
  "How does mRNA vaccine technology work?",
  "What are the latest developments in nuclear fusion?",
]
```

## URL Structure

```
/                    - Home (query input)
/research/[job_id]   - Research job (progress or results)
```

## Key Considerations

- Debounce submission to prevent double-submit
- Preserve query in URL params for refresh
- Handle job not found (404 from API)
- Show loading state during initial job fetch
- Query input should remain accessible from results page

## Deliverables

- [ ] Query input accepts and validates input
- [ ] Character count displays correctly
- [ ] Ctrl+Enter submits form
- [ ] Example queries populate input on click
- [ ] Submission calls API and transitions state
- [ ] Job page loads job from URL parameter
- [ ] Error state displays with retry option
- [ ] State persists across page navigation
