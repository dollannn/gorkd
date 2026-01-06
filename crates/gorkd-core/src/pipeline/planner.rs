//! Query planning for research pipeline.

use crate::search::{ProviderId, SearchPlan, SearchQuery};

#[derive(Clone, Debug)]
pub struct PlannerConfig {
    pub max_queries: usize,
    pub default_providers: Vec<String>,
}

impl Default for PlannerConfig {
    fn default() -> Self {
        Self {
            max_queries: 3,
            default_providers: vec!["tavily".to_string()],
        }
    }
}

pub struct Planner {
    config: PlannerConfig,
}

impl Planner {
    pub fn new(config: PlannerConfig) -> Self {
        Self { config }
    }

    pub fn plan(&self, query: &str) -> SearchPlan {
        let queries = vec![SearchQuery::new(query)];

        let providers = self
            .config
            .default_providers
            .iter()
            .map(ProviderId::new)
            .collect();

        SearchPlan::new(queries, providers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planner_creates_search_plan() {
        let planner = Planner::new(PlannerConfig::default());
        let plan = planner.plan("What is Rust?");

        assert_eq!(plan.queries.len(), 1);
        assert_eq!(plan.queries[0].text, "What is Rust?");
        assert!(!plan.providers.is_empty());
    }

    #[test]
    fn planner_uses_configured_providers() {
        let config = PlannerConfig {
            max_queries: 3,
            default_providers: vec!["exa".to_string(), "searxng".to_string()],
        };
        let planner = Planner::new(config);
        let plan = planner.plan("test");

        assert_eq!(plan.providers.len(), 2);
        assert_eq!(plan.providers[0].as_str(), "exa");
        assert_eq!(plan.providers[1].as_str(), "searxng");
    }
}
