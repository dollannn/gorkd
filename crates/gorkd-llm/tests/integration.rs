//! Integration tests for LLM providers.
//!
//! These tests make real API calls and require valid API keys.
//! Run with: `cargo test -p gorkd-llm -- --ignored --test-threads=1`

mod fixtures;

use std::collections::HashSet;
use std::sync::Arc;

use gorkd_core::{Confidence, LlmError, LlmProvider, Source};
use gorkd_llm::anthropic::types::{MODEL_CLAUDE_HAIKU_35, MODEL_CLAUDE_SONNET_4};
use gorkd_llm::openai::types::{MODEL_GPT_4O, MODEL_GPT_4O_MINI};
use gorkd_llm::{
    AnthropicConfig, AnthropicProvider, LlmConfig, LlmRegistry, OpenAiConfig, OpenAiProvider,
};
use reqwest::Client;
use secrecy::SecretString;

fn get_anthropic_api_key() -> Option<String> {
    std::env::var("ANTHROPIC_API_KEY").ok()
}

fn get_openai_api_key() -> Option<String> {
    std::env::var("OPENAI_API_KEY").ok()
}

fn create_http_client() -> Client {
    Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .expect("failed to create HTTP client")
}

fn create_anthropic_provider(model: &str) -> Option<AnthropicProvider> {
    let api_key = get_anthropic_api_key()?;
    let config = AnthropicConfig {
        api_key: SecretString::from(api_key),
        base_url: "https://api.anthropic.com".to_string(),
    };
    Some(AnthropicProvider::new(create_http_client(), &config, model))
}

fn create_openai_provider(model: &str) -> Option<OpenAiProvider> {
    let api_key = get_openai_api_key()?;
    let config = OpenAiConfig {
        api_key: SecretString::from(api_key),
        base_url: "https://api.openai.com".to_string(),
    };
    Some(OpenAiProvider::new(create_http_client(), &config, model))
}

fn assert_valid_answer(
    answer: &gorkd_core::ResearchAnswer,
    sources: &[Source],
    expected_model_substring: &str,
) {
    assert!(!answer.summary.is_empty(), "summary should not be empty");
    assert!(!answer.detail.is_empty(), "detail should not be empty");
    assert!(
        answer
            .synthesis_metadata
            .model
            .contains(expected_model_substring),
        "model should contain '{}', got '{}'",
        expected_model_substring,
        answer.synthesis_metadata.model
    );
    assert!(
        answer.synthesis_metadata.tokens_used > 0,
        "should report token usage"
    );

    let source_ids: HashSet<_> = sources.iter().map(|s| s.id.as_str()).collect();
    for citation in &answer.citations {
        assert!(
            source_ids.contains(citation.source_id.as_str()),
            "citation source_id '{}' not in provided sources",
            citation.source_id.as_str()
        );
    }
}

#[tokio::test]
#[ignore = "requires ANTHROPIC_API_KEY"]
async fn anthropic_sonnet_synthesizes_answer() {
    let Some(provider) = create_anthropic_provider(MODEL_CLAUDE_SONNET_4) else {
        eprintln!("Skipping: ANTHROPIC_API_KEY not set");
        return;
    };

    let sources = fixtures::minimal_sources();
    let answer = provider
        .synthesize(fixtures::SIMPLE_QUERY, &sources)
        .await
        .expect("synthesis should succeed");

    assert_valid_answer(&answer, &sources, "claude");
    assert!(
        answer.is_answerable(),
        "should be answerable with valid sources"
    );
}

#[tokio::test]
#[ignore = "requires ANTHROPIC_API_KEY"]
async fn anthropic_haiku_synthesizes_answer() {
    let Some(provider) = create_anthropic_provider(MODEL_CLAUDE_HAIKU_35) else {
        eprintln!("Skipping: ANTHROPIC_API_KEY not set");
        return;
    };

    let sources = fixtures::minimal_sources();
    let answer = provider
        .synthesize(fixtures::SIMPLE_QUERY, &sources)
        .await
        .expect("synthesis should succeed");

    assert_valid_answer(&answer, &sources, "haiku");
}

#[tokio::test]
#[ignore = "requires OPENAI_API_KEY"]
async fn openai_gpt4o_synthesizes_answer() {
    let Some(provider) = create_openai_provider(MODEL_GPT_4O) else {
        eprintln!("Skipping: OPENAI_API_KEY not set");
        return;
    };

    let sources = fixtures::minimal_sources();
    let answer = provider
        .synthesize(fixtures::SIMPLE_QUERY, &sources)
        .await
        .expect("synthesis should succeed");

    assert_valid_answer(&answer, &sources, "gpt-4o");
    assert!(answer.is_answerable());
}

#[tokio::test]
#[ignore = "requires OPENAI_API_KEY"]
async fn openai_gpt4o_mini_synthesizes_answer() {
    let Some(provider) = create_openai_provider(MODEL_GPT_4O_MINI) else {
        eprintln!("Skipping: OPENAI_API_KEY not set");
        return;
    };

    let sources = fixtures::minimal_sources();
    let answer = provider
        .synthesize(fixtures::SIMPLE_QUERY, &sources)
        .await
        .expect("synthesis should succeed");

    assert_valid_answer(&answer, &sources, "gpt-4o-mini");
}

#[tokio::test]
#[ignore = "requires ANTHROPIC_API_KEY"]
async fn anthropic_handles_empty_sources() {
    let Some(provider) = create_anthropic_provider(MODEL_CLAUDE_HAIKU_35) else {
        eprintln!("Skipping: ANTHROPIC_API_KEY not set");
        return;
    };

    let sources = fixtures::empty_sources();
    let answer = provider
        .synthesize(fixtures::SIMPLE_QUERY, &sources)
        .await
        .expect("synthesis should succeed even with no sources");

    assert_eq!(
        answer.confidence,
        Confidence::Insufficient,
        "should report insufficient confidence with no sources"
    );
}

#[tokio::test]
#[ignore = "requires OPENAI_API_KEY"]
async fn openai_handles_empty_sources() {
    let Some(provider) = create_openai_provider(MODEL_GPT_4O_MINI) else {
        eprintln!("Skipping: OPENAI_API_KEY not set");
        return;
    };

    let sources = fixtures::empty_sources();
    let answer = provider
        .synthesize(fixtures::SIMPLE_QUERY, &sources)
        .await
        .expect("synthesis should succeed even with no sources");

    assert_eq!(
        answer.confidence,
        Confidence::Insufficient,
        "should report insufficient confidence with no sources"
    );
}

#[tokio::test]
#[ignore = "requires ANTHROPIC_API_KEY"]
async fn anthropic_handles_many_sources() {
    let Some(provider) = create_anthropic_provider(MODEL_CLAUDE_HAIKU_35) else {
        eprintln!("Skipping: ANTHROPIC_API_KEY not set");
        return;
    };

    let sources = fixtures::many_sources();
    let answer = provider
        .synthesize(fixtures::SIMPLE_QUERY, &sources)
        .await
        .expect("synthesis should succeed with many sources");

    assert_valid_answer(&answer, &sources, "haiku");
}

#[tokio::test]
#[ignore = "requires OPENAI_API_KEY"]
async fn openai_handles_many_sources() {
    let Some(provider) = create_openai_provider(MODEL_GPT_4O_MINI) else {
        eprintln!("Skipping: OPENAI_API_KEY not set");
        return;
    };

    let sources = fixtures::many_sources();
    let answer = provider
        .synthesize(fixtures::SIMPLE_QUERY, &sources)
        .await
        .expect("synthesis should succeed with many sources");

    assert_valid_answer(&answer, &sources, "gpt-4o-mini");
}

#[tokio::test]
#[ignore = "requires ANTHROPIC_API_KEY"]
async fn anthropic_validates_citations() {
    let Some(provider) = create_anthropic_provider(MODEL_CLAUDE_HAIKU_35) else {
        eprintln!("Skipping: ANTHROPIC_API_KEY not set");
        return;
    };

    let sources = fixtures::citation_test_sources();
    let answer = provider
        .synthesize(fixtures::MULTI_SOURCE_QUERY, &sources)
        .await
        .expect("synthesis should succeed");

    let source_ids: HashSet<_> = sources.iter().map(|s| s.id.as_str()).collect();

    for citation in &answer.citations {
        assert!(
            source_ids.contains(citation.source_id.as_str()),
            "citation references unknown source: {}",
            citation.source_id.as_str()
        );
        assert!(
            !citation.claim.is_empty(),
            "citation should have a non-empty claim"
        );
    }
}

#[tokio::test]
#[ignore = "requires OPENAI_API_KEY"]
async fn openai_validates_citations() {
    let Some(provider) = create_openai_provider(MODEL_GPT_4O_MINI) else {
        eprintln!("Skipping: OPENAI_API_KEY not set");
        return;
    };

    let sources = fixtures::citation_test_sources();
    let answer = provider
        .synthesize(fixtures::MULTI_SOURCE_QUERY, &sources)
        .await
        .expect("synthesis should succeed");

    let source_ids: HashSet<_> = sources.iter().map(|s| s.id.as_str()).collect();

    for citation in &answer.citations {
        assert!(
            source_ids.contains(citation.source_id.as_str()),
            "citation references unknown source: {}",
            citation.source_id.as_str()
        );
    }
}

#[tokio::test]
async fn invalid_anthropic_key_returns_auth_error() {
    let config = AnthropicConfig {
        api_key: SecretString::from("sk-invalid-key-12345"),
        base_url: "https://api.anthropic.com".to_string(),
    };
    let provider = AnthropicProvider::new(create_http_client(), &config, MODEL_CLAUDE_HAIKU_35);

    let sources = fixtures::minimal_sources();
    let result = provider.synthesize(fixtures::SIMPLE_QUERY, &sources).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        LlmError::Provider(msg) => {
            assert!(
                msg.contains("invalid") || msg.contains("API key") || msg.contains("auth"),
                "error should mention auth issue: {}",
                msg
            );
        }
        _ => panic!("expected Provider error, got {:?}", err),
    }
}

#[tokio::test]
async fn invalid_openai_key_returns_auth_error() {
    let config = OpenAiConfig {
        api_key: SecretString::from("sk-invalid-key-12345"),
        base_url: "https://api.openai.com".to_string(),
    };
    let provider = OpenAiProvider::new(create_http_client(), &config, MODEL_GPT_4O_MINI);

    let sources = fixtures::minimal_sources();
    let result = provider.synthesize(fixtures::SIMPLE_QUERY, &sources).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        LlmError::Provider(msg) => {
            assert!(
                msg.contains("invalid") || msg.contains("API key") || msg.contains("auth"),
                "error should mention auth issue: {}",
                msg
            );
        }
        _ => panic!("expected Provider error, got {:?}", err),
    }
}

#[tokio::test]
#[ignore = "requires ANTHROPIC_API_KEY and OPENAI_API_KEY"]
async fn registry_fallback_on_rate_limit() {
    let anthropic_key = match get_anthropic_api_key() {
        Some(k) => k,
        None => {
            eprintln!("Skipping: ANTHROPIC_API_KEY not set");
            return;
        }
    };
    let openai_key = match get_openai_api_key() {
        Some(k) => k,
        None => {
            eprintln!("Skipping: OPENAI_API_KEY not set");
            return;
        }
    };

    let http = create_http_client();

    let anthropic_config = AnthropicConfig {
        api_key: SecretString::from(anthropic_key),
        base_url: "https://api.anthropic.com".to_string(),
    };
    let openai_config = OpenAiConfig {
        api_key: SecretString::from(openai_key),
        base_url: "https://api.openai.com".to_string(),
    };

    let anthropic = AnthropicProvider::new(http.clone(), &anthropic_config, MODEL_CLAUDE_HAIKU_35);
    let openai = OpenAiProvider::new(http.clone(), &openai_config, MODEL_GPT_4O_MINI);

    let registry = LlmRegistry::builder()
        .register(MODEL_CLAUDE_HAIKU_35, Arc::new(anthropic))
        .register(MODEL_GPT_4O_MINI, Arc::new(openai))
        .default_model(MODEL_CLAUDE_HAIKU_35)
        .fallback_model(MODEL_GPT_4O_MINI)
        .build();

    let sources = fixtures::minimal_sources();

    let answer = registry
        .synthesize_with_fallback(fixtures::SIMPLE_QUERY, &sources, None)
        .await
        .expect("registry synthesis should succeed");

    assert!(answer.is_answerable());
    assert_valid_answer(&answer, &sources, "");
}

#[tokio::test]
#[ignore = "requires ANTHROPIC_API_KEY or OPENAI_API_KEY"]
async fn registry_from_config_integration() {
    let config = LlmConfig::from_env();

    if !config.has_provider() {
        eprintln!("Skipping: no LLM providers configured");
        return;
    }

    let http = create_http_client();
    let registry = LlmRegistry::from_config(http, &config);

    assert!(!registry.is_empty(), "registry should have providers");

    let sources = fixtures::minimal_sources();
    let answer = registry
        .synthesize_with_fallback(fixtures::SIMPLE_QUERY, &sources, None)
        .await
        .expect("synthesis should succeed");

    assert!(answer.is_answerable());
}
