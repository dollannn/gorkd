use std::fmt;

use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[non_exhaustive]
pub enum SearchError {
    #[error("provider not available: {provider}")]
    ProviderUnavailable { provider: String },

    #[error("rate limited by provider: {provider}")]
    RateLimited { provider: String },

    #[error("search timeout after {timeout_secs}s")]
    Timeout { timeout_secs: u64 },

    #[error("invalid query: {reason}")]
    InvalidQuery { reason: String },

    #[error("network error: {0}")]
    Network(String),

    #[error("provider error: {0}")]
    Provider(String),
}

#[derive(Debug, Clone, Error)]
#[non_exhaustive]
pub enum LlmError {
    #[error("model not available: {model}")]
    ModelUnavailable { model: String },

    #[error("rate limited by provider")]
    RateLimited,

    #[error("context length exceeded: {max_tokens} tokens max, got {got_tokens}")]
    ContextLengthExceeded {
        max_tokens: usize,
        got_tokens: usize,
    },

    #[error("content filtered: {reason}")]
    ContentFiltered { reason: String },

    #[error("synthesis timeout after {timeout_secs}s")]
    Timeout { timeout_secs: u64 },

    #[error("network error: {0}")]
    Network(String),

    #[error("provider error: {0}")]
    Provider(String),
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StoreError {
    #[error("job not found: {id}")]
    JobNotFound { id: String },

    #[error("connection failed: {0}")]
    Connection(String),

    #[error("query failed: {0}")]
    Query(String),

    #[error("serialization failed: {0}")]
    Serialization(String),

    #[error("conflict: {0}")]
    Conflict(String),
}

impl SearchError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::ProviderUnavailable { .. }
                | Self::RateLimited { .. }
                | Self::Timeout { .. }
                | Self::Network(_)
        )
    }
}

impl LlmError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::ModelUnavailable { .. }
                | Self::RateLimited
                | Self::Timeout { .. }
                | Self::Network(_)
        )
    }
}

impl StoreError {
    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::Connection(_))
    }
}

#[derive(Debug)]
pub struct ErrorContext {
    pub operation: String,
    pub details: Option<String>,
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "during {}", self.operation)?;
        if let Some(ref details) = self.details {
            write!(f, ": {}", details)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_error_is_retryable() {
        assert!(SearchError::RateLimited {
            provider: "tavily".into()
        }
        .is_retryable());
        assert!(SearchError::Timeout { timeout_secs: 30 }.is_retryable());
        assert!(!SearchError::InvalidQuery {
            reason: "empty".into()
        }
        .is_retryable());
    }

    #[test]
    fn llm_error_is_retryable() {
        assert!(LlmError::RateLimited.is_retryable());
        assert!(!LlmError::ContentFiltered {
            reason: "policy".into()
        }
        .is_retryable());
    }

    #[test]
    fn store_error_is_retryable() {
        assert!(StoreError::Connection("timeout".into()).is_retryable());
        assert!(!StoreError::JobNotFound {
            id: "job_123".into()
        }
        .is_retryable());
    }

    #[test]
    fn error_context_displays_correctly() {
        let ctx = ErrorContext {
            operation: "search".into(),
            details: Some("provider timeout".into()),
        };
        assert_eq!(ctx.to_string(), "during search: provider timeout");

        let ctx_no_details = ErrorContext {
            operation: "store".into(),
            details: None,
        };
        assert_eq!(ctx_no_details.to_string(), "during store");
    }
}
