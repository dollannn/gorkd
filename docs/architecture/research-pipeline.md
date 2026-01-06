# Research Pipeline

The core algorithm that turns a question into a cited answer.

## Pipeline Stages

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   INTAKE    │ -> │    PLAN     │ -> │   SEARCH    │ -> │  SYNTHESIZE │ -> │   DELIVER   │
│             │    │             │    │             │    │             │    │             │
│ Parse query │    │ Cache check │    │ Execute     │    │ LLM call    │    │ Format      │
│ Validate    │    │ Query gen   │    │ Fetch pages │    │ Citations   │    │ Store       │
│ Create job  │    │ Strategy    │    │ Clean text  │    │ Confidence  │    │ Stream      │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
```

## Stage 1: Intake

**Input**: Raw user query (string)
**Output**: `ResearchJob` with parsed intent

### Operations

1. **Validate query**
   - Non-empty, reasonable length (<2000 chars)
   - Not obviously malicious or nonsensical

2. **Parse intent** (optional LLM call for complex queries)
   - Identify question type: factual, comparison, explanation, current event
   - Extract key entities and time constraints
   - Detect language

3. **Create job**
   - Assign job ID
   - Set initial status: `pending`
   - Record timestamp

### Output Schema

```rust
struct ResearchJob {
    id: JobId,
    query: String,
    intent: QueryIntent,
    status: JobStatus,
    created_at: DateTime<Utc>,
}

struct QueryIntent {
    question_type: QuestionType,  // Factual, Comparison, Explanation, CurrentEvent
    entities: Vec<String>,
    time_constraint: Option<TimeConstraint>,  // Recent, Historical, Specific date
    language: String,
}
```

## Stage 2: Plan

**Input**: `ResearchJob` with intent
**Output**: `SearchPlan` with queries and strategy

### Operations

1. **Check cache**
   - Embed query using embedding model
   - Search vector store for similar queries (cosine similarity > 0.92)
   - If found and fresh (< TTL), return cached result immediately

2. **Generate search queries**
   - Transform user question into effective search queries
   - Multiple variations to increase coverage
   - Add time filters for current events

3. **Select search providers**
   - Tavily for factual queries (default)
   - Exa for semantic/conceptual queries
   - Multiple providers for high-importance queries

### Output Schema

```rust
struct SearchPlan {
    queries: Vec<SearchQuery>,
    providers: Vec<ProviderId>,
    max_sources: usize,
    timeout: Duration,
}

struct SearchQuery {
    text: String,
    filters: SearchFilters,
}

struct SearchFilters {
    recency: Option<Recency>,  // Day, Week, Month, Year, Any
    domains: Option<Vec<String>>,  // Include/exclude
    content_type: Option<ContentType>,  // News, Academic, General
}
```

## Stage 3: Search

**Input**: `SearchPlan`
**Output**: `SourceCollection`

### Operations

1. **Execute searches** (parallel)
   - Call each provider with each query
   - Respect rate limits
   - Timeout individual calls (10s default)

2. **Fetch page content**
   - For each search result URL, fetch full content
   - Use readability extraction (clean HTML → text)
   - Respect robots.txt and rate limits

3. **Process sources**
   - Deduplicate by URL
   - Extract metadata (title, date, author, domain)
   - Compute initial relevance score

4. **Rank and filter**
   - Score by: query relevance, source authority, recency, content quality
   - Keep top N sources (default: 10)
   - Ensure diversity (not all from same domain)

### Output Schema

```rust
struct SourceCollection {
    sources: Vec<Source>,
    search_metadata: SearchMetadata,
}

struct Source {
    id: SourceId,
    url: String,
    title: String,
    content: String,  // Cleaned text
    metadata: SourceMetadata,
    relevance_score: f32,
}

struct SourceMetadata {
    domain: String,
    published_at: Option<DateTime<Utc>>,
    author: Option<String>,
    word_count: usize,
}

struct SearchMetadata {
    queries_executed: Vec<String>,
    providers_used: Vec<ProviderId>,
    total_results: usize,
    fetch_duration: Duration,
}
```

## Stage 4: Synthesize

**Input**: `ResearchJob`, `SourceCollection`
**Output**: `ResearchAnswer`

### Operations

1. **Prepare context**
   - Format sources for LLM consumption
   - Include source IDs for citation tracking
   - Truncate if exceeding context window

2. **LLM synthesis call**
   - System prompt: "You are a research assistant. Answer based ONLY on provided sources. Cite every claim."
   - User prompt: Original query + formatted sources
   - Request structured output (answer + citations)

3. **Extract citations**
   - Parse LLM output for citation markers
   - Map citations to source IDs
   - Verify each citation actually supports the claim

4. **Score confidence**
   - High: Multiple corroborating sources, authoritative domains
   - Medium: Single strong source or multiple weaker sources
   - Low: Conflicting sources or weak evidence
   - Insufficient: Cannot answer from available sources

5. **Generate limitations**
   - Note conflicting information
   - Note missing perspectives
   - Note recency concerns

### Output Schema

```rust
struct ResearchAnswer {
    summary: String,  // Direct answer to the question
    detail: String,   // Extended explanation
    citations: Vec<Citation>,
    confidence: Confidence,
    limitations: Vec<String>,
    synthesis_metadata: SynthesisMetadata,
}

struct Citation {
    claim: String,
    source_id: SourceId,
    quote: Option<String>,  // Direct quote if applicable
}

enum Confidence {
    High,
    Medium,
    Low,
    Insufficient,
}

struct SynthesisMetadata {
    model: String,
    tokens_used: usize,
    synthesis_duration: Duration,
}
```

## Stage 5: Deliver

**Input**: `ResearchJob`, `ResearchAnswer`, `SourceCollection`
**Output**: Formatted response to client

### Operations

1. **Store results**
   - Save job with final status: `completed` or `failed`
   - Store answer and sources in database
   - Update vector cache with query embedding

2. **Format for client**
   - Web UI: Full structured response
   - Discord: Embed with summary, expandable sources
   - Slack: Blocks with summary, threaded sources

3. **Stream updates**
   - Send final SSE event with complete response
   - Close stream

### Response Formats

**Web UI (JSON)**
```json
{
  "job_id": "job_abc123",
  "status": "completed",
  "answer": {
    "summary": "Yes, the FDA approved Leqembi...",
    "detail": "Extended explanation...",
    "confidence": "high",
    "limitations": ["Limited long-term data"]
  },
  "citations": [
    {"claim": "...", "source_id": "src_1", "quote": "..."}
  ],
  "sources": [
    {"id": "src_1", "url": "...", "title": "...", "domain": "fda.gov"}
  ],
  "metadata": {
    "duration_ms": 12340,
    "sources_considered": 15,
    "cached": false
  }
}
```

**Discord (Embed)**
```
📊 Research Complete

**Question**: Did the FDA approve the new Alzheimer's drug?

**Answer**: Yes, the FDA granted traditional approval to Leqembi 
(lecanemab) in July 2023 for early Alzheimer's disease.

**Confidence**: High (3 corroborating sources)

**Sources**:
1. FDA.gov - FDA Approves Treatment for Adults...
2. NEJM - Lecanemab in Early Alzheimer's Disease
3. Reuters - FDA grants full approval to Eisai...

[View Full Analysis](https://gorkd.app/jobs/abc123)
```

## Error Handling

| Stage | Error | Action |
|-------|-------|--------|
| Intake | Invalid query | Return 400 with message |
| Plan | Cache error | Continue without cache |
| Search | Provider timeout | Use available results |
| Search | All providers fail | Return error with retry option |
| Synthesize | LLM error | Retry once, then fail |
| Synthesize | No relevant sources | Return "insufficient sources" answer |
| Deliver | Store error | Log, return result anyway |

## Performance Targets

| Metric | Target |
|--------|--------|
| Cache hit | <500ms |
| Simple query (cached sources) | <5s |
| Standard query | <15s |
| Complex query (many sources) | <30s |
| Timeout | 60s |

## Observability

Each stage emits:
- Structured logs with job ID
- Timing metrics
- Error counts by type

Key metrics:
- `pipeline_stage_duration_seconds{stage="..."}`
- `search_provider_latency_seconds{provider="..."}`
- `llm_tokens_used{model="..."}`
- `cache_hit_rate`
- `source_fetch_success_rate`
