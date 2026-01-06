# Phase 2: Core Domain Types

**Status**: Completed
**Duration**: ~1-2 days
**Depends on**: Phase 1

## Overview

Implement all domain types in gorkd-core as documented in research-pipeline.md. Pure data structures, no I/O, no async.

## Tasks

| Task | Description |
|------|-------------|
| Implement ID types | JobId, SourceId with nanoid generation and prefixes |
| Implement ResearchJob | Core job struct with status tracking |
| Implement QueryIntent | Question type, entities, time constraints |
| Implement SearchPlan | Search queries and strategy |
| Implement Source types | Source, SourceMetadata, SourceCollection |
| Implement Answer types | ResearchAnswer, Citation, Confidence |
| Implement error types | QueryError, ValidationError |
| Implement JobStatus enum | All pipeline states |
| Add serde derives | JSON serialization for all types |
| Add builder patterns | For complex types (optional) |
| Write unit tests | Validation, serialization roundtrips |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-core/src/lib.rs` | Module declarations, re-exports |
| `crates/gorkd-core/src/id.rs` | **New file** - JobId, SourceId types |
| `crates/gorkd-core/src/job.rs` | **New file** - ResearchJob, JobStatus |
| `crates/gorkd-core/src/query.rs` | **New file** - QueryIntent, QuestionType, TimeConstraint |
| `crates/gorkd-core/src/search.rs` | **New file** - SearchPlan, SearchQuery, SearchFilters |
| `crates/gorkd-core/src/source.rs` | **New file** - Source, SourceMetadata, SourceCollection |
| `crates/gorkd-core/src/answer.rs` | **New file** - ResearchAnswer, Citation, Confidence |
| `crates/gorkd-core/src/error.rs` | **New file** - Error types |

## Type Specifications

### IDs
- `JobId`: newtype around String, prefix `job_`, 12 char nanoid
- `SourceId`: newtype around String, prefix `src_`, 12 char nanoid
- Implement `Display`, `FromStr`, `Serialize`, `Deserialize`

### JobStatus
```
Pending -> Planning -> Searching -> Fetching -> Synthesizing -> Completed
                                                            \-> Failed
```

### Confidence
```
High | Medium | Low | Insufficient
```

### QuestionType
```
Factual | Comparison | Explanation | CurrentEvent | HowTo | Opinion
```

## Key Considerations

- All types must be `Clone`, `Debug`
- Public types need `Serialize`, `Deserialize`
- No `Default` for types that require meaningful values
- Use `#[non_exhaustive]` on enums for future extensibility
- Validation in constructors, not setters

## Deliverables

- [x] All types from research-pipeline.md implemented
- [x] `cargo test -p gorkd-core` passes (42 tests)
- [x] `cargo doc -p gorkd-core` generates clean docs
- [x] No `unwrap()` or `panic!()` in library code
- [x] All public items have self-documenting names
