# Phase 1: Shared Infrastructure

**Status**: Planned
**Duration**: ~1 day
**Depends on**: None

## Overview

Set up the foundational components: configuration loading, HTTP client for API communication, bot state container, and logging. This phase establishes the skeleton that all subsequent phases build upon.

## Tasks

| Task | Description |
|------|-------------|
| Config module | Load DISCORD_TOKEN, API_URL, rate limits from env |
| API client | reqwest-based client for gorkd-api with timeout/retry |
| Bot state | Shared state container for client, config, tracking |
| Logging setup | tracing with env_filter, structured logging |
| Client builder | Serenity client with intents and event handler stub |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-bot-discord/Cargo.toml` | Add reqwest, dashmap, secrecy dependencies |
| `crates/gorkd-bot-discord/src/main.rs` | Client setup, graceful shutdown |
| `crates/gorkd-bot-discord/src/lib.rs` | **New file** - Module exports |
| `crates/gorkd-bot-discord/src/config.rs` | **New file** - BotConfig struct |
| `crates/gorkd-bot-discord/src/api.rs` | **New file** - ApiClient for gorkd-api |
| `crates/gorkd-bot-discord/src/state.rs` | **New file** - BotState container |
| `crates/gorkd-bot-discord/src/handler.rs` | **New file** - EventHandler stub |

## Implementation Details

### Config Structure

```
BotConfig
├── discord_token: SecretString
├── application_id: Option<u64>
├── api_url: String (default: "http://localhost:4000")
├── web_ui_url: Option<String>
├── rate_limit_per_user: u32 (default: 5)
└── rate_limit_per_guild: u32 (default: 20)
```

### API Client Methods

```
ApiClient
├── new(base_url, timeout) -> Self
├── create_research(query) -> Result<CreateResearchResponse>
├── get_job(job_id) -> Result<JobResponse>
└── get_sources(job_id) -> Result<Vec<Source>>
```

### Bot State

```
BotState
├── config: BotConfig
├── api: ApiClient
├── active_jobs: DashMap<MessageId, JobId>  // Track which messages are being updated
└── http: Arc<Http>  // Serenity HTTP client for message editing
```

### Gateway Intents

Required intents for bot functionality:
- `GUILDS` - Server join/leave events
- `GUILD_MESSAGES` - Message content for mentions
- `MESSAGE_CONTENT` - Read message text (privileged intent)
- `DIRECT_MESSAGES` - DM support (optional)

## Key Considerations

- DISCORD_TOKEN is sensitive; use secrecy crate to prevent logging
- API client should have reasonable timeout (30s) matching research duration
- Fail fast if required env vars missing
- Graceful shutdown: wait for active jobs to complete or timeout

## Deliverables

- [ ] BotConfig loads from environment with validation
- [ ] ApiClient can reach gorkd-api health endpoint
- [ ] Serenity client connects to Discord gateway
- [ ] Bot logs "Ready" with username when connected
- [ ] Ctrl+C triggers graceful shutdown
- [ ] `cargo build -p gorkd-bot-discord` passes
- [ ] `cargo clippy -p gorkd-bot-discord` clean
