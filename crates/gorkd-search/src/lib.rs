#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Search provider implementations (Tavily, Exa, SearXNG).

mod client;
mod config;
mod registry;

pub mod exa;
pub mod searxng;
pub mod tavily;

pub use client::{HttpClient, HttpClientError};
pub use config::{ConfigError, SearchConfig};
pub use exa::{ExaProvider, SearchType as ExaSearchType};
pub use gorkd_core::traits::{SearchProvider, SearchResult};
pub use registry::ProviderRegistry;
pub use tavily::{SearchDepth, TavilyProvider};
