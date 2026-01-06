# Web UI - Implementation Plan

**Status**: Planning
**Total Duration**: ~6-8 days
**Priority**: High

## Overview

Build the SvelteKit frontend for gorkd. A single-page research interface: query input, real-time progress via SSE, and rich results display with citations and sources. Focused, minimal, fast.

## Current State

| Component | Status | Notes |
|-----------|--------|-------|
| API endpoints | Done | POST /research, GET /jobs/:id, SSE streaming |
| API types | Done | ResearchJob, Source, Citation, Answer |
| web/ folder | Missing | No frontend exists |

## Phases

| Phase | Description | Duration |
|-------|-------------|----------|
| [1. Project Scaffolding](./01-project-scaffolding.md) | SvelteKit + Bun + TypeScript + Tailwind setup | 0.5 days |
| [2. API Client & Types](./02-api-client-types.md) | TypeScript types, fetch client, error handling | 0.5-1 day |
| [3. Core Layout](./03-core-layout.md) | App shell, design tokens, base components | 1 day |
| [4. Query Flow](./04-query-flow.md) | Input, submission, state management | 1-1.5 days |
| [5. SSE Streaming](./05-sse-streaming.md) | Real-time updates, progress display | 1 day |
| [6. Results Display](./06-results-display.md) | Answer, citations, sources panel | 1-1.5 days |
| [7. Polish & Testing](./07-polish-testing.md) | Error states, loading, a11y, tests | 1 day |

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Framework | SvelteKit 2 | Specified in project docs |
| Package manager | Bun | Specified in project docs, fast |
| Styling | Tailwind CSS v4 | Utility-first, fast iteration |
| Component library | None (custom) | "No bloat" principle, full control |
| State management | Svelte 5 runes | Modern, built-in reactivity |
| Icons | Lucide Svelte | Consistent, tree-shakeable |
| SSE handling | Native EventSource | No extra dependencies |
| Dark mode | System preference + toggle | User expectation |
| Job history | localStorage | MVP simplicity, no backend changes |
| Mobile support | Responsive (desktop-first) | Primary use case is desktop research |

## Design Language

| Element | Value |
|---------|-------|
| Primary color | Neutral/slate palette |
| Accent | Blue (trust, research) |
| Typography | System font stack |
| Spacing | 4px base unit |
| Border radius | Subtle (4-8px) |
| Shadows | Minimal, for elevation only |

## Dependencies

- Rust API running (localhost:4000)
- No external services required for frontend
- Node.js 20+ or Bun 1.0+ installed

## Environment Variables

```bash
# web/.env
PUBLIC_API_URL=http://localhost:4000
```

## Related Files

- `docs/interfaces/http-api.md` - API contract
- `docs/architecture/research-pipeline.md` - Pipeline stages for progress UI
- `crates/gorkd-api/src/dto.rs` - Response types to mirror
- `AGENTS.md` - Frontend code style guidelines
