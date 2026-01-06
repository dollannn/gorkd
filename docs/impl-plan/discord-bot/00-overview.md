# Discord Bot - Implementation Plan

**Status**: Planning
**Total Duration**: ~5-7 days
**Priority**: High

## Overview

Implement the Discord bot adapter for gorkd research. Users can mention the bot or use slash commands to submit research queries. The bot responds with rich embeds showing the answer, confidence level, and cited sources. Supports progress updates, interactive buttons, and context menu for fact-checking existing messages.

## Current State

| Component | Status | Notes |
|-----------|--------|-------|
| `gorkd-bot-discord` crate | Stub | Empty `main.rs`, basic `Cargo.toml` |
| Discord interface spec | Complete | `docs/interfaces/discord.md` |
| Serenity dependency | Configured | v0.12 in workspace with client, gateway, model features |
| gorkd-api | Complete | HTTP endpoints + SSE streaming ready |
| Pipeline | Complete | Full research flow via API |

## Phases

| Phase | Description | Duration |
|-------|-------------|----------|
| [1. Shared Infrastructure](./01-shared-infrastructure.md) | Config, API client, bot state, logging | 1 day |
| [2. Event Handlers](./02-event-handlers.md) | Mention listener, slash commands, context menu | 1-2 days |
| [3. Research Flow](./03-research-flow.md) | Job creation, progress polling, message updates | 1-2 days |
| [4. Rich Responses](./04-rich-responses.md) | Embeds, buttons, ephemeral sources, error states | 1 day |
| [5. Rate Limiting & Polish](./05-rate-limiting.md) | Rate limits, threading, graceful shutdown | 1 day |

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Integration pattern | HTTP API | Matches architecture, API already handles orchestration, independent scaling |
| Discord library | Serenity 0.12 | Already in workspace, well-maintained, good docs |
| Progress updates | Poll API + edit message | SSE adds complexity; polling every 2s is simpler for MVP |
| State management | In-memory with dashmap | Fast, concurrent-safe; no persistence needed for bot state |
| Rate limit storage | In-memory | Stateless per-instance; acceptable for single-bot deployment |

## Architecture

```
User
  │
  ▼ (mention / slash command / context menu)
┌─────────────────────────────────────────────────┐
│              gorkd-bot-discord                   │
│                                                  │
│  ┌──────────────┐  ┌──────────────────────────┐ │
│  │ EventHandler │  │ ResearchManager          │ │
│  │              │  │                          │ │
│  │ - message    │  │ - create_job()           │ │
│  │ - interaction│  │ - poll_progress()        │ │
│  │ - ready      │  │ - format_response()      │ │
│  └──────┬───────┘  └────────────┬─────────────┘ │
│         │                       │               │
│         └───────────┬───────────┘               │
│                     ▼                           │
│           ┌─────────────────┐                   │
│           │   ApiClient     │                   │
│           │                 │                   │
│           │ POST /research  │                   │
│           │ GET /jobs/:id   │                   │
│           └────────┬────────┘                   │
└────────────────────┼────────────────────────────┘
                     │
                     ▼ HTTP
            ┌─────────────────┐
            │   gorkd-api     │
            │                 │
            │   Pipeline      │
            └─────────────────┘
```

## Environment Variables

```bash
# Required
DISCORD_TOKEN=your-bot-token
GORKD_API_URL=http://localhost:4000  # API server URL

# Optional
DISCORD_APPLICATION_ID=123456789     # For slash command registration
WEB_UI_URL=http://localhost:5173     # For "View Full Analysis" links
RATE_LIMIT_PER_USER=5                # Queries per minute per user
RATE_LIMIT_PER_GUILD=20              # Queries per minute per guild
```

## Dependencies

- gorkd-api must be running and accessible
- Discord bot token with required permissions:
  - Send Messages
  - Embed Links
  - Use External Emojis
  - Add Reactions
  - Read Message History (for context menu)
  - Use Slash Commands

## Related Files

- `docs/interfaces/discord.md` - Complete interaction protocol spec
- `docs/architecture/overview.md` - System architecture (bot as thin adapter)
- `crates/gorkd-api/src/routes/research.rs` - API endpoint the bot calls
- `crates/gorkd-api/src/routes/jobs.rs` - Job status endpoint
- `crates/gorkd-api/src/dto.rs` - Request/response types to mirror
