# AGENTS.md - gorkd

Guidelines for AI agents working in this codebase.

## Project Overview

**gorkd** is a truth-seeking research bot: Query → Search → Synthesize → Cite.
Rust backend (Axum API + bot adapters) + SvelteKit frontend.

Status: **Greenfield** - Design & Documentation phase.

## Build / Test / Lint Commands

### Rust (Backend)

```bash
cargo build                           # Build all crates
cargo build -p gorkd-core             # Build specific crate
cargo test                            # Run all tests
cargo test -p gorkd-core              # Run tests for specific crate
cargo test -p gorkd-core -- test_name --exact  # Run single test
cargo fmt                             # Format code
cargo clippy -- -D warnings           # Lint
cargo run -p gorkd-api                # Run API server
```

### Frontend (SvelteKit)

```bash
cd web
bun install         # Install dependencies
bun dev             # Development server
bun test            # Run tests
bun run check       # Type check
bun run lint        # Lint
bun run format      # Format
```

### Infrastructure

```bash
docker compose up -d   # Start local Postgres + vector DB
docker compose down    # Stop infrastructure
```

## Repository Structure

```
/crates
  /gorkd-core         # Research pipeline, domain types, traits (NO I/O)
  /gorkd-api          # HTTP API (Axum)
  /gorkd-bot-discord  # Discord bot adapter
  /gorkd-bot-slack    # Slack bot adapter
  /gorkd-search       # Search providers (Tavily, Exa, SearXNG)
  /gorkd-llm          # LLM provider abstraction
  /gorkd-store        # Vector DB + job storage
/web                  # SvelteKit frontend
/docs                 # Architecture, interfaces, decisions
```

## Code Style - Rust

### Imports (order: std → external → internal → local)

```rust
use std::collections::HashMap;
use axum::Router;
use gorkd_core::ResearchJob;
use crate::handlers;
```

### Naming & Types

- Types: `PascalCase`, Functions: `snake_case`, Constants: `SCREAMING_SNAKE_CASE`
- ID prefixes: `job_`, `src_` (e.g., `job_abc123`)

### Error Handling

```rust
// Use Result types, never panic in library code
#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("query cannot be empty")]
    Empty,
}
```

### gorkd-core Rules (CRITICAL)

- **NO I/O**: Pure domain logic only
- **NO async runtime**: Sync operations, traits may be async in implementors
- Define traits here, implement in other crates

## Code Style - TypeScript/Svelte

### Formatting

- Prettier defaults, tabs, single quotes, no semicolons

### Imports (order: Svelte → external → internal → types)

```typescript
import { onMount } from 'svelte'
import { z } from 'zod'
import { api } from '$lib/api'
import type { ResearchJob } from '$lib/types'
```

### Types

```typescript
// Interfaces for objects
interface ResearchJob { id: string; query: string; status: JobStatus }

// Type for unions
type JobStatus = 'pending' | 'searching' | 'completed' | 'failed'

// Zod for runtime validation
const QuerySchema = z.object({ query: z.string().min(1).max(2000) })
```

## API Conventions

- RESTful endpoints: `GET /jobs/:id`, `POST /research`
- All timestamps: ISO 8601 UTC
- ID prefixes: `job_`, `src_`
- Errors: `{ "error": { "code": "...", "message": "..." } }`

## Architecture Decisions

Document significant decisions in `/docs/decisions/` as ADRs:
- Library/framework choices
- Data model design
- API contracts
- Trade-offs accepting technical debt

## Key Principles

1. **Evidence-first**: Every claim needs a citation
2. **One workflow**: Ask → Research → Deliver (no modes)
3. **Show the work**: Intermediate steps visible
4. **Speed matters**: P50 < 15s, cached < 2s
5. **No bloat**: Every feature justifies its existence

## Testing

### Rust

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creates_job_with_valid_query() {
        let job = create_job("What is X?").unwrap();
        assert_eq!(job.status, JobStatus::Pending);
    }
}
```

### Frontend

- Unit tests: Vitest
- Component tests: Testing Library
- E2E: Playwright (when applicable)

## What NOT to Do

- Never suppress type errors (`as any`, `@ts-ignore`, `// @ts-expect-error`)
- Never use empty catch blocks
- Never panic in gorkd-core (use Result)
- Never put I/O in gorkd-core
- Never commit secrets or API keys
- Never delete failing tests to make CI pass
