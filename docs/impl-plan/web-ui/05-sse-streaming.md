# Phase 5: SSE Streaming

**Status**: Planned
**Duration**: ~1 day
**Depends on**: Phase 4 (Query Flow)

## Overview

Connect to the SSE stream for real-time research progress. Display pipeline stages, incoming sources, and synthesis status as they happen.

## Tasks

| Task | Description |
|------|-------------|
| Create SSE client | EventSource wrapper with reconnection |
| Parse SSE events | Handle status, source, answer, complete events |
| Integrate with research store | Update state from stream events |
| Build ProgressStages component | Visual pipeline progress |
| Build SourcePreview component | Source appearing in real-time |
| Build StreamingIndicator | Active connection indicator |
| Handle reconnection | Auto-reconnect on disconnect |
| Handle stream errors | Display error, offer retry |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `web/src/lib/api/stream.ts` | **New file** - SSE client |
| `web/src/lib/stores/research.svelte.ts` | **Modify** - Add streaming state |
| `web/src/lib/components/ProgressStages.svelte` | **New file** - Pipeline visualization |
| `web/src/lib/components/SourcePreview.svelte` | **New file** - Incoming source card |
| `web/src/lib/components/StreamingIndicator.svelte` | **New file** - Connection status |
| `web/src/routes/research/[id]/+page.svelte` | **Modify** - Add streaming UI |

## SSE Event Types

From `docs/interfaces/http-api.md`:

```typescript
type SSEEvent =
  | { type: 'status'; data: { stage: string; message: string; progress?: number } }
  | { type: 'source'; data: { id: string; url: string; title: string; relevance: number } }
  | { type: 'answer'; data: { summary: string; confidence: string } }
  | { type: 'complete'; data: { job_id: string; duration_ms: number } }
```

## SSE Client API

```typescript
const stream = createResearchStream(jobId, {
  onStatus: (status) => { /* update progress */ },
  onSource: (source) => { /* add to sources list */ },
  onAnswer: (answer) => { /* show answer preview */ },
  onComplete: (meta) => { /* transition to completed */ },
  onError: (error) => { /* handle error */ },
})

stream.connect()
stream.disconnect()
```

## Progress Stages UI

```
[Planning]  ->  [Searching]  ->  [Synthesizing]  ->  [Complete]
   [x]            [~]               [ ]               [ ]
              "Searching Tavily..."
                   ████░░░░ 40%
```

Visual states per stage:
- Pending: Gray, empty circle
- Active: Blue, animated pulse
- Complete: Green, checkmark

## Source Preview Card

As sources arrive, show:
- Favicon + domain
- Title (truncated)
- Relevance score (if high: badge)
- Animate in (fade + slide)

## Key Considerations

- EventSource reconnects automatically (browser built-in)
- Add manual reconnect button if auto-reconnect fails 3x
- Buffer sources during rapid arrival (batch UI updates)
- Show connection status indicator (connected/reconnecting)
- Clean up EventSource on component destroy
- Handle case where stream completes before page loads (fetch final state)

## Reconnection Logic

```typescript
const MAX_RETRIES = 3
const RETRY_DELAYS = [1000, 2000, 5000] // ms

// On disconnect:
// 1. Show "Reconnecting..." indicator
// 2. Attempt reconnect after delay
// 3. After MAX_RETRIES, show error with manual retry button
```

## Deliverables

- [ ] SSE client connects to stream endpoint
- [ ] Status events update progress stages
- [ ] Source events add cards in real-time
- [ ] Answer event shows summary preview
- [ ] Complete event transitions to results
- [ ] Reconnection works on disconnect
- [ ] Error state shown after max retries
- [ ] Stream cleaned up on navigation away
