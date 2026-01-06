use async_trait::async_trait;

use crate::answer::ResearchAnswer;
use crate::source::Source;
use crate::traits::errors::LlmError;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn synthesize(&self, query: &str, sources: &[Source])
        -> Result<ResearchAnswer, LlmError>;

    fn model_id(&self) -> &str;

    fn provider_name(&self) -> &str;

    fn max_context_tokens(&self) -> usize {
        128_000
    }

    fn supports_streaming(&self) -> bool {
        false
    }
}
