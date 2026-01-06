use async_trait::async_trait;

use crate::id::JobId;
use crate::job::ResearchJob;
use crate::source::Source;
use crate::traits::errors::StoreError;

#[async_trait]
pub trait Store: Send + Sync {
    async fn create_job(&self, job: &ResearchJob) -> Result<(), StoreError>;

    async fn get_job(&self, id: &JobId) -> Result<Option<ResearchJob>, StoreError>;

    async fn update_job(&self, job: &ResearchJob) -> Result<(), StoreError>;

    async fn list_jobs(&self, limit: usize, offset: usize) -> Result<Vec<ResearchJob>, StoreError>;

    async fn store_sources(&self, job_id: &JobId, sources: &[Source]) -> Result<(), StoreError>;

    async fn get_sources(&self, job_id: &JobId) -> Result<Vec<Source>, StoreError>;

    async fn find_similar(
        &self,
        embedding: &[f32],
        threshold: f32,
    ) -> Result<Option<JobId>, StoreError>;
}
