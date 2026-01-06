#![forbid(unsafe_code)]

mod answer;
mod error;
mod id;
mod job;
pub mod mock;
mod query;
mod search;
mod source;
pub mod traits;

pub use answer::{Citation, Confidence, ResearchAnswer, SynthesisMetadata};
pub use error::{IdParseError, QueryError, ValidationError, MAX_QUERY_LENGTH};
pub use id::{JobId, SourceId};
pub use job::{JobStatus, ResearchJob};
pub use mock::{MockLlmProvider, MockSearchProvider, MockStore};
pub use query::{QueryIntent, QuestionType, TimeConstraint};
pub use search::{
    ContentType, ProviderId, Recency, SearchFilters, SearchPlan, SearchQuery, DEFAULT_MAX_SOURCES,
    DEFAULT_TIMEOUT_SECS,
};
pub use source::{SearchMetadata, Source, SourceCollection, SourceMetadata};
pub use traits::{
    ErrorContext, LlmError, LlmProvider, SearchError, SearchProvider, SearchResult, Store,
    StoreError,
};
