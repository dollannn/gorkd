use std::collections::HashMap;
use std::sync::Arc;

use gorkd_core::{LlmError, LlmProvider, ResearchAnswer, Source};
use reqwest::Client;
use tracing::{info, warn};

use crate::anthropic::types::{MODEL_CLAUDE_HAIKU_35, MODEL_CLAUDE_SONNET_4};
use crate::config::LlmConfig;
use crate::openai::types::{MODEL_GPT_4O, MODEL_GPT_4O_MINI};
use crate::{AnthropicProvider, OpenAiProvider};

#[derive(Clone)]
pub struct LlmRegistry {
    providers: HashMap<String, Arc<dyn LlmProvider>>,
    default_model: Option<String>,
    fallback_model: Option<String>,
}

impl Default for LlmRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl LlmRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            default_model: None,
            fallback_model: None,
        }
    }

    pub fn builder() -> LlmRegistryBuilder {
        LlmRegistryBuilder::new()
    }

    pub fn from_config(http: Client, config: &LlmConfig) -> Self {
        let mut builder = Self::builder();

        if let Some(ref anthropic_config) = config.anthropic {
            let sonnet =
                AnthropicProvider::new(http.clone(), anthropic_config, MODEL_CLAUDE_SONNET_4);
            builder = builder.register(MODEL_CLAUDE_SONNET_4, Arc::new(sonnet));
            info!(
                model = MODEL_CLAUDE_SONNET_4,
                provider = "anthropic",
                "registered LLM provider"
            );

            let haiku =
                AnthropicProvider::new(http.clone(), anthropic_config, MODEL_CLAUDE_HAIKU_35);
            builder = builder.register(MODEL_CLAUDE_HAIKU_35, Arc::new(haiku));
            info!(
                model = MODEL_CLAUDE_HAIKU_35,
                provider = "anthropic",
                "registered LLM provider"
            );
        }

        if let Some(ref openai_config) = config.openai {
            let gpt4o = OpenAiProvider::new(http.clone(), openai_config, MODEL_GPT_4O);
            builder = builder.register(MODEL_GPT_4O, Arc::new(gpt4o));
            info!(
                model = MODEL_GPT_4O,
                provider = "openai",
                "registered LLM provider"
            );

            let gpt4o_mini = OpenAiProvider::new(http.clone(), openai_config, MODEL_GPT_4O_MINI);
            builder = builder.register(MODEL_GPT_4O_MINI, Arc::new(gpt4o_mini));
            info!(
                model = MODEL_GPT_4O_MINI,
                provider = "openai",
                "registered LLM provider"
            );
        }

        builder = builder.default_model(&config.default_model);

        if let Some(ref fallback) = config.fallback_model {
            builder = builder.fallback_model(fallback);
        }

        builder.build()
    }

    pub fn register(&mut self, model_id: impl Into<String>, provider: Arc<dyn LlmProvider>) {
        let model_id = model_id.into();
        if self.default_model.is_none() {
            self.default_model = Some(model_id.clone());
        }
        self.providers.insert(model_id, provider);
    }

    pub fn set_default(&mut self, model_id: impl Into<String>) {
        self.default_model = Some(model_id.into());
    }

    pub fn set_fallback(&mut self, model_id: impl Into<String>) {
        self.fallback_model = Some(model_id.into());
    }

    pub fn get(&self, model_id: &str) -> Option<Arc<dyn LlmProvider>> {
        self.providers.get(model_id).cloned()
    }

    pub fn default(&self) -> Option<Arc<dyn LlmProvider>> {
        self.default_model.as_ref().and_then(|id| self.get(id))
    }

    pub fn fallback(&self) -> Option<Arc<dyn LlmProvider>> {
        self.fallback_model.as_ref().and_then(|id| self.get(id))
    }

    pub fn default_model_id(&self) -> Option<&str> {
        self.default_model.as_deref()
    }

    pub fn fallback_model_id(&self) -> Option<&str> {
        self.fallback_model.as_deref()
    }

    pub fn available_models(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }

    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    pub fn len(&self) -> usize {
        self.providers.len()
    }

    pub async fn synthesize_with_fallback(
        &self,
        query: &str,
        sources: &[Source],
        model_id: Option<&str>,
    ) -> Result<ResearchAnswer, LlmError> {
        let primary = match model_id {
            Some(id) => self.get(id).ok_or_else(|| LlmError::ModelUnavailable {
                model: id.to_string(),
            })?,
            None => self.default().ok_or_else(|| LlmError::ModelUnavailable {
                model: "no default model configured".to_string(),
            })?,
        };

        let primary_model_id = primary.model_id().to_string();

        match primary.synthesize(query, sources).await {
            Ok(answer) => Ok(answer),
            Err(err) if err.is_retryable() => {
                if let Some(fallback) = self.fallback() {
                    if fallback.model_id() != primary_model_id {
                        warn!(
                            primary_model = %primary_model_id,
                            fallback_model = %fallback.model_id(),
                            error = %err,
                            "primary LLM failed, attempting fallback"
                        );
                        return fallback.synthesize(query, sources).await;
                    }
                }
                Err(err)
            }
            Err(err) => Err(err),
        }
    }
}

impl std::fmt::Debug for LlmRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LlmRegistry")
            .field("models", &self.available_models())
            .field("default", &self.default_model)
            .field("fallback", &self.fallback_model)
            .finish()
    }
}

pub struct LlmRegistryBuilder {
    providers: HashMap<String, Arc<dyn LlmProvider>>,
    default_model: Option<String>,
    fallback_model: Option<String>,
}

impl Default for LlmRegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LlmRegistryBuilder {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            default_model: None,
            fallback_model: None,
        }
    }

    pub fn register(mut self, model_id: impl Into<String>, provider: Arc<dyn LlmProvider>) -> Self {
        let model_id = model_id.into();
        if self.default_model.is_none() {
            self.default_model = Some(model_id.clone());
        }
        self.providers.insert(model_id, provider);
        self
    }

    pub fn default_model(mut self, model_id: impl Into<String>) -> Self {
        self.default_model = Some(model_id.into());
        self
    }

    pub fn fallback_model(mut self, model_id: impl Into<String>) -> Self {
        self.fallback_model = Some(model_id.into());
        self
    }

    pub fn build(self) -> LlmRegistry {
        LlmRegistry {
            providers: self.providers,
            default_model: self.default_model,
            fallback_model: self.fallback_model,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use gorkd_core::{Confidence, ResearchAnswer};
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockProvider {
        model_id: String,
        provider_name: String,
        call_count: AtomicUsize,
        error: Option<LlmError>,
    }

    impl MockProvider {
        fn new(model_id: impl Into<String>) -> Self {
            let model_id = model_id.into();
            Self {
                provider_name: "mock".to_string(),
                model_id,
                call_count: AtomicUsize::new(0),
                error: None,
            }
        }

        fn with_error(mut self, error: LlmError) -> Self {
            self.error = Some(error);
            self
        }

        fn call_count(&self) -> usize {
            self.call_count.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl LlmProvider for MockProvider {
        async fn synthesize(
            &self,
            _query: &str,
            _sources: &[Source],
        ) -> Result<ResearchAnswer, LlmError> {
            self.call_count.fetch_add(1, Ordering::SeqCst);

            if let Some(ref err) = self.error {
                return Err(err.clone());
            }

            Ok(ResearchAnswer::new(
                "summary",
                "detail",
                Confidence::High,
                &self.model_id,
            ))
        }

        fn model_id(&self) -> &str {
            &self.model_id
        }

        fn provider_name(&self) -> &str {
            &self.provider_name
        }
    }

    #[test]
    fn creates_empty_registry() {
        let registry = LlmRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
        assert!(registry.default().is_none());
    }

    #[test]
    fn registers_and_retrieves_provider() {
        let mut registry = LlmRegistry::new();
        let provider = Arc::new(MockProvider::new("test-model"));
        registry.register("test-model", provider);

        assert!(!registry.is_empty());
        assert_eq!(registry.len(), 1);
        assert!(registry.get("test-model").is_some());
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn first_registered_becomes_default() {
        let mut registry = LlmRegistry::new();
        registry.register("first", Arc::new(MockProvider::new("first")));
        registry.register("second", Arc::new(MockProvider::new("second")));

        assert_eq!(registry.default_model_id(), Some("first"));
    }

    #[test]
    fn can_override_default() {
        let mut registry = LlmRegistry::new();
        registry.register("first", Arc::new(MockProvider::new("first")));
        registry.register("second", Arc::new(MockProvider::new("second")));
        registry.set_default("second");

        assert_eq!(registry.default_model_id(), Some("second"));
    }

    #[test]
    fn builder_creates_registry() {
        let registry = LlmRegistry::builder()
            .register("model-a", Arc::new(MockProvider::new("model-a")))
            .register("model-b", Arc::new(MockProvider::new("model-b")))
            .default_model("model-b")
            .fallback_model("model-a")
            .build();

        assert_eq!(registry.len(), 2);
        assert_eq!(registry.default_model_id(), Some("model-b"));
        assert_eq!(registry.fallback_model_id(), Some("model-a"));
    }

    #[test]
    fn lists_available_models() {
        let mut registry = LlmRegistry::new();
        registry.register("alpha", Arc::new(MockProvider::new("alpha")));
        registry.register("beta", Arc::new(MockProvider::new("beta")));

        let models = registry.available_models();
        assert_eq!(models.len(), 2);
        assert!(models.contains(&"alpha".to_string()));
        assert!(models.contains(&"beta".to_string()));
    }

    #[test]
    fn debug_shows_structure() {
        let registry = LlmRegistry::builder()
            .register("model-a", Arc::new(MockProvider::new("model-a")))
            .default_model("model-a")
            .fallback_model("model-b")
            .build();

        let debug = format!("{:?}", registry);
        assert!(debug.contains("LlmRegistry"));
        assert!(debug.contains("model-a"));
    }

    #[tokio::test]
    async fn synthesize_uses_specified_model() {
        let primary = Arc::new(MockProvider::new("primary"));
        let secondary = Arc::new(MockProvider::new("secondary"));

        let registry = LlmRegistry::builder()
            .register("primary", primary.clone())
            .register("secondary", secondary.clone())
            .default_model("primary")
            .build();

        let result = registry
            .synthesize_with_fallback("query", &[], Some("secondary"))
            .await;

        assert!(result.is_ok());
        assert_eq!(primary.call_count(), 0);
        assert_eq!(secondary.call_count(), 1);
    }

    #[tokio::test]
    async fn synthesize_uses_default_when_no_model_specified() {
        let primary = Arc::new(MockProvider::new("primary"));

        let registry = LlmRegistry::builder()
            .register("primary", primary.clone())
            .default_model("primary")
            .build();

        let result = registry.synthesize_with_fallback("query", &[], None).await;

        assert!(result.is_ok());
        assert_eq!(primary.call_count(), 1);
    }

    #[tokio::test]
    async fn synthesize_falls_back_on_retryable_error() {
        let primary = Arc::new(MockProvider::new("primary").with_error(LlmError::RateLimited));
        let fallback = Arc::new(MockProvider::new("fallback"));

        let registry = LlmRegistry::builder()
            .register("primary", primary.clone())
            .register("fallback", fallback.clone())
            .default_model("primary")
            .fallback_model("fallback")
            .build();

        let result = registry.synthesize_with_fallback("query", &[], None).await;

        assert!(result.is_ok());
        assert_eq!(primary.call_count(), 1);
        assert_eq!(fallback.call_count(), 1);
    }

    #[tokio::test]
    async fn synthesize_does_not_fallback_on_non_retryable_error() {
        let primary = Arc::new(MockProvider::new("primary").with_error(
            LlmError::ContentFiltered {
                reason: "policy violation".to_string(),
            },
        ));
        let fallback = Arc::new(MockProvider::new("fallback"));

        let registry = LlmRegistry::builder()
            .register("primary", primary.clone())
            .register("fallback", fallback.clone())
            .default_model("primary")
            .fallback_model("fallback")
            .build();

        let result = registry.synthesize_with_fallback("query", &[], None).await;

        assert!(result.is_err());
        assert_eq!(primary.call_count(), 1);
        assert_eq!(fallback.call_count(), 0);
    }

    #[tokio::test]
    async fn synthesize_returns_error_when_model_not_found() {
        let registry = LlmRegistry::builder()
            .register("existing", Arc::new(MockProvider::new("existing")))
            .build();

        let result = registry
            .synthesize_with_fallback("query", &[], Some("nonexistent"))
            .await;

        assert!(matches!(result, Err(LlmError::ModelUnavailable { .. })));
    }

    #[tokio::test]
    async fn synthesize_returns_error_when_no_default() {
        let registry = LlmRegistry::new();

        let result = registry.synthesize_with_fallback("query", &[], None).await;

        assert!(matches!(result, Err(LlmError::ModelUnavailable { .. })));
    }

    #[tokio::test]
    async fn fallback_not_used_when_same_as_primary() {
        let provider = Arc::new(MockProvider::new("same").with_error(LlmError::RateLimited));

        let registry = LlmRegistry::builder()
            .register("same", provider.clone())
            .default_model("same")
            .fallback_model("same")
            .build();

        let result = registry.synthesize_with_fallback("query", &[], None).await;

        assert!(result.is_err());
        assert_eq!(provider.call_count(), 1);
    }

    #[tokio::test]
    async fn fallback_on_timeout_error() {
        let primary = Arc::new(
            MockProvider::new("primary").with_error(LlmError::Timeout { timeout_secs: 30 }),
        );
        let fallback = Arc::new(MockProvider::new("fallback"));

        let registry = LlmRegistry::builder()
            .register("primary", primary.clone())
            .register("fallback", fallback.clone())
            .default_model("primary")
            .fallback_model("fallback")
            .build();

        let result = registry.synthesize_with_fallback("query", &[], None).await;

        assert!(result.is_ok());
        assert_eq!(fallback.call_count(), 1);
    }

    #[tokio::test]
    async fn fallback_on_network_error() {
        let primary = Arc::new(
            MockProvider::new("primary").with_error(LlmError::Network("connection refused".into())),
        );
        let fallback = Arc::new(MockProvider::new("fallback"));

        let registry = LlmRegistry::builder()
            .register("primary", primary.clone())
            .register("fallback", fallback.clone())
            .default_model("primary")
            .fallback_model("fallback")
            .build();

        let result = registry.synthesize_with_fallback("query", &[], None).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn no_fallback_on_context_length_exceeded() {
        let primary = Arc::new(MockProvider::new("primary").with_error(
            LlmError::ContextLengthExceeded {
                max_tokens: 128000,
                got_tokens: 150000,
            },
        ));
        let fallback = Arc::new(MockProvider::new("fallback"));

        let registry = LlmRegistry::builder()
            .register("primary", primary.clone())
            .register("fallback", fallback.clone())
            .default_model("primary")
            .fallback_model("fallback")
            .build();

        let result = registry.synthesize_with_fallback("query", &[], None).await;

        assert!(result.is_err());
        assert_eq!(fallback.call_count(), 0);
    }
}
