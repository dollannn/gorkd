# ADR 0001: Architecture Decision Records

## Status

Accepted

## Context

As gorkd development spans multiple sessions, we need a way to:
- Remember why decisions were made
- Avoid re-litigating settled questions
- Onboard future contributors quickly
- Track technical debt intentionally created

## Decision

We will use Architecture Decision Records (ADRs) to document significant technical decisions.

### What qualifies as "significant"?

- Choice of library/framework where alternatives exist
- Data model design that affects multiple components
- API contract decisions
- Trade-offs that accept technical debt
- Decisions that constrain future options

### What doesn't need an ADR?

- Implementation details within a single file
- Obvious choices with no real alternatives
- Temporary scaffolding that will be replaced

### Format

Each ADR follows this template:

```markdown
# ADR NNNN: Title

## Status

[Proposed | Accepted | Deprecated | Superseded by ADR-NNNN]

## Context

What is the issue that we're seeing that is motivating this decision?

## Decision

What is the change that we're proposing and/or doing?

## Consequences

What becomes easier or more difficult because of this change?
```

### Naming

- Files: `NNNN-short-title.md` (e.g., `0002-use-axum.md`)
- Sequential numbering
- Lowercase, hyphen-separated

### Lifecycle

1. **Proposed**: Draft, open for discussion
2. **Accepted**: Decision made, implementation proceeds
3. **Deprecated**: No longer relevant (kept for history)
4. **Superseded**: Replaced by newer ADR (link to it)

## Consequences

**Benefits:**
- Decisions are discoverable and searchable
- New contributors understand the "why"
- Prevents decision amnesia between sessions

**Costs:**
- Small overhead to write ADRs
- Must remember to create them

**Mitigation:**
- Keep ADRs short (aim for <1 page)
- Write them when making the decision, not after
- Reference ADRs from relevant code comments
