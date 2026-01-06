# Phase 3: Exa Provider

**Status**: Planned
**Duration**: ~1-2 days
**Depends on**: Phase 1

## Overview

Implement Exa AI search provider for semantic/neural search capabilities. Exa excels at understanding query intent and finding conceptually relevant results, complementing Tavily's keyword-focused search.

## Tasks

| Task | Description |
|------|-------------|
| Request/Response types | Serde structs for Exa API |
| ExaProvider struct | Implements SearchProvider trait |
| Search type selection | Neural vs keyword based on query |
| Filter mapping | Map SearchFilters to Exa params |
| Error mapping | Map Exa errors to SearchError |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-search/src/exa.rs` | **Replace stub** - Full implementation |
| `crates/gorkd-search/src/lib.rs` | Export ExaProvider |

## API Details

**Endpoint**: `POST https://api.exa.ai/search`

**Headers**: `x-api-key: exa-xxx`

**Request**:
```json
{
  "query": "search text",
  "type": "neural",
  "numResults": 10,
  "includeDomains": ["example.com"],
  "excludeDomains": ["spam.com"],
  "startPublishedDate": "2024-01-01T00:00:00Z",
  "endPublishedDate": "2024-12-31T23:59:59Z"
}
```

**Response**:
```json
{
  "results": [
    {
      "title": "Result Title",
      "url": "https://example.com/page",
      "text": "Snippet or full text...",
      "score": 0.234,
      "publishedDate": "2024-06-15T00:00:00Z"
    }
  ]
}
```

## Filter Mapping

| SearchFilters | Exa Parameter |
|---------------|---------------|
| recency: Day | startPublishedDate: now - 1 day |
| recency: Week | startPublishedDate: now - 7 days |
| recency: Month | startPublishedDate: now - 30 days |
| recency: Year | startPublishedDate: now - 365 days |
| include_domains | includeDomains |
| exclude_domains | excludeDomains |

## Search Type Selection

| Query Pattern | Search Type |
|---------------|-------------|
| Factual questions | neural |
| Specific terms/names | keyword |
| Conceptual queries | neural |
| Default | neural |

## Key Considerations

- API key in header (`x-api-key`), not body
- Exa scores are not normalized to 0-1; normalize in mapping
- Date filters use ISO 8601 format
- Neural search is default and works best for research queries
- `text` field may contain more content than Tavily snippets

## Deliverables

- [ ] ExaProvider implements SearchProvider
- [ ] All SearchFilters mapped to Exa params
- [ ] Score normalization (Exa scores vary widely)
- [ ] Errors mapped to SearchError variants
- [ ] Unit tests with mocked responses
- [ ] Integration test (feature-gated)
- [ ] `cargo test -p gorkd-search` passes
