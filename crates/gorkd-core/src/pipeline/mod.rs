//! Research pipeline orchestration.

mod executor;
mod planner;
mod synthesizer;

pub use executor::{Executor, ExecutorConfig};
pub use planner::{Planner, PlannerConfig};
pub use synthesizer::{Synthesizer, SynthesizerConfig};

use std::sync::Arc;

use crate::answer::ResearchAnswer;
use crate::job::{JobStatus, ResearchJob};
use crate::source::Source;
use crate::traits::{LlmProvider, SearchProvider, Store};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum PipelineError {
    #[error("planning failed: {0}")]
    Planning(String),

    #[error("search failed: {0}")]
    Search(String),

    #[error("synthesis failed: {0}")]
    Synthesis(String),

    #[error("store error: {0}")]
    Store(#[from] crate::traits::StoreError),

    #[error("no sources found for query")]
    NoSources,
}

#[derive(Clone, Debug)]
pub struct PipelineResult {
    pub job: ResearchJob,
    pub sources: Vec<Source>,
    pub answer: ResearchAnswer,
}

#[derive(Clone, Debug, Default)]
pub struct PipelineConfig {
    pub planner: PlannerConfig,
    pub executor: ExecutorConfig,
    pub synthesizer: SynthesizerConfig,
}

pub struct Pipeline {
    store: Arc<dyn Store>,
    search_provider: Arc<dyn SearchProvider>,
    llm_provider: Arc<dyn LlmProvider>,
    config: PipelineConfig,
}

impl Pipeline {
    pub fn new(
        store: Arc<dyn Store>,
        search_provider: Arc<dyn SearchProvider>,
        llm_provider: Arc<dyn LlmProvider>,
    ) -> Self {
        Self {
            store,
            search_provider,
            llm_provider,
            config: PipelineConfig::default(),
        }
    }

    pub fn with_config(mut self, config: PipelineConfig) -> Self {
        self.config = config;
        self
    }

    pub async fn run(&self, mut job: ResearchJob) -> Result<PipelineResult, PipelineError> {
        job.transition_to(JobStatus::Planning);
        self.store.update_job(&job).await?;

        let planner = Planner::new(self.config.planner.clone());
        let search_plan = planner.plan(&job.query);

        job.transition_to(JobStatus::Searching);
        self.store.update_job(&job).await?;

        let executor = Executor::new(
            Arc::clone(&self.search_provider),
            self.config.executor.clone(),
        );
        let sources = executor
            .execute(&search_plan)
            .await
            .map_err(|e| PipelineError::Search(e.to_string()))?;

        if sources.is_empty() {
            job.fail("No sources found for query");
            self.store.update_job(&job).await?;
            return Err(PipelineError::NoSources);
        }

        self.store.store_sources(&job.id, &sources).await?;

        job.transition_to(JobStatus::Synthesizing);
        self.store.update_job(&job).await?;

        let synthesizer = Synthesizer::new(
            Arc::clone(&self.llm_provider),
            self.config.synthesizer.clone(),
        );
        let answer = synthesizer
            .synthesize(&job.query, &sources)
            .await
            .map_err(|e| PipelineError::Synthesis(e.to_string()))?;

        job.transition_to(JobStatus::Completed);
        self.store.update_job(&job).await?;

        Ok(PipelineResult {
            job,
            sources,
            answer,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{MockLlmProvider, MockSearchProvider, MockStore};

    fn create_test_pipeline() -> Pipeline {
        let store: Arc<dyn Store> = Arc::new(MockStore::new());
        let search: Arc<dyn SearchProvider> = Arc::new(MockSearchProvider::new("mock"));
        let llm: Arc<dyn LlmProvider> = Arc::new(MockLlmProvider::new("mock-gpt-4"));

        Pipeline::new(store, search, llm)
    }

    #[tokio::test]
    async fn pipeline_runs_full_flow() {
        let pipeline = create_test_pipeline();
        let job = ResearchJob::new("What is Rust?").unwrap();

        pipeline.store.create_job(&job).await.unwrap();

        let result = pipeline.run(job).await.unwrap();

        assert_eq!(result.job.status, JobStatus::Completed);
        assert!(!result.sources.is_empty());
        assert!(!result.answer.summary.is_empty());
    }

    #[tokio::test]
    async fn pipeline_updates_job_status() {
        let store: Arc<dyn Store> = Arc::new(MockStore::new());
        let search: Arc<dyn SearchProvider> = Arc::new(MockSearchProvider::new("mock"));
        let llm: Arc<dyn LlmProvider> = Arc::new(MockLlmProvider::new("mock-gpt-4"));

        let pipeline = Pipeline::new(Arc::clone(&store), search, llm);
        let job = ResearchJob::new("Test query").unwrap();
        let job_id = job.id.clone();

        store.create_job(&job).await.unwrap();
        pipeline.run(job).await.unwrap();

        let final_job = store.get_job(&job_id).await.unwrap().unwrap();
        assert_eq!(final_job.status, JobStatus::Completed);
    }

    #[tokio::test]
    async fn pipeline_stores_sources() {
        let store: Arc<dyn Store> = Arc::new(MockStore::new());
        let search: Arc<dyn SearchProvider> = Arc::new(MockSearchProvider::new("mock"));
        let llm: Arc<dyn LlmProvider> = Arc::new(MockLlmProvider::new("mock-gpt-4"));

        let pipeline = Pipeline::new(Arc::clone(&store), search, llm);
        let job = ResearchJob::new("Test query").unwrap();
        let job_id = job.id.clone();

        store.create_job(&job).await.unwrap();
        pipeline.run(job).await.unwrap();

        let sources = store.get_sources(&job_id).await.unwrap();
        assert!(!sources.is_empty());
    }
}
