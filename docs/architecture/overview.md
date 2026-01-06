# Architecture Overview

## System Diagram

```
                                    ┌─────────────────────────────────────┐
                                    │            Clients                   │
                                    │                                      │
                                    │  ┌─────────┐ ┌─────────┐ ┌────────┐ │
                                    │  │ Web UI  │ │ Discord │ │ Slack  │ │
                                    │  │SvelteKit│ │   Bot   │ │  Bot   │ │
                                    │  └────┬────┘ └────┬────┘ └───┬────┘ │
                                    └───────┼──────────┼───────────┼──────┘
                                            │          │           │
                                            └──────────┼───────────┘
                                                       │
                                                       ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│                              gorkd-api (Axum)                                 │
│                                                                               │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────────┐  │
│  │  POST /research │  │  GET /jobs/:id  │  │  GET /jobs/:id/stream (SSE) │  │
│  └────────┬────────┘  └────────┬────────┘  └──────────────┬──────────────┘  │
│           │                    │                          │                  │
│           └────────────────────┼──────────────────────────┘                  │
│                                │                                              │
│                                ▼                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                        gorkd-core                                     │   │
│  │                                                                       │   │
│  │  ┌──────────────┐    ┌──────────────┐    ┌──────────────────────┐   │   │
│  │  │ QueryPlanner │ -> │ SearchAgent  │ -> │ SynthesisEngine      │   │   │
│  │  │              │    │              │    │                      │   │   │
│  │  │ Parse intent │    │ Execute      │    │ LLM synthesis        │   │   │
│  │  │ Plan search  │    │ searches     │    │ Citation extraction  │   │   │
│  │  │ strategy     │    │ Fetch pages  │    │ Confidence scoring   │   │   │
│  │  └──────────────┘    └──────────────┘    └──────────────────────┘   │   │
│  │                                                                       │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                │                                              │
└────────────────────────────────┼──────────────────────────────────────────────┘
                                 │
                 ┌───────────────┼───────────────┬───────────────┐
                 │               │               │               │
                 ▼               ▼               ▼               ▼
         ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
         │ gorkd-search │ │  gorkd-llm   │ │ gorkd-store  │ │   External   │
         │              │ │              │ │              │ │   Services   │
         │ - Tavily     │ │ - OpenAI     │ │ - Postgres   │ │              │
         │ - Exa.ai     │ │ - Anthropic  │ │ - pgvector   │ │ - Tavily API │
         │ - SearXNG    │ │ - Ollama     │ │ - Job queue  │ │ - OpenAI API │
         └──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘
```

## Components

### gorkd-core

The heart of the system. Contains:

- **Domain types**: `ResearchJob`, `Source`, `Citation`, `Answer`
- **Research pipeline**: Query → Search → Fetch → Synthesize → Cite
- **Traits**: `SearchProvider`, `LlmProvider`, `Store`
- **Business logic**: Source ranking, confidence scoring, deduplication

**Dependencies**: None (pure domain logic)

### gorkd-api

HTTP API server built on Axum. Responsibilities:

- Request validation and routing
- Job lifecycle management
- SSE streaming for real-time updates
- Error handling and response formatting

**Dependencies**: gorkd-core, gorkd-store

### gorkd-bot-discord

Discord bot adapter. Responsibilities:

- Listen for commands/mentions
- Translate Discord interactions to research requests
- Format responses for Discord (embeds, buttons, length limits)
- Handle threading and follow-ups

**Dependencies**: gorkd-core (via HTTP or direct)

### gorkd-bot-slack

Slack bot adapter. Responsibilities:

- Handle Slack events (app mentions, slash commands)
- Format responses for Slack (blocks, threading)
- Manage rate limits and acknowledgments

**Dependencies**: gorkd-core (via HTTP or direct)

### gorkd-search

Search provider implementations:

- **Tavily**: High-quality factual search (primary)
- **Exa.ai**: Semantic search with embeddings
- **SearXNG**: Self-hosted meta-search (privacy option)

All implement the `SearchProvider` trait.

### gorkd-llm

LLM provider implementations:

- **OpenAI**: GPT-4o, GPT-4-turbo
- **Anthropic**: Claude 3.5 Sonnet, Claude 3 Opus
- **Ollama**: Local models (Llama, Mistral, etc.)

All implement the `LlmProvider` trait.

### gorkd-store

Persistence layer:

- **Postgres**: Job records, source metadata
- **pgvector**: Embeddings for semantic cache
- **Job queue**: Async job processing (if needed)

### web (SvelteKit)

Frontend application:

- Query input and submission
- Real-time result streaming
- Source inspection and verification
- Job history (local/session only)

## Data Flow

### Happy Path: New Query

```
1. User submits query (Web UI / Discord / Slack)
2. API creates ResearchJob, returns job_id
3. Client opens SSE stream for updates

4. QueryPlanner analyzes query:
   - Extract intent and key entities
   - Check vector cache for similar queries
   - Generate search strategy

5. SearchAgent executes searches:
   - Call Tavily/Exa with generated queries
   - Fetch and clean page content
   - Deduplicate sources

6. SynthesisEngine creates answer:
   - Send sources + query to LLM
   - Extract claims and map to citations
   - Score confidence

7. Store results:
   - Save job with answer and sources
   - Update vector cache

8. Stream final result to client
```

### Cached Query

```
1. User submits query
2. QueryPlanner finds similar query in vector cache
3. Return cached answer with "cached" flag
4. Total time: <2 seconds
```

## Key Boundaries

| Boundary | Rule |
|----------|------|
| gorkd-core | No I/O, no async runtime, pure logic |
| Providers (search/llm) | Implement traits, isolated from each other |
| API | Thin HTTP layer, delegates to core |
| Bots | Thin adapters, can call API or core directly |
| Store | Behind trait, swappable implementations |

## Concurrency Model

- **API**: Tokio async runtime, connection pooling
- **Research jobs**: One job = one task, parallel searches within job
- **Bots**: Async event loops, defer long work to API
- **Rate limiting**: Per-provider, token bucket

## Error Handling Strategy

| Layer | Strategy |
|-------|----------|
| API | Return structured errors with codes |
| Core | Result types, no panics |
| Providers | Retry with backoff, circuit breaker |
| Bots | Acknowledge fast, report errors async |

## Configuration

All via environment variables:

```
# Required
DATABASE_URL=postgres://...
OPENAI_API_KEY=sk-...
TAVILY_API_KEY=tvly-...

# Optional
ANTHROPIC_API_KEY=sk-ant-...
DISCORD_TOKEN=...
SLACK_BOT_TOKEN=xoxb-...
SEARXNG_URL=http://localhost:8080

# Tuning
RESEARCH_TIMEOUT_SECS=60
MAX_SOURCES_PER_QUERY=10
CACHE_TTL_HOURS=24
```

## Future Considerations

Deferred until proven necessary:

- **Message queue**: For now, in-process async. Add NATS/RabbitMQ if scale demands
- **Separate workers**: For now, API does research inline. Split if latency matters
- **Multi-region**: For now, single deployment. Add edge caching if global users
- **Auth**: For now, open. Add API keys if abuse becomes problem
