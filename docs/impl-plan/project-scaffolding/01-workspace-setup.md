# Phase 1: Workspace Setup

**Status**: Complete
**Duration**: ~1 day
**Depends on**: None

## Overview

Create Cargo workspace with all crate stubs, shared configs, and CI-ready structure. Every crate should compile (even if empty).

## Tasks

| Task | Description |
|------|-------------|
| Create workspace Cargo.toml | Define members, shared dependencies, workspace settings |
| Create gorkd-core crate | Lib crate, no dependencies |
| Create gorkd-api crate | Binary crate, axum + tokio |
| Create gorkd-search crate | Lib crate, reqwest + async-trait |
| Create gorkd-llm crate | Lib crate, reqwest + async-trait |
| Create gorkd-store crate | Lib crate, sqlx + async-trait |
| Create gorkd-bot-discord crate | Binary crate, serenity |
| Create gorkd-bot-slack crate | Binary crate, slack-morphism |
| Add shared configs | rustfmt.toml, clippy.toml, .cargo/config.toml |
| Add .env.example | All required environment variables |
| Verify cargo build | All crates compile |
| Verify cargo clippy | No warnings |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `Cargo.toml` | **New file** - Workspace manifest |
| `crates/gorkd-core/Cargo.toml` | **New file** - Core crate manifest |
| `crates/gorkd-core/src/lib.rs` | **New file** - Module exports |
| `crates/gorkd-api/Cargo.toml` | **New file** - API crate manifest |
| `crates/gorkd-api/src/main.rs` | **New file** - Entry point stub |
| `crates/gorkd-search/Cargo.toml` | **New file** - Search crate manifest |
| `crates/gorkd-search/src/lib.rs` | **New file** - Module exports |
| `crates/gorkd-llm/Cargo.toml` | **New file** - LLM crate manifest |
| `crates/gorkd-llm/src/lib.rs` | **New file** - Module exports |
| `crates/gorkd-store/Cargo.toml` | **New file** - Store crate manifest |
| `crates/gorkd-store/src/lib.rs` | **New file** - Module exports |
| `crates/gorkd-bot-discord/Cargo.toml` | **New file** - Discord bot manifest |
| `crates/gorkd-bot-discord/src/main.rs` | **New file** - Entry point stub |
| `crates/gorkd-bot-slack/Cargo.toml` | **New file** - Slack bot manifest |
| `crates/gorkd-bot-slack/src/main.rs` | **New file** - Entry point stub |
| `rustfmt.toml` | **New file** - Formatter config |
| `.cargo/config.toml` | **New file** - Cargo config (clippy lints) |
| `.env.example` | **New file** - Environment template |

## Key Considerations

- Workspace-level dependency versions (avoid version drift)
- gorkd-core must have zero I/O dependencies (no tokio, no reqwest)
- Binary crates need `[[bin]]` section or default naming
- Use Rust 2021 edition throughout

## Workspace Dependency Strategy

```toml
# Root Cargo.toml [workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }
tokio = { version = "1.0", features = ["full"] }
axum = "0.7"
# etc.
```

Crates inherit via `package.workspace = true`.

## Deliverables

- [x] `cargo build` succeeds for all crates
- [x] `cargo clippy -- -D warnings` passes
- [x] `cargo fmt --check` passes
- [x] All 7 crates exist with correct structure
- [x] .env.example documents all expected variables
