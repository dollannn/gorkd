# Phase 2: Tavily Provider

**Status**: Complete
**Duration**: ~1-2 days
**Depends on**: Phase 1

## Overview

Implement Tavily search provider as the primary search backend. Tavily offers high-quality web search with relevance scoring, recency filters, and domain filtering - all features that map directly to gorkd's SearchQuery model.

## Tasks

| Task | Description |
|------|-------------|
| Request/Response types | Serde structs for Tavily API |
| TavilyProvider struct | Implements SearchProvider trait |
| Filter mapping | Map SearchFilters to Tavily params |
| Error mapping | Map Tavily errors to SearchError |
| Integration test | Test against real API (optional, gated) |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-search/src/tavily.rs` | **Replace stub** - Full implementation |
| `crates/gorkd-search/src/lib.rs` | Export TavilyProvider |

## API Details

**Endpoint**: `POST https://api.tavily.com/search`

**Request**:
```json
{
  "api_key": "tvly-xxx",
  "query": "search text",
  "search_depth": "basic",
  "max_results": 10,
  "include_domains": ["example.com"],
  "exclude_domains": ["spam.com"],
  "time_range": "week"
}
```

**Response**:
```json
{
  "results": [
    {
      "title": "Result Title",
      "url": "https://example.com/page",
      "content": "Snippet text...",
      "score": 0.95
    }
  ]
}
```

## Filter Mapping

| SearchFilters | Tavily Parameter |
|---------------|------------------|
| recency: Day | time_range: "day" |
| recency: Week | time_range: "week" |
| recency: Month | time_range: "month" |
| recency: Year | time_range: "year" |
| include_domains | include_domains |
| exclude_domains | exclude_domains |
| content_type: News | topic: "news" |

## Error Mapping

| Tavily Error | SearchError |
|--------------|-------------|
| 401 Unauthorized | ProviderUnavailable |
| 429 Rate Limited | RateLimited |
| Timeout | Timeout |
| Network failure | Network |
| Other 4xx/5xx | Provider |

## Key Considerations

- API key goes in request body, not header (Tavily convention)
- search_depth: "basic" for speed, "advanced" for thoroughness
- Score is 0.0-1.0 and maps directly to SearchResult.score
- Content field is the snippet, not full page content

## Deliverables

- [x] TavilyProvider implements SearchProvider
- [x] All SearchFilters mapped to Tavily params
- [x] Errors mapped to SearchError variants
- [x] Unit tests with mocked responses
- [x] Integration test (feature-gated: `--features integration`)
- [x] `cargo test -p gorkd-search` passes
