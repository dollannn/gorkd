use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::IdParseError;

const NANOID_LENGTH: usize = 12;
const JOB_PREFIX: &str = "job_";
const SOURCE_PREFIX: &str = "src_";

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JobId(String);

impl JobId {
    pub fn new() -> Self {
        Self(format!("{}{}", JOB_PREFIX, nanoid::nanoid!(NANOID_LENGTH)))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for JobId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for JobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for JobId {
    type Err = IdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with(JOB_PREFIX) {
            return Err(IdParseError::InvalidPrefix {
                expected: JOB_PREFIX.to_string(),
                got: s.to_string(),
            });
        }

        let suffix = &s[JOB_PREFIX.len()..];
        if suffix.len() != NANOID_LENGTH {
            return Err(IdParseError::InvalidLength {
                expected: JOB_PREFIX.len() + NANOID_LENGTH,
                got: s.len(),
            });
        }

        Ok(Self(s.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SourceId(String);

impl SourceId {
    pub fn new() -> Self {
        Self(format!(
            "{}{}",
            SOURCE_PREFIX,
            nanoid::nanoid!(NANOID_LENGTH)
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SourceId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SourceId {
    type Err = IdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with(SOURCE_PREFIX) {
            return Err(IdParseError::InvalidPrefix {
                expected: SOURCE_PREFIX.to_string(),
                got: s.to_string(),
            });
        }

        let suffix = &s[SOURCE_PREFIX.len()..];
        if suffix.len() != NANOID_LENGTH {
            return Err(IdParseError::InvalidLength {
                expected: SOURCE_PREFIX.len() + NANOID_LENGTH,
                got: s.len(),
            });
        }

        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_id_has_correct_prefix() {
        let id = JobId::new();
        assert!(id.as_str().starts_with("job_"));
    }

    #[test]
    fn job_id_has_correct_length() {
        let id = JobId::new();
        assert_eq!(id.as_str().len(), JOB_PREFIX.len() + NANOID_LENGTH);
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
        assert!(id.as_str().starts_with("src_"));
    }

    #[test]
    fn source_id_has_correct_length() {
        let id = SourceId::new();
        assert_eq!(id.as_str().len(), SOURCE_PREFIX.len() + NANOID_LENGTH);
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
        assert!(json.contains("job_"));
    }

    #[test]
    fn job_id_deserializes_from_string() {
        let id = JobId::new();
        let json = serde_json::to_string(&id).unwrap();
        let parsed: JobId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }
}
