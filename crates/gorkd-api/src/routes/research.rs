use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use gorkd_core::ResearchJob;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::dto::{CreateResearchRequest, CreateResearchResponse, JobStatus};
use crate::error::{ApiError, AppError};
use crate::state::AppState;

#[utoipa::path(
    post,
    path = "/v1/research",
    tag = "research",
    request_body = CreateResearchRequest,
    responses(
        (status = 202, description = "Job created", body = CreateResearchResponse),
        (status = 400, description = "Invalid request", body = ApiError),
    )
)]
pub async fn create_research(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateResearchRequest>,
) -> Result<(StatusCode, Json<CreateResearchResponse>), AppError> {
    let job = ResearchJob::new(&req.query)?;
    let job_id = job.id.to_string();

    state.store.create_job(&job).await?;

    tracing::info!(job_id = %job_id, query = %req.query, "created research job");

    let response = CreateResearchResponse {
        job_id: job_id.clone(),
        status: JobStatus::Pending,
        stream_url: format!("/v1/jobs/{}/stream", job_id),
    };

    Ok((StatusCode::ACCEPTED, Json(response)))
}

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new().routes(routes!(create_research))
}
