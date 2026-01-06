# gorkd

A truth-seeking research bot. Ask a question, get a cited answer.

Web search + LLM reasoning + source citations. Accessible via web UI, Discord, and Slack.

## What it does

- **Research on demand**: Query → Search → Synthesize → Cite
- **Multi-interface**: Web UI (SvelteKit), Discord bot, Slack bot
- **Evidence-first**: Every answer includes sources and reasoning
- **Fast repeat lookups**: Vector cache for previously researched topics
- **Transparent**: Shows search queries, sources considered, confidence levels

## What it doesn't do

- General-purpose chatbot or assistant
- Autonomous long-running agents
- Plugin marketplace or extensibility framework
- Multi-tenant enterprise features
- RAG over your private documents (this is for web research)

## Principles

| Principle | Meaning |
|-----------|---------|
| **Evidence-first** | Output includes citations, source quality signals, and "why this source" |
| **One workflow** | Ask → Research → Deliver. No modes, no complexity |
| **Show the work** | Intermediate steps visible: queries used, sources fetched, synthesis reasoning |
| **Speed matters** | Common queries answered in <30s, cached queries instant |
| **No bloat** | Every feature must justify its existence. When in doubt, leave it out |

## Repository Structure

```
/crates
  /gorkd-core         # Research pipeline, domain types, traits
  /gorkd-api          # HTTP API (Axum)
  /gorkd-bot-discord  # Discord bot adapter
  /gorkd-bot-slack    # Slack bot adapter
  /gorkd-search       # Search providers (Tavily, SearXNG, etc.)
  /gorkd-llm          # LLM provider abstraction
  /gorkd-store        # Vector DB + job storage

/web                  # SvelteKit frontend

/docs                 # Architecture, interfaces, decisions
```

## Quickstart

> **Status**: Greenfield. These commands are targets, not yet implemented.

### Requirements

- Rust 1.75+
- Node.js 20+ / Bun 1.0+
- Docker (for local Postgres + vector DB)
- API keys: OpenAI/Anthropic (LLM), Tavily/Exa (search)

### Run locally

```bash
# Start infrastructure
docker compose up -d

# Copy and configure environment
cp .env.example .env
# Edit .env with your API keys

# Start API server
cargo run -p gorkd-api

# Start web UI (separate terminal)
cd web && bun dev

# Start bots (optional, separate terminals)
cargo run -p gorkd-bot-discord
cargo run -p gorkd-bot-slack
```

### Ports

| Service | Port | URL |
|---------|------|-----|
| API | 4000 | http://localhost:4000 |
| Web UI | 5173 | http://localhost:5173 |

### Example query

```bash
curl -X POST http://localhost:4000/v1/research \
  -H "Content-Type: application/json" \
  -d '{"query": "What caused the 2024 CrowdStrike outage?"}'
```

## Configuration

All configuration via environment variables. See `.env.example` for full list.

| Variable | Required | Description |
|----------|----------|-------------|
| `OPENAI_API_KEY` | Yes* | OpenAI API key |
| `ANTHROPIC_API_KEY` | Yes* | Anthropic API key |
| `TAVILY_API_KEY` | Yes | Tavily search API key |
| `DATABASE_URL` | Yes | Postgres connection string |
| `DISCORD_TOKEN` | For bot | Discord bot token |
| `SLACK_BOT_TOKEN` | For bot | Slack bot token |

*At least one LLM provider required.

## Development

```bash
# Run all tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# Frontend
cd web && bun test && bun run check
```

## Status

**Phase**: Design & Documentation

### Now
- [ ] Core research pipeline design
- [ ] API contract definition
- [ ] Project scaffolding

### Next
- [ ] End-to-end research flow (API only)
- [ ] Basic web UI
- [ ] Discord bot MVP

### Later
- [ ] Slack bot
- [ ] Vector caching
- [ ] Source quality scoring
- [ ] Streaming responses

## Documentation

- [Vision & Philosophy](docs/vision.md)
- [Architecture Overview](docs/architecture/overview.md)
- [Research Pipeline](docs/architecture/research-pipeline.md)
- [HTTP API](docs/interfaces/http-api.md)
- [Discord Bot](docs/interfaces/discord.md)
- [Slack Bot](docs/interfaces/slack.md)
- [Decisions](docs/decisions/)

## License

MIT
