#![forbid(unsafe_code)]

//! Core domain types and research pipeline logic.
//!
//! **No I/O**: This crate must not depend on tokio, reqwest, or any async runtime.
//! All I/O is abstracted behind traits, implemented in other crates.

mod answer;
mod error;
mod id;
mod job;
mod query;
mod search;
mod source;
mod traits;

pub use answer::{Citation, Confidence, ResearchAnswer, SynthesisMetadata};
pub use error::{IdParseError, QueryError, ValidationError, MAX_QUERY_LENGTH};
pub use id::{JobId, SourceId};
pub use job::{JobStatus, ResearchJob};
pub use query::{QueryIntent, QuestionType, TimeConstraint};
pub use search::{
    ContentType, ProviderId, Recency, SearchFilters, SearchPlan, SearchQuery, DEFAULT_MAX_SOURCES,
    DEFAULT_TIMEOUT_SECS,
};
pub use source::{SearchMetadata, Source, SourceCollection, SourceMetadata};
