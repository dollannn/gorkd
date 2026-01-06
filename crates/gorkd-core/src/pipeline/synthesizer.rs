//! Answer synthesis for research pipeline.

use std::sync::Arc;

use crate::answer::ResearchAnswer;
use crate::source::Source;
use crate::traits::{LlmError, LlmProvider};

#[derive(Clone, Debug)]
pub struct SynthesizerConfig {
    pub max_context_sources: usize,
}

impl Default for SynthesizerConfig {
    fn default() -> Self {
        Self {
            max_context_sources: 5,
        }
    }
}

pub struct Synthesizer {
    provider: Arc<dyn LlmProvider>,
    config: SynthesizerConfig,
}

impl Synthesizer {
    pub fn new(provider: Arc<dyn LlmProvider>, config: SynthesizerConfig) -> Self {
        Self { provider, config }
    }

    pub async fn synthesize(
        &self,
        query: &str,
        sources: &[Source],
    ) -> Result<ResearchAnswer, LlmError> {
        let context_sources: Vec<_> = sources
            .iter()
            .take(self.config.max_context_sources)
            .cloned()
            .collect();

        self.provider.synthesize(query, &context_sources).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::answer::Confidence;
    use crate::mock::MockLlmProvider;

    fn create_test_sources() -> Vec<Source> {
        vec![
            Source::new(
                "https://example.com/1",
                "Source 1",
                "Content about the topic",
            ),
            Source::new("https://example.com/2", "Source 2", "More relevant content"),
            Source::new(
                "https://example.com/3",
                "Source 3",
                "Additional information",
            ),
        ]
    }

    #[tokio::test]
    async fn synthesizer_generates_answer() {
        let provider = Arc::new(MockLlmProvider::new("mock-gpt-4"));
        let synthesizer = Synthesizer::new(provider, SynthesizerConfig::default());
        let sources = create_test_sources();

        let answer = synthesizer
            .synthesize("What is Rust?", &sources)
            .await
            .unwrap();

        assert!(!answer.summary.is_empty());
        assert!(!answer.detail.is_empty());
        assert!(answer.is_answerable());
    }

    #[tokio::test]
    async fn synthesizer_limits_context_sources() {
        let provider = Arc::new(MockLlmProvider::new("mock-gpt-4"));
        let config = SynthesizerConfig {
            max_context_sources: 2,
        };
        let synthesizer = Synthesizer::new(provider, config);

        let mut sources = create_test_sources();
        sources.extend(create_test_sources());

        let answer = synthesizer.synthesize("test", &sources).await.unwrap();
        assert!(answer.is_answerable());
    }

    #[tokio::test]
    async fn synthesizer_returns_insufficient_for_empty_sources() {
        let provider = Arc::new(MockLlmProvider::new("mock-gpt-4"));
        let synthesizer = Synthesizer::new(provider, SynthesizerConfig::default());

        let answer = synthesizer.synthesize("test", &[]).await.unwrap();

        assert_eq!(answer.confidence, Confidence::Insufficient);
        assert!(!answer.is_answerable());
    }
}
