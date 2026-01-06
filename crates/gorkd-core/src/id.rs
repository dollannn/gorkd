use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::IdParseError;

/// Length of the nanoid suffix for all public IDs.
pub const NANOID_LENGTH: usize = 12;

/// Macro to define a strongly-typed ID with prefix pattern.
///
/// Generates a newtype struct with:
/// - `new()` - creates ID with nanoid
/// - `as_str()` - returns the full ID string
/// - `prefix()` - returns the prefix (e.g., "job_")
/// - `Clone`, `Debug`, `PartialEq`, `Eq`, `Hash`
/// - `Serialize`, `Deserialize` (transparent)
/// - `Display`, `Default`, `FromStr`
///
/// # Example
///
/// ```ignore
/// define_id!(JobId, "job_");
/// define_id!(MessageId, "msg_");
///
/// let job = JobId::new();
/// assert!(job.as_str().starts_with("job_"));
/// ```
#[macro_export]
macro_rules! define_id {
    ($name:ident, $prefix:literal) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Creates a new ID with a random nanoid suffix.
            pub fn new() -> Self {
                Self(format!("{}{}", $prefix, nanoid::nanoid!(NANOID_LENGTH)))
            }

            /// Returns the full ID as a string slice.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Returns the prefix for this ID type.
            pub const fn prefix() -> &'static str {
                $prefix
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl FromStr for $name {
            type Err = IdParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if !s.starts_with($prefix) {
                    return Err(IdParseError::InvalidPrefix {
                        expected: $prefix.to_string(),
                        got: s.to_string(),
                    });
                }

                let suffix = &s[$prefix.len()..];
                if suffix.len() != NANOID_LENGTH {
                    return Err(IdParseError::InvalidLength {
                        expected: $prefix.len() + NANOID_LENGTH,
                        got: s.len(),
                    });
                }

                Ok(Self(s.to_string()))
            }
        }
    };
}

define_id!(JobId, "job_");
define_id!(SourceId, "src_");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_id_has_correct_prefix() {
        let id = JobId::new();
        assert!(id.as_str().starts_with(JobId::prefix()));
    }

    #[test]
    fn job_id_has_correct_length() {
        let id = JobId::new();
        assert_eq!(id.as_str().len(), JobId::prefix().len() + NANOID_LENGTH);
    }

    #[test]
    fn job_id_parses_valid_string() {
        let id = JobId::new();
        let parsed: JobId = id.to_string().parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn job_id_rejects_wrong_prefix() {
        let result: Result<JobId, _> = "src_abc123abc123".parse();
        assert!(matches!(result, Err(IdParseError::InvalidPrefix { .. })));
    }

    #[test]
    fn job_id_rejects_wrong_length() {
        let result: Result<JobId, _> = "job_short".parse();
        assert!(matches!(result, Err(IdParseError::InvalidLength { .. })));
    }

    #[test]
    fn source_id_has_correct_prefix() {
        let id = SourceId::new();
        assert!(id.as_str().starts_with(SourceId::prefix()));
    }

    #[test]
    fn source_id_has_correct_length() {
        let id = SourceId::new();
        assert_eq!(id.as_str().len(), SourceId::prefix().len() + NANOID_LENGTH);
    }

    #[test]
    fn source_id_parses_valid_string() {
        let id = SourceId::new();
        let parsed: SourceId = id.to_string().parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn job_id_serializes_as_string() {
        let id = JobId::new();
        let json = serde_json::to_string(&id).unwrap();
        assert!(json.starts_with('"'));
        assert!(json.contains(JobId::prefix()));
    }

    #[test]
    fn job_id_deserializes_from_string() {
        let id = JobId::new();
        let json = serde_json::to_string(&id).unwrap();
        let parsed: JobId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }
}
