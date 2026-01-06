use utoipa::OpenApi;

use crate::dto::{
    CreateResearchRequest, CreateResearchResponse, JobResponse, JobSourceResponse, JobStatus,
    SourceDetail,
};
use crate::error::{ApiError, ApiErrorBody};
use crate::routes::health::HealthResponse;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "gorkd API",
        version = "0.1.0",
        description = "Truth-seeking research bot API",
        license(name = "MIT")
    ),
    tags(
        (name = "research", description = "Research operations"),
        (name = "jobs", description = "Job management"),
        (name = "health", description = "Health checks")
    ),
    components(schemas(
        CreateResearchRequest,
        CreateResearchResponse,
        JobResponse,
        JobSourceResponse,
        SourceDetail,
        JobStatus,
        ApiError,
        ApiErrorBody,
        HealthResponse,
    ))
)]
pub struct ApiDoc;
