#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Core domain types and research pipeline logic.
//!
//! **No I/O**: This crate must not depend on tokio, reqwest, or any async runtime.
//! All I/O is abstracted behind traits, implemented in other crates.

/// Domain error types.
pub mod error;
/// Provider traits (SearchProvider, LlmProvider).
pub mod traits;
/// Domain types (ResearchJob, Source, Citation).
pub mod types;
