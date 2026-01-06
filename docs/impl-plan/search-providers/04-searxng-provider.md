# Phase 4: SearXNG Provider

**Status**: Planned
**Duration**: ~1 day
**Depends on**: Phase 1

## Overview

Implement SearXNG provider as a self-hosted fallback option. SearXNG is a privacy-respecting metasearch engine that aggregates results from multiple sources. No API key required, but needs a running instance.

## Tasks

| Task | Description |
|------|-------------|
| Request/Response types | Serde structs for SearXNG JSON API |
| SearxngProvider struct | Implements SearchProvider trait |
| Instance configuration | Support custom instance URLs |
| Filter mapping | Map SearchFilters to query params |
| Fallback handling | Handle instance unavailability |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-search/src/searxng.rs` | **Replace stub** - Full implementation |
| `crates/gorkd-search/src/lib.rs` | Export SearxngProvider |

## API Details

**Endpoint**: `GET {instance_url}/search?format=json`

**Query Parameters**:
```
q=search+text
format=json
categories=general
time_range=month
engines=google,bing,duckduckgo
safesearch=0
```

**Response**:
```json
{
  "query": "search text",
  "results": [
    {
      "title": "Result Title",
      "url": "https://example.com/page",
      "content": "Snippet text...",
      "engine": "google",
      "engines": ["google", "bing"],
      "score": 0.5
    }
  ]
}
```

## Filter Mapping

| SearchFilters | SearXNG Parameter |
|---------------|-------------------|
| recency: Day | time_range=day |
| recency: Week | time_range=week |
| recency: Month | time_range=month |
| recency: Year | time_range=year |
| include_domains | q=site:domain.com (query syntax) |
| content_type: News | categories=news |
| content_type: Academic | categories=science |

## Instance Configuration

```rust
SearxngProvider::new(instance_url)
// or
SearxngProvider::from_env()  // Uses SEARXNG_URL
```

## Key Considerations

- Instance must have JSON format enabled (not all public instances do)
- No authentication required
- Scores may be missing or inconsistent across engines
- Domain filtering uses query syntax (`site:example.com`), not API param
- Consider using multiple instances for redundancy
- User-Agent header recommended for identification

## Error Handling

| Condition | SearchError |
|-----------|-------------|
| Instance unreachable | ProviderUnavailable |
| 403 (JSON disabled) | ProviderUnavailable |
| Timeout | Timeout |
| Empty results | Ok([]) (not an error) |

## Public Instance Discovery

For testing/fallback, can query: `https://searx.space/data/instances.json`
Filter for instances with `http_status == 200` and `json == true`.

## Deliverables

- [ ] SearxngProvider implements SearchProvider
- [ ] Configurable instance URL
- [ ] SearchFilters mapped (including query syntax for domains)
- [ ] Graceful handling of missing scores
- [ ] Unit tests with mocked responses
- [ ] Integration test against public instance
- [ ] `cargo test -p gorkd-search` passes
