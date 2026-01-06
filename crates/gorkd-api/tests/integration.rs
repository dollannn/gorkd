use std::sync::Arc;
use std::time::Duration;

use axum_test::TestServer;
use gorkd_api::{app, AppState};
use gorkd_core::{MockLlmProvider, MockSearchProvider, MockStore};
use serde_json::{json, Value};

fn create_test_app() -> TestServer {
    let store = Arc::new(MockStore::new());
    let search_provider = Arc::new(MockSearchProvider::new("mock-tavily"));
    let llm_provider = Arc::new(MockLlmProvider::new("mock-gpt-4"));

    let state = Arc::new(AppState::new(store, search_provider, llm_provider));
    let app = app(state);

    TestServer::new(app).unwrap()
}

#[cfg(feature = "integration")]
fn create_real_provider_app() -> Option<TestServer> {
    use gorkd_search::{ProviderRegistry, SearchConfig};

    let config = SearchConfig::from_env().ok()?;
    let registry = ProviderRegistry::from_config(&config);

    if registry.is_empty() {
        return None;
    }

    let store = Arc::new(MockStore::new());
    let llm_provider = Arc::new(MockLlmProvider::new("mock-gpt-4"));
    let state = Arc::new(AppState::with_registry(store, registry, llm_provider));

    Some(TestServer::new(app(state)).unwrap())
}

#[tokio::test]
async fn test_health_check() {
    let server = create_test_app();

    let response = server.get("/health").await;

    response.assert_status_ok();
    let body: Value = response.json();
    assert_eq!(body["status"], "healthy");
    assert!(body["version"].as_str().is_some());
}

#[tokio::test]
async fn test_research_happy_path() {
    let server = create_test_app();

    let response = server
        .post("/v1/research")
        .json(&json!({"query": "What is Rust?"}))
        .await;

    response.assert_status(axum::http::StatusCode::ACCEPTED);

    let body: Value = response.json();
    let job_id = body["job_id"].as_str().unwrap();
    assert!(job_id.starts_with("job_"));
    assert_eq!(body["status"], "pending");

    tokio::time::sleep(Duration::from_millis(100)).await;

    let response = server.get(&format!("/v1/jobs/{}", job_id)).await;
    response.assert_status_ok();

    let job: Value = response.json();
    assert_eq!(job["job_id"], job_id);
    assert!([
        "pending",
        "planning",
        "searching",
        "synthesizing",
        "completed"
    ]
    .contains(&job["status"].as_str().unwrap()));
}

#[tokio::test]
async fn test_invalid_query_empty() {
    let server = create_test_app();

    let response = server
        .post("/v1/research")
        .json(&json!({"query": ""}))
        .await;

    response.assert_status(axum::http::StatusCode::BAD_REQUEST);

    let body: Value = response.json();
    assert!(body["error"]["code"].as_str().is_some());
}

#[tokio::test]
async fn test_invalid_query_too_long() {
    let server = create_test_app();

    let long_query = "x".repeat(2001);
    let response = server
        .post("/v1/research")
        .json(&json!({"query": long_query}))
        .await;

    response.assert_status(axum::http::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_job_not_found() {
    let server = create_test_app();

    let response = server.get("/v1/jobs/job_abcdef123456").await;

    response.assert_status(axum::http::StatusCode::NOT_FOUND);

    let body: Value = response.json();
    assert_eq!(body["error"]["code"], "not_found");
}

#[tokio::test]
async fn test_invalid_job_id_format() {
    let server = create_test_app();

    let response = server.get("/v1/jobs/invalid-format").await;

    response.assert_status(axum::http::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_get_sources() {
    let server = create_test_app();

    let response = server
        .post("/v1/research")
        .json(&json!({"query": "What is Rust?"}))
        .await;

    let body: Value = response.json();
    let job_id = body["job_id"].as_str().unwrap();

    tokio::time::sleep(Duration::from_millis(200)).await;

    let response = server.get(&format!("/v1/jobs/{}/sources", job_id)).await;
    response.assert_status_ok();

    let body: Value = response.json();
    assert!(body["sources"].is_array());
}

#[tokio::test]
async fn test_pipeline_completes() {
    let server = create_test_app();

    let response = server
        .post("/v1/research")
        .json(&json!({"query": "What is Rust programming language?"}))
        .await;

    let body: Value = response.json();
    let job_id = body["job_id"].as_str().unwrap();

    let mut completed = false;
    for _ in 0..20 {
        tokio::time::sleep(Duration::from_millis(50)).await;

        let response = server.get(&format!("/v1/jobs/{}", job_id)).await;
        let job: Value = response.json();

        if job["status"] == "completed" {
            completed = true;
            break;
        }
        if job["status"] == "failed" {
            panic!("Pipeline failed: {:?}", job["error_message"]);
        }
    }

    assert!(completed, "Pipeline did not complete within timeout");
}

#[cfg(feature = "integration")]
mod real_provider_tests {
    use super::*;

    #[tokio::test]
    async fn test_research_with_real_providers() {
        let Some(server) = create_real_provider_app() else {
            eprintln!("Skipping: no search providers configured");
            return;
        };

        let response = server
            .post("/v1/research")
            .json(&json!({"query": "What is the Rust programming language?"}))
            .await;

        response.assert_status(axum::http::StatusCode::ACCEPTED);

        let body: Value = response.json();
        let job_id = body["job_id"].as_str().unwrap();

        let mut completed = false;
        for _ in 0..60 {
            tokio::time::sleep(Duration::from_millis(500)).await;

            let response = server.get(&format!("/v1/jobs/{}", job_id)).await;
            let job: Value = response.json();

            match job["status"].as_str() {
                Some("completed") => {
                    completed = true;
                    break;
                }
                Some("failed") => {
                    panic!(
                        "Pipeline failed: {:?}",
                        job["error_message"].as_str().unwrap_or("unknown")
                    );
                }
                _ => continue,
            }
        }

        assert!(completed, "Pipeline did not complete within 30s timeout");

        let sources_response = server.get(&format!("/v1/jobs/{}/sources", job_id)).await;
        sources_response.assert_status_ok();

        let sources: Value = sources_response.json();
        let sources_array = sources["sources"].as_array().unwrap();
        assert!(!sources_array.is_empty(), "should have real sources");

        for source in sources_array {
            assert!(source["url"].as_str().is_some(), "source should have URL");
            assert!(
                source["title"].as_str().is_some(),
                "source should have title"
            );
        }
    }

    #[tokio::test]
    async fn test_provider_fallback_on_invalid_query() {
        let Some(server) = create_real_provider_app() else {
            eprintln!("Skipping: no search providers configured");
            return;
        };

        let response = server
            .post("/v1/research")
            .json(&json!({"query": "test query for fallback verification"}))
            .await;

        response.assert_status(axum::http::StatusCode::ACCEPTED);

        let body: Value = response.json();
        assert!(body["job_id"].as_str().is_some());
    }
}
