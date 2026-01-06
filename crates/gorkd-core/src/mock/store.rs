use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::id::JobId;
use crate::job::ResearchJob;
use crate::source::Source;
use crate::traits::{Store, StoreError};

pub struct MockStore {
    jobs: RwLock<HashMap<String, ResearchJob>>,
    sources: RwLock<HashMap<String, Vec<Source>>>,
}

impl MockStore {
    pub fn new() -> Self {
        Self {
            jobs: RwLock::new(HashMap::new()),
            sources: RwLock::new(HashMap::new()),
        }
    }

    pub fn job_count(&self) -> usize {
        self.jobs.read().unwrap().len()
    }

    pub fn source_count(&self) -> usize {
        self.sources.read().unwrap().values().map(|v| v.len()).sum()
    }
}

impl Default for MockStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Store for MockStore {
    async fn create_job(&self, job: &ResearchJob) -> Result<(), StoreError> {
        let mut jobs = self.jobs.write().unwrap();
        let id = job.id.as_str().to_string();

        if jobs.contains_key(&id) {
            return Err(StoreError::Conflict(format!(
                "job {} already exists",
                job.id
            )));
        }

        jobs.insert(id, job.clone());
        Ok(())
    }

    async fn get_job(&self, id: &JobId) -> Result<Option<ResearchJob>, StoreError> {
        let jobs = self.jobs.read().unwrap();
        Ok(jobs.get(id.as_str()).cloned())
    }

    async fn update_job(&self, job: &ResearchJob) -> Result<(), StoreError> {
        let mut jobs = self.jobs.write().unwrap();
        let id = job.id.as_str().to_string();

        if !jobs.contains_key(&id) {
            return Err(StoreError::JobNotFound { id });
        }

        jobs.insert(id, job.clone());
        Ok(())
    }

    async fn list_jobs(&self, limit: usize, offset: usize) -> Result<Vec<ResearchJob>, StoreError> {
        let jobs = self.jobs.read().unwrap();
        let mut all_jobs: Vec<_> = jobs.values().cloned().collect();

        all_jobs.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(all_jobs.into_iter().skip(offset).take(limit).collect())
    }

    async fn store_sources(&self, job_id: &JobId, sources: &[Source]) -> Result<(), StoreError> {
        let mut store = self.sources.write().unwrap();
        store.insert(job_id.as_str().to_string(), sources.to_vec());
        Ok(())
    }

    async fn get_sources(&self, job_id: &JobId) -> Result<Vec<Source>, StoreError> {
        let store = self.sources.read().unwrap();
        Ok(store.get(job_id.as_str()).cloned().unwrap_or_default())
    }

    async fn find_similar(
        &self,
        _embedding: &[f32],
        _threshold: f32,
    ) -> Result<Option<JobId>, StoreError> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_store_creates_and_retrieves_job() {
        let store = MockStore::new();
        let job = ResearchJob::new("What is Rust?").unwrap();
        let job_id = job.id.clone();

        store.create_job(&job).await.unwrap();
        let retrieved = store.get_job(&job_id).await.unwrap();

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().query, "What is Rust?");
    }

    #[tokio::test]
    async fn mock_store_rejects_duplicate_job() {
        let store = MockStore::new();
        let job = ResearchJob::new("test").unwrap();

        store.create_job(&job).await.unwrap();
        let result = store.create_job(&job).await;

        assert!(matches!(result, Err(StoreError::Conflict(_))));
    }

    #[tokio::test]
    async fn mock_store_updates_job() {
        let store = MockStore::new();
        let mut job = ResearchJob::new("test").unwrap();
        let job_id = job.id.clone();

        store.create_job(&job).await.unwrap();

        job.transition_to(crate::job::JobStatus::Searching);
        store.update_job(&job).await.unwrap();

        let retrieved = store.get_job(&job_id).await.unwrap().unwrap();
        assert_eq!(retrieved.status, crate::job::JobStatus::Searching);
    }

    #[tokio::test]
    async fn mock_store_returns_none_for_missing_job() {
        let store = MockStore::new();
        let job_id = JobId::new();

        let result = store.get_job(&job_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn mock_store_stores_and_retrieves_sources() {
        let store = MockStore::new();
        let job = ResearchJob::new("test").unwrap();
        let job_id = job.id.clone();

        store.create_job(&job).await.unwrap();

        let sources = vec![
            Source::new("https://example.com/1", "Source 1", "Content 1"),
            Source::new("https://example.com/2", "Source 2", "Content 2"),
        ];

        store.store_sources(&job_id, &sources).await.unwrap();
        let retrieved = store.get_sources(&job_id).await.unwrap();

        assert_eq!(retrieved.len(), 2);
    }

    #[tokio::test]
    async fn mock_store_lists_jobs_with_pagination() {
        let store = MockStore::new();

        for i in 0..5 {
            let job = ResearchJob::new(format!("query {}", i)).unwrap();
            store.create_job(&job).await.unwrap();
        }

        let page1 = store.list_jobs(2, 0).await.unwrap();
        let page2 = store.list_jobs(2, 2).await.unwrap();
        let page3 = store.list_jobs(2, 4).await.unwrap();

        assert_eq!(page1.len(), 2);
        assert_eq!(page2.len(), 2);
        assert_eq!(page3.len(), 1);
    }

    #[tokio::test]
    async fn mock_store_tracks_counts() {
        let store = MockStore::new();
        let job = ResearchJob::new("test").unwrap();
        let job_id = job.id.clone();

        assert_eq!(store.job_count(), 0);

        store.create_job(&job).await.unwrap();
        assert_eq!(store.job_count(), 1);

        let sources = vec![Source::new("https://example.com", "Title", "Content")];
        store.store_sources(&job_id, &sources).await.unwrap();
        assert_eq!(store.source_count(), 1);
    }
}
