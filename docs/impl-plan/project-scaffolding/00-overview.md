# Project Scaffolding - Implementation Plan

**Status**: Planning
**Total Duration**: ~5-7 days
**Priority**: High

## Overview

Bootstrap the gorkd Rust workspace from zero to a working skeleton: Cargo workspace, core domain types, provider traits, and minimal API server that compiles and runs.

## Current State

| Component | Status | Notes |
|-----------|--------|-------|
| Design docs | Done | Vision, architecture, pipeline, API spec |
| Rust code | Missing | No crates exist |
| Frontend | Missing | No SvelteKit app |
| Infrastructure | Missing | No docker-compose |

## Phases

| Phase | Description | Duration |
|-------|-------------|----------|
| [1. Workspace Setup](./01-workspace-setup.md) | Cargo workspace, all crate stubs, configs | 1 day |
| [2. Core Domain Types](./02-core-domain-types.md) | ResearchJob, Source, Answer, all enums | 1-2 days |
| [3. Provider Traits](./03-provider-traits.md) | SearchProvider, LlmProvider, Store traits | 1 day |
| [4. API Skeleton](./04-api-skeleton.md) | Axum server with stubbed endpoints | 1-2 days |
| [5. Integration Smoke Test](./05-integration-test.md) | End-to-end flow with mock providers | 1 day |

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Error handling | `thiserror` + custom error types per crate | Idiomatic, good error messages |
| ID generation | `nanoid` with prefixes (`job_`, `src_`) | URL-safe, collision-resistant, readable |
| Async runtime | `tokio` (API/bots only, not core) | Standard, well-supported |
| HTTP framework | `axum` | Type-safe, tower ecosystem, async-first |
| Serialization | `serde` + `serde_json` | De facto standard |
| Time handling | `chrono` with UTC | Timezone-aware, ISO 8601 support |

## Dependencies

- Rust 1.75+ installed
- No external services needed for scaffolding phase

## Related Files

- `docs/architecture/overview.md` - Component boundaries
- `docs/architecture/research-pipeline.md` - Domain types reference
- `docs/interfaces/http-api.md` - API contract
- `AGENTS.md` - Code style guidelines
