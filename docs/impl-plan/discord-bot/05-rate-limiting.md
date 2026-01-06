# Phase 5: Rate Limiting & Polish

**Status**: Planned
**Duration**: ~1 day
**Depends on**: Phase 4

## Overview

Add rate limiting to prevent abuse, implement proper threading for follow-up questions, handle edge cases, and ensure graceful shutdown. This phase focuses on production-readiness and polish.

## Tasks

| Task | Description |
|------|-------------|
| Per-user rate limit | 5 queries/minute per user |
| Per-guild rate limit | 20 queries/minute per server |
| Rate limit response | Friendly "please wait" message |
| Threading | Follow-ups in thread, busy channel auto-thread |
| Graceful shutdown | Wait for active jobs, cleanup |
| Health check | Expose simple health endpoint for monitoring |
| Metrics | Basic counters for queries, errors, latency |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-bot-discord/src/rate_limit.rs` | **New file** - RateLimiter |
| `crates/gorkd-bot-discord/src/handler.rs` | Add rate limit checks |
| `crates/gorkd-bot-discord/src/main.rs` | Graceful shutdown, signal handling |
| `crates/gorkd-bot-discord/src/metrics.rs` | **New file** - Basic metrics |

## Implementation Details

### Rate Limiter Structure

```
RateLimiter
├── user_limits: DashMap<UserId, TokenBucket>
├── guild_limits: DashMap<GuildId, TokenBucket>
├── check_user(user_id) -> Result<(), Duration>
├── check_guild(guild_id) -> Result<(), Duration>
└── cleanup_expired() // Run periodically
```

### Token Bucket Algorithm

```
TokenBucket
├── tokens: u32 (current available)
├── max_tokens: u32 (capacity)
├── refill_rate: Duration (time per token)
├── last_refill: Instant
├── try_consume() -> Result<(), Duration>
└── refill() // Called before consume
```

### Rate Limit Response

```
You're sending queries too fast. Please wait 12 seconds.

Limits: 5 queries/minute per user, 20 queries/minute per server.
```

### Threading Rules

| Condition | Action |
|-----------|--------|
| First query in channel | Reply in channel |
| Query is reply to bot message | Continue in same thread |
| Channel has >10 messages in last minute | Auto-create thread |
| Query in existing thread | Reply in thread |

### Graceful Shutdown

1. Receive SIGINT/SIGTERM
2. Stop accepting new queries
3. Wait up to 30s for active research to complete
4. Cancel any remaining jobs
5. Disconnect from gateway
6. Exit cleanly

### Metrics to Track

| Metric | Type | Description |
|--------|------|-------------|
| `queries_total` | Counter | Total queries received |
| `queries_by_type` | Counter | Breakdown by mention/slash/menu |
| `research_duration_seconds` | Histogram | Time from query to response |
| `errors_total` | Counter | Errors by type |
| `rate_limits_hit` | Counter | Rate limit rejections |
| `active_jobs` | Gauge | Currently polling jobs |

## Key Considerations

- Rate limit state is per-instance; multiple bot instances don't share
- DashMap entries should expire to prevent memory growth
- Cleanup task runs every 60 seconds to remove stale entries
- Threading requires MANAGE_THREADS permission in some cases
- Metrics are internal only (no external endpoint) for MVP

## Deliverables

- [ ] Users rate limited to 5/min
- [ ] Guilds rate limited to 20/min  
- [ ] Rate limit message shows wait time
- [ ] Follow-up queries go to threads
- [ ] Ctrl+C waits for active jobs (up to 30s)
- [ ] No memory leaks from rate limit tracking
- [ ] Metrics logged on shutdown (summary)
- [ ] Bot recovers from temporary API outages
- [ ] `cargo clippy -p gorkd-bot-discord` clean
- [ ] README updated with bot setup instructions
