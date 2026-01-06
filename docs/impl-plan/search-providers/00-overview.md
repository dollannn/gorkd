# Search Providers - Implementation Plan

**Status**: Planning
**Total Duration**: ~5-7 days
**Priority**: High

## Overview

Implement real search provider integrations for gorkd. Currently the system uses mock providers; this plan adds Tavily (primary), Exa (semantic search), and SearXNG (self-hosted fallback) as real search backends.

## Current State

| Component | Status | Notes |
|-----------|--------|-------|
| SearchProvider trait | Done | Defined in gorkd-core |
| SearchQuery, SearchResult types | Done | Supports filters, scoring |
| MockSearchProvider | Done | Works for testing |
| gorkd-search crate | Stub | Empty provider files |
| Multi-provider support | Missing | Executor uses single provider |

## Phases

| Phase | Description | Duration |
|-------|-------------|----------|
| [1. Shared Infrastructure](./01-shared-infrastructure.md) | Config, HTTP client, provider registry | 1 day |
| [2. Tavily Provider](./02-tavily-provider.md) | Primary search - Tavily API integration | 1-2 days |
| [3. Exa Provider](./03-exa-provider.md) | Semantic search - Exa AI integration | 1-2 days |
| [4. SearXNG Provider](./04-searxng-provider.md) | Self-hosted fallback - SearXNG integration | 1 day |
| [5. Integration Testing](./05-integration-testing.md) | E2E tests, provider selection, fallback | 1 day |

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Primary provider | Tavily | Best relevance scoring, includes answer generation |
| Semantic search | Exa | Neural search for nuanced queries |
| Fallback | SearXNG | Self-hosted, no API costs, privacy-friendly |
| HTTP client | reqwest | Already in workspace, async, well-maintained |
| Config | Environment vars | Standard, works with docker-compose |
| Provider selection | Registry pattern | Extensible, runtime provider switching |

## Provider Comparison

| Feature | Tavily | Exa | SearXNG |
|---------|--------|-----|---------|
| Relevance scoring | Excellent | Excellent | Basic |
| Recency filter | Yes | Yes | Yes |
| Domain filter | Yes | Yes | Via query syntax |
| Rate limits | 1000/month free | 1000/month free | Self-hosted |
| Cost | $0.01/search | $0.001/search | Free |
| Self-hostable | No | No | Yes |

## Dependencies

- API keys: `TAVILY_API_KEY`, `EXA_API_KEY`
- SearXNG instance URL (optional): `SEARXNG_URL`
- No database required for this phase

## Environment Variables

```bash
# Required (at least one)
TAVILY_API_KEY=tvly-xxxxx
EXA_API_KEY=exa-xxxxx

# Optional
SEARXNG_URL=https://searx.example.com
SEARCH_TIMEOUT_SECS=30
SEARCH_MAX_RESULTS=10
```

## Related Files

- `crates/gorkd-core/src/traits/search.rs` - SearchProvider trait
- `crates/gorkd-core/src/search.rs` - SearchQuery, SearchFilters
- `crates/gorkd-core/src/mock/search.rs` - MockSearchProvider (reference)
- `crates/gorkd-search/src/` - Implementation target
