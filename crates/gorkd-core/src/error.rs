use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum QueryError {
    #[error("query cannot be empty")]
    Empty,

    #[error("query exceeds maximum length of {max} characters (got {got})")]
    TooLong { max: usize, got: usize },

    #[error("query contains invalid characters")]
    InvalidCharacters,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum IdParseError {
    #[error("invalid ID prefix: expected '{expected}', got '{got}'")]
    InvalidPrefix { expected: String, got: String },

    #[error("invalid ID length: expected {expected}, got {got}")]
    InvalidLength { expected: usize, got: usize },
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ValidationError {
    #[error("invalid query: {0}")]
    Query(#[from] QueryError),

    #[error("invalid ID: {0}")]
    Id(#[from] IdParseError),

    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    #[error("missing required field: {0}")]
    MissingField(String),
}

pub const MAX_QUERY_LENGTH: usize = 2000;

pub fn validate_query(query: &str) -> Result<(), QueryError> {
    if query.is_empty() {
        return Err(QueryError::Empty);
    }

    if query.len() > MAX_QUERY_LENGTH {
        return Err(QueryError::TooLong {
            max: MAX_QUERY_LENGTH,
            got: query.len(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_query_accepts_valid_query() {
        assert!(validate_query("What is Rust?").is_ok());
    }

    #[test]
    fn validate_query_rejects_empty() {
        assert!(matches!(validate_query(""), Err(QueryError::Empty)));
    }

    #[test]
    fn validate_query_rejects_too_long() {
        let long_query = "a".repeat(MAX_QUERY_LENGTH + 1);
        assert!(matches!(
            validate_query(&long_query),
            Err(QueryError::TooLong { .. })
        ));
    }

    #[test]
    fn validate_query_accepts_max_length() {
        let max_query = "a".repeat(MAX_QUERY_LENGTH);
        assert!(validate_query(&max_query).is_ok());
    }
}
