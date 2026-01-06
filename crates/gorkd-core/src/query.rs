use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum QuestionType {
    Factual,
    Comparison,
    Explanation,
    CurrentEvent,
    HowTo,
    Opinion,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TimeConstraint {
    Recent,
    Historical,
    SpecificDate(DateTime<Utc>),
    DateRange {
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryIntent {
    pub question_type: QuestionType,
    pub entities: Vec<String>,
    pub time_constraint: Option<TimeConstraint>,
    pub language: String,
}

impl QueryIntent {
    pub fn new(question_type: QuestionType) -> Self {
        Self {
            question_type,
            entities: Vec::new(),
            time_constraint: None,
            language: "en".to_string(),
        }
    }

    pub fn with_entities(mut self, entities: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.entities = entities.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_time_constraint(mut self, constraint: TimeConstraint) -> Self {
        self.time_constraint = Some(constraint);
        self
    }

    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_basic_intent() {
        let intent = QueryIntent::new(QuestionType::Factual);
        assert_eq!(intent.question_type, QuestionType::Factual);
        assert_eq!(intent.language, "en");
        assert!(intent.entities.is_empty());
    }

    #[test]
    fn builds_intent_with_entities() {
        let intent = QueryIntent::new(QuestionType::Comparison)
            .with_entities(["Rust", "Go"])
            .with_language("en");
        assert_eq!(intent.entities, vec!["Rust", "Go"]);
    }

    #[test]
    fn serializes_question_type() {
        let json = serde_json::to_string(&QuestionType::CurrentEvent).unwrap();
        assert_eq!(json, "\"current_event\"");
    }

    #[test]
    fn serializes_time_constraint_recent() {
        let json = serde_json::to_string(&TimeConstraint::Recent).unwrap();
        assert_eq!(json, "\"recent\"");
    }

    #[test]
    fn serializes_intent() {
        let intent = QueryIntent::new(QuestionType::Factual).with_entities(["Rust"]);
        let json = serde_json::to_string(&intent).unwrap();
        assert!(json.contains("factual"));
        assert!(json.contains("Rust"));
    }
}
