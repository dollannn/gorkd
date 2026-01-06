use std::net::SocketAddr;
use std::sync::Arc;

use gorkd_api::{app, AppState};
use gorkd_core::{MockLlmProvider, MockSearchProvider, MockStore};
use gorkd_llm::{default_http_client, LlmConfig, LlmRegistry};
use gorkd_search::{ProviderRegistry, SearchConfig};
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(4000);

    let store = Arc::new(MockStore::new());

    let llm_config = LlmConfig::from_env();
    let llm_registry = if llm_config.has_provider() {
        let http = default_http_client().expect("failed to create HTTP client");
        let registry = LlmRegistry::from_config(http, &llm_config);
        tracing::info!(
            models = ?registry.available_models(),
            default = ?registry.default_model_id(),
            fallback = ?registry.fallback_model_id(),
            "initialized LLM providers from environment"
        );
        registry
    } else {
        tracing::warn!("no LLM providers configured, using mock provider");
        let mock = Arc::new(MockLlmProvider::new("mock-gpt-4"));
        LlmRegistry::builder()
            .register("mock-gpt-4", mock)
            .default_model("mock-gpt-4")
            .build()
    };

    let search_registry = match SearchConfig::from_env() {
        Ok(config) => {
            let registry = ProviderRegistry::from_config(&config);
            tracing::info!(
                providers = ?registry.list(),
                "initialized search providers from environment"
            );
            registry
        }
        Err(_) => {
            tracing::warn!("no search providers configured, using mock provider");
            let mut registry = ProviderRegistry::new();
            registry.register(
                "mock-tavily",
                Arc::new(MockSearchProvider::new("mock-tavily")),
            );
            registry
        }
    };

    let state = Arc::new(AppState::with_registries(
        store,
        search_registry,
        llm_registry,
    ));

    let app = app(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);
    tracing::info!("docs available at http://localhost:{}/docs", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    tracing::info!("shutdown complete");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("shutdown signal received");
}
