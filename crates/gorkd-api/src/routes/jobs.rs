use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Path, State};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::IntoResponse;
use axum::Json;
use futures::stream::{self, Stream};
use gorkd_core::JobId;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::dto::{JobResponse, JobSourceResponse, SourceDetail};
use crate::error::{ApiError, AppError};
use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/v1/jobs/{id}",
    tag = "jobs",
    params(
        ("id" = String, Path, description = "Job ID")
    ),
    responses(
        (status = 200, description = "Job found", body = JobResponse),
        (status = 404, description = "Job not found", body = ApiError),
    )
)]
pub async fn get_job(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<JobResponse>, AppError> {
    let job_id: JobId = id
        .parse()
        .map_err(|_| AppError::validation("invalid job ID format"))?;

    let job = state
        .store
        .get_job(&job_id)
        .await?
        .ok_or_else(|| AppError::not_found(job_id.to_string()))?;

    Ok(Json(job.into()))
}

#[utoipa::path(
    get,
    path = "/v1/jobs/{id}/sources",
    tag = "jobs",
    params(
        ("id" = String, Path, description = "Job ID")
    ),
    responses(
        (status = 200, description = "Sources found", body = JobSourceResponse),
        (status = 404, description = "Job not found", body = ApiError),
    )
)]
pub async fn get_sources(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<JobSourceResponse>, AppError> {
    let job_id: JobId = id
        .parse()
        .map_err(|_| AppError::validation("invalid job ID format"))?;

    state
        .store
        .get_job(&job_id)
        .await?
        .ok_or_else(|| AppError::not_found(job_id.to_string()))?;

    let sources = state.store.get_sources(&job_id).await?;
    let source_details: Vec<SourceDetail> = sources.into_iter().map(Into::into).collect();

    Ok(Json(JobSourceResponse {
        sources: source_details,
    }))
}

#[utoipa::path(
    get,
    path = "/v1/jobs/{id}/stream",
    tag = "jobs",
    params(
        ("id" = String, Path, description = "Job ID")
    ),
    responses(
        (status = 200, description = "SSE stream", content_type = "text/event-stream"),
        (status = 404, description = "Job not found", body = ApiError),
    )
)]
pub async fn get_stream(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let job_id: JobId = id
        .parse()
        .map_err(|_| AppError::validation("invalid job ID format"))?;

    state
        .store
        .get_job(&job_id)
        .await?
        .ok_or_else(|| AppError::not_found(job_id.to_string()))?;

    let stream = stubbed_event_stream(job_id.to_string());

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

fn stubbed_event_stream(
    job_id: String,
) -> impl Stream<Item = Result<Event, Infallible>> + Send + 'static {
    stream::unfold(0u8, move |state| {
        let job_id = job_id.clone();
        async move {
            match state {
                0 => {
                    let event = Event::default()
                        .event("status")
                        .data(r#"{"stage":"pending","message":"Job queued..."}"#);
                    Some((Ok(event), 1))
                }
                1 => {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    let event = Event::default()
                        .event("status")
                        .data(r#"{"stage":"searching","message":"Searching sources..."}"#);
                    Some((Ok(event), 2))
                }
                2 => {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    let data = format!(r#"{{"job_id":"{}","duration_ms":1000}}"#, job_id);
                    let event = Event::default().event("complete").data(data);
                    Some((Ok(event), 3))
                }
                _ => None,
            }
        }
    })
}

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(get_job))
        .routes(routes!(get_sources))
        .routes(routes!(get_stream))
}
