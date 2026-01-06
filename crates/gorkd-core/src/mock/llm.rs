use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use async_trait::async_trait;

use crate::answer::{Citation, Confidence, ResearchAnswer, SynthesisMetadata};
use crate::source::Source;
use crate::traits::{LlmError, LlmProvider};

pub struct MockLlmProvider {
    model_id: String,
    call_count: AtomicUsize,
    fail_after: Option<usize>,
    confidence: Confidence,
}

impl MockLlmProvider {
    pub fn new(model_id: impl Into<String>) -> Self {
        Self {
            model_id: model_id.into(),
            call_count: AtomicUsize::new(0),
            fail_after: None,
            confidence: Confidence::High,
        }
    }

    pub fn with_confidence(mut self, confidence: Confidence) -> Self {
        self.confidence = confidence;
        self
    }

    pub fn fail_after(mut self, n: usize) -> Self {
        self.fail_after = Some(n);
        self
    }

    pub fn call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }

    fn generate_answer(&self, query: &str, sources: &[Source]) -> ResearchAnswer {
        let summary = format!(
            "Based on {} sources, here is the answer to: {}",
            sources.len(),
            query
        );
        let detail = format!(
            "After analyzing the provided sources, the answer to \"{}\" is as follows:\n\n\
            The sources indicate that this topic has been well-documented. \
            Multiple reliable sources confirm the key findings.\n\n\
            Sources analyzed: {}",
            query,
            sources.len()
        );

        let citations: Vec<Citation> = sources
            .iter()
            .take(3)
            .map(|s| Citation::new(format!("Information from {}", s.title), s.id.clone()))
            .collect();

        let metadata = SynthesisMetadata::new(&self.model_id)
            .with_tokens_used(500)
            .with_duration(Duration::from_millis(250));

        ResearchAnswer::new(summary, detail, self.confidence.clone(), &self.model_id)
            .with_citations(citations)
            .with_metadata(metadata)
    }
}

#[async_trait]
impl LlmProvider for MockLlmProvider {
    async fn synthesize(
        &self,
        query: &str,
        sources: &[Source],
    ) -> Result<ResearchAnswer, LlmError> {
        let count = self.call_count.fetch_add(1, Ordering::SeqCst);

        if let Some(fail_after) = self.fail_after {
            if count >= fail_after {
                return Err(LlmError::RateLimited);
            }
        }

        if sources.is_empty() {
            return Ok(ResearchAnswer::new(
                "Unable to provide an answer without sources.",
                "No sources were provided for analysis.",
                Confidence::Insufficient,
                &self.model_id,
            ));
        }

        Ok(self.generate_answer(query, sources))
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn provider_name(&self) -> &str {
        "mock"
    }

    fn max_context_tokens(&self) -> usize {
        128_000
    }

    fn supports_streaming(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_sources() -> Vec<Source> {
        vec![
            Source::new("https://example.com/1", "Source 1", "Content 1"),
            Source::new("https://example.com/2", "Source 2", "Content 2"),
        ]
    }

    #[tokio::test]
    async fn mock_llm_synthesizes_answer() {
        let provider = MockLlmProvider::new("mock-gpt-4");
        let sources = create_test_sources();

        let answer = provider
            .synthesize("What is Rust?", &sources)
            .await
            .unwrap();

        assert!(!answer.summary.is_empty());
        assert_eq!(answer.confidence, Confidence::High);
        assert!(!answer.citations.is_empty());
    }

    #[tokio::test]
    async fn mock_llm_returns_insufficient_for_empty_sources() {
        let provider = MockLlmProvider::new("mock-gpt-4");

        let answer = provider.synthesize("What is Rust?", &[]).await.unwrap();

        assert_eq!(answer.confidence, Confidence::Insufficient);
        assert!(!answer.is_answerable());
    }

    #[tokio::test]
    async fn mock_llm_tracks_call_count() {
        let provider = MockLlmProvider::new("mock-gpt-4");
        let sources = create_test_sources();

        assert_eq!(provider.call_count(), 0);

        provider.synthesize("query", &sources).await.unwrap();
        assert_eq!(provider.call_count(), 1);
    }

    #[tokio::test]
    async fn mock_llm_fails_after_n_calls() {
        let provider = MockLlmProvider::new("mock-gpt-4").fail_after(1);
        let sources = create_test_sources();

        assert!(provider.synthesize("query", &sources).await.is_ok());
        assert!(provider.synthesize("query", &sources).await.is_err());
    }

    #[tokio::test]
    async fn mock_llm_respects_configured_confidence() {
        let provider = MockLlmProvider::new("mock-gpt-4").with_confidence(Confidence::Low);
        let sources = create_test_sources();

        let answer = provider.synthesize("query", &sources).await.unwrap();
        assert_eq!(answer.confidence, Confidence::Low);
    }
}
