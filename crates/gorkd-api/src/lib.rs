use std::sync::Arc;

use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

mod dto;
mod error;
mod openapi;
pub mod routes;
mod state;

pub use state::AppState;

use openapi::ApiDoc;

pub fn app(state: Arc<AppState>) -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(routes::health::router())
        .merge(routes::research::router())
        .merge(routes::jobs::router())
        .split_for_parts();

    router
        .merge(Scalar::with_url("/docs", api))
        .with_state(state)
}
