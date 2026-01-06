#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Storage implementations (Postgres, vector DB).

/// PostgreSQL job storage.
pub mod postgres;
/// Vector storage for embeddings.
pub mod vector;
