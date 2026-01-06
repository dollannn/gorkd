# Vision

## The Problem

Finding truth online is hard. Search engines return links, not answers. LLMs hallucinate without sources. Fact-checking is manual and slow.

People need a tool that:
1. Understands what they're actually asking
2. Searches multiple sources intelligently
3. Synthesizes findings with citations
4. Admits uncertainty when appropriate
5. Works where they already are (Discord, Slack, web)

## What gorkd Is

A research assistant that treats every query as a mini-investigation:

```
User: "Did the FDA approve that new Alzheimer's drug?"

gorkd:
- Interprets: Looking for FDA approval status of recent Alzheimer's treatment
- Searches: FDA announcements, medical news, pharma press releases
- Finds: Lecanemab (Leqembi) received traditional approval July 2023
- Synthesizes: Yes, with caveats about efficacy debates
- Cites: [FDA.gov], [NEJM], [Reuters]
- Notes: "Earlier accelerated approval was controversial"
```

The answer isn't just "yes" — it's evidence-backed, cited, and contextualized.

## What gorkd Is Not

- **Not a chatbot**: No memory of past conversations, no personality, no chit-chat
- **Not a document QA tool**: Doesn't index your files — it searches the web
- **Not an agent framework**: One workflow, executed well
- **Not a search engine**: Doesn't return links — returns synthesized answers

## Core Principles

### 1. Evidence-First

Every claim must have a source. No source, no claim.

**Acceptance criteria**:
- Every response includes at least one citation
- Sources are ranked by credibility signals (domain authority, recency, corroboration)
- When sources conflict, the conflict is surfaced

### 2. Transparent Process

Show how we got the answer, not just the answer.

**Acceptance criteria**:
- User can see: search queries generated, sources fetched, sources used vs discarded
- Reasoning chain is inspectable (especially in web UI)
- Confidence level is explicit ("high confidence", "limited sources", "conflicting information")

### 3. Speed Over Perfection

A good answer in 20 seconds beats a perfect answer in 2 minutes.

**Acceptance criteria**:
- P50 response time < 15 seconds
- P95 response time < 45 seconds
- Cached queries return < 2 seconds
- Streaming partial results when possible

### 4. One Workflow

Ask → Research → Deliver. No modes, settings, or configuration for users.

**Acceptance criteria**:
- Zero onboarding needed
- No "which model" or "how many sources" choices
- System makes intelligent defaults; power users can inspect but not configure

### 5. Graceful Degradation

When we can't find good answers, say so clearly.

**Acceptance criteria**:
- "I couldn't find reliable sources for this" is a valid response
- Distinguish between "no results" and "conflicting results"
- Never fabricate citations
- Never present low-confidence claims as facts

## Success Metrics

| Metric | Target |
|--------|--------|
| Time to first useful response | < 15s (P50) |
| Citation accuracy | 100% (citations exist and support claims) |
| User can verify claim | Every claim traceable to source |
| New user to first query | < 2 minutes |
| Query to answer (cached) | < 2 seconds |

## Anti-Goals

Things we explicitly won't build:

| Anti-Goal | Why |
|-----------|-----|
| Plugin system | Complexity without clear value |
| Multi-model comparison | Confuses users, doubles cost |
| Chat memory | Different product (assistant vs research) |
| User accounts (initially) | Friction before value |
| Enterprise features | Premature optimization |
| Mobile apps | Web works on mobile |

## Target Users

1. **Curious individuals**: Want quick, reliable answers to factual questions
2. **Researchers/journalists**: Need cited sources for verification
3. **Developers**: Want a research tool in their workflow (Discord/Slack)
4. **Anyone tired of**: Opening 10 tabs, reading SEO spam, wondering what's true

## The Name

**gorkd** = Grok + d (daemon)

Inspired by Grok's truth-seeking ethos, running as a service.
