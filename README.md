# gorkd

> [!CAUTION]
> This project is **heavily AI-developed** and in **active development**. It is not ready for production use or any serious use. Expect breaking changes, incomplete features, and rough edges.

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

### Requirements

- Rust 1.75+
- Node.js 20+ / Bun 1.0+ (for web UI)
- Docker (optional, for Postgres + vector DB)

### Run locally (with mock providers)

```bash
# Start API server (uses mock providers, no API keys needed)
cargo run -p gorkd-api

# API available at http://localhost:4000
# Docs at http://localhost:4000/docs
```

### Run with real providers

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

### Rust (Backend)

```bash
cargo test                    # Run all tests
cargo fmt                     # Format code
cargo clippy -- -D warnings   # Lint
```

### Frontend (Web UI)

```bash
cd web
bun install       # Install dependencies
bun dev           # Start dev server (http://localhost:5173)
bun run build     # Production build
bun run check     # Type check
bun run lint      # ESLint
bun run format    # Prettier format
bun test          # Run tests
```

## Status

**Phase**: MVP Development

### Done
- [x] Core research pipeline design
- [x] API contract definition
- [x] Project scaffolding
- [x] End-to-end research flow (API with mock providers)

### Now
- [ ] Real search provider integration (Tavily)
- [ ] Real LLM provider integration (OpenAI/Anthropic)
- [ ] Basic web UI

### Next
- [ ] Discord bot MVP
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
