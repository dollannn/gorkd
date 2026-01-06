use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("validation error: {0}")]
    Validation(String),

    #[error("job not found: {0}")]
    NotFound(String),

    #[error("internal error: {0}")]
    Internal(String),
}

impl AppError {
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}

impl From<gorkd_core::QueryError> for AppError {
    fn from(err: gorkd_core::QueryError) -> Self {
        Self::Validation(err.to_string())
    }
}

impl From<gorkd_core::StoreError> for AppError {
    fn from(err: gorkd_core::StoreError) -> Self {
        match err {
            gorkd_core::StoreError::JobNotFound { id } => Self::NotFound(id),
            _ => Self::Internal(err.to_string()),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiError {
    pub error: ApiErrorBody,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiErrorBody {
    #[schema(example = "validation_error")]
    pub code: String,
    #[schema(example = "Query cannot be empty")]
    pub message: String,
}

impl ApiError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: ApiErrorBody {
                code: code.into(),
                message: message.into(),
            },
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            Self::Validation(_) => (StatusCode::BAD_REQUEST, "validation_error"),
            Self::NotFound(_) => (StatusCode::NOT_FOUND, "not_found"),
            Self::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
        };

        let body = ApiError::new(code, self.to_string());
        (status, Json(body)).into_response()
    }
}
