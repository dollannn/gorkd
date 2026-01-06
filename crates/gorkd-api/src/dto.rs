use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateResearchRequest {
    #[schema(
        example = "What caused the 2024 CrowdStrike outage?",
        min_length = 1,
        max_length = 2000
    )]
    pub query: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateResearchResponse {
    #[schema(example = "job_abc123xyz456")]
    pub job_id: String,
    pub status: JobStatus,
    #[schema(example = "/v1/jobs/job_abc123xyz456/stream")]
    pub stream_url: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Planning,
    Searching,
    Fetching,
    Synthesizing,
    Completed,
    Failed,
}

impl From<gorkd_core::JobStatus> for JobStatus {
    fn from(status: gorkd_core::JobStatus) -> Self {
        match status {
            gorkd_core::JobStatus::Pending => Self::Pending,
            gorkd_core::JobStatus::Planning => Self::Planning,
            gorkd_core::JobStatus::Searching => Self::Searching,
            gorkd_core::JobStatus::Fetching => Self::Fetching,
            gorkd_core::JobStatus::Synthesizing => Self::Synthesizing,
            gorkd_core::JobStatus::Completed => Self::Completed,
            gorkd_core::JobStatus::Failed => Self::Failed,
            _ => Self::Pending,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JobResponse {
    #[schema(example = "job_abc123xyz456")]
    pub job_id: String,
    pub status: JobStatus,
    #[schema(example = "What caused the 2024 CrowdStrike outage?")]
    pub query: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[schema(nullable)]
    pub error_message: Option<String>,
}

impl From<gorkd_core::ResearchJob> for JobResponse {
    fn from(job: gorkd_core::ResearchJob) -> Self {
        Self {
            job_id: job.id.to_string(),
            status: job.status.into(),
            query: job.query,
            created_at: job.created_at,
            updated_at: job.updated_at,
            error_message: job.error_message,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SourceDetail {
    #[schema(example = "src_abc123xyz456")]
    pub id: String,
    #[schema(example = "https://blogs.microsoft.com/...")]
    pub url: String,
    #[schema(example = "Helping our customers through the CrowdStrike outage")]
    pub title: String,
    #[schema(example = "microsoft.com")]
    pub domain: String,
    #[schema(nullable)]
    pub published_at: Option<DateTime<Utc>>,
    pub relevance_score: f32,
}

impl From<gorkd_core::Source> for SourceDetail {
    fn from(source: gorkd_core::Source) -> Self {
        Self {
            id: source.id.to_string(),
            url: source.url,
            title: source.title,
            domain: source.metadata.domain,
            published_at: source.metadata.published_at,
            relevance_score: source.relevance_score,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JobSourceResponse {
    pub sources: Vec<SourceDetail>,
}
