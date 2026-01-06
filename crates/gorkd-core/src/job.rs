use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::{validate_query, QueryError};
use crate::id::JobId;
use crate::query::QueryIntent;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum JobStatus {
    Pending,
    Planning,
    Searching,
    Fetching,
    Synthesizing,
    Completed,
    Failed,
}

impl JobStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed)
    }

    pub fn is_active(&self) -> bool {
        !self.is_terminal()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResearchJob {
    pub id: JobId,
    pub query: String,
    pub intent: Option<QueryIntent>,
    pub status: JobStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub error_message: Option<String>,
}

impl ResearchJob {
    pub fn new(query: impl Into<String>) -> Result<Self, QueryError> {
        let query = query.into();
        validate_query(&query)?;

        let now = Utc::now();
        Ok(Self {
            id: JobId::new(),
            query,
            intent: None,
            status: JobStatus::Pending,
            created_at: now,
            updated_at: now,
            error_message: None,
        })
    }

    pub fn with_intent(mut self, intent: QueryIntent) -> Self {
        self.intent = Some(intent);
        self.updated_at = Utc::now();
        self
    }

    pub fn transition_to(&mut self, status: JobStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    pub fn fail(&mut self, message: impl Into<String>) {
        self.status = JobStatus::Failed;
        self.error_message = Some(message.into());
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_job_with_valid_query() {
        let job = ResearchJob::new("What is Rust?").unwrap();
        assert_eq!(job.query, "What is Rust?");
        assert_eq!(job.status, JobStatus::Pending);
        assert!(job.id.as_str().starts_with("job_"));
    }

    #[test]
    fn rejects_empty_query() {
        let result = ResearchJob::new("");
        assert!(matches!(result, Err(QueryError::Empty)));
    }

    #[test]
    fn status_is_terminal_for_completed() {
        assert!(JobStatus::Completed.is_terminal());
        assert!(JobStatus::Failed.is_terminal());
    }

    #[test]
    fn status_is_active_for_pending() {
        assert!(JobStatus::Pending.is_active());
        assert!(JobStatus::Searching.is_active());
    }

    #[test]
    fn transition_updates_status() {
        let mut job = ResearchJob::new("test").unwrap();
        job.transition_to(JobStatus::Searching);
        assert_eq!(job.status, JobStatus::Searching);
    }

    #[test]
    fn fail_sets_error_message() {
        let mut job = ResearchJob::new("test").unwrap();
        job.fail("Something went wrong");
        assert_eq!(job.status, JobStatus::Failed);
        assert_eq!(job.error_message, Some("Something went wrong".to_string()));
    }

    #[test]
    fn job_serializes_to_json() {
        let job = ResearchJob::new("What is Rust?").unwrap();
        let json = serde_json::to_string(&job).unwrap();
        assert!(json.contains("pending"));
        assert!(json.contains("What is Rust?"));
    }
}
