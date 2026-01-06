use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Recency {
    Day,
    Week,
    Month,
    Year,
    Any,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ContentType {
    News,
    Academic,
    General,
    Blog,
    Forum,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SearchFilters {
    pub recency: Option<Recency>,
    pub include_domains: Option<Vec<String>>,
    pub exclude_domains: Option<Vec<String>>,
    pub content_type: Option<ContentType>,
}

impl SearchFilters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_recency(mut self, recency: Recency) -> Self {
        self.recency = Some(recency);
        self
    }

    pub fn include_domains(mut self, domains: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.include_domains = Some(domains.into_iter().map(Into::into).collect());
        self
    }

    pub fn exclude_domains(mut self, domains: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclude_domains = Some(domains.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = Some(content_type);
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub text: String,
    pub filters: SearchFilters,
}

impl SearchQuery {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            filters: SearchFilters::default(),
        }
    }

    pub fn with_filters(mut self, filters: SearchFilters) -> Self {
        self.filters = filters;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProviderId(pub String);

impl ProviderId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ProviderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const DEFAULT_MAX_SOURCES: usize = 10;
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchPlan {
    pub queries: Vec<SearchQuery>,
    pub providers: Vec<ProviderId>,
    pub max_sources: usize,
    #[serde(with = "humantime_serde")]
    pub timeout: Duration,
}

impl SearchPlan {
    pub fn new(queries: Vec<SearchQuery>, providers: Vec<ProviderId>) -> Self {
        Self {
            queries,
            providers,
            max_sources: DEFAULT_MAX_SOURCES,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        }
    }

    pub fn with_max_sources(mut self, max: usize) -> Self {
        self.max_sources = max;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

mod humantime_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_secs().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_search_query() {
        let query = SearchQuery::new("Rust programming language");
        assert_eq!(query.text, "Rust programming language");
    }

    #[test]
    fn builds_filters() {
        let filters = SearchFilters::new()
            .with_recency(Recency::Week)
            .include_domains(["rust-lang.org"])
            .with_content_type(ContentType::News);

        assert_eq!(filters.recency, Some(Recency::Week));
        assert_eq!(
            filters.include_domains,
            Some(vec!["rust-lang.org".to_string()])
        );
    }

    #[test]
    fn creates_search_plan() {
        let queries = vec![SearchQuery::new("test")];
        let providers = vec![ProviderId::new("tavily")];
        let plan = SearchPlan::new(queries, providers);

        assert_eq!(plan.max_sources, DEFAULT_MAX_SOURCES);
        assert_eq!(plan.timeout.as_secs(), DEFAULT_TIMEOUT_SECS);
    }

    #[test]
    fn serializes_recency() {
        let json = serde_json::to_string(&Recency::Week).unwrap();
        assert_eq!(json, "\"week\"");
    }

    #[test]
    fn serializes_search_plan() {
        let plan = SearchPlan::new(
            vec![SearchQuery::new("test")],
            vec![ProviderId::new("tavily")],
        );
        let json = serde_json::to_string(&plan).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("tavily"));
    }
}
