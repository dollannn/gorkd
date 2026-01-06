use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::id::SourceId;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Confidence {
    High,
    Medium,
    Low,
    Insufficient,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Citation {
    pub claim: String,
    pub source_id: SourceId,
    pub quote: Option<String>,
}

impl Citation {
    pub fn new(claim: impl Into<String>, source_id: SourceId) -> Self {
        Self {
            claim: claim.into(),
            source_id,
            quote: None,
        }
    }

    pub fn with_quote(mut self, quote: impl Into<String>) -> Self {
        self.quote = Some(quote.into());
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SynthesisMetadata {
    pub model: String,
    pub tokens_used: usize,
    #[serde(with = "duration_millis")]
    pub synthesis_duration: Duration,
}

impl SynthesisMetadata {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            tokens_used: 0,
            synthesis_duration: Duration::ZERO,
        }
    }

    pub fn with_tokens_used(mut self, tokens: usize) -> Self {
        self.tokens_used = tokens;
        self
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.synthesis_duration = duration;
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResearchAnswer {
    pub summary: String,
    pub detail: String,
    pub citations: Vec<Citation>,
    pub confidence: Confidence,
    pub limitations: Vec<String>,
    pub synthesis_metadata: SynthesisMetadata,
}

impl ResearchAnswer {
    pub fn new(
        summary: impl Into<String>,
        detail: impl Into<String>,
        confidence: Confidence,
        model: impl Into<String>,
    ) -> Self {
        Self {
            summary: summary.into(),
            detail: detail.into(),
            citations: Vec::new(),
            confidence,
            limitations: Vec::new(),
            synthesis_metadata: SynthesisMetadata::new(model),
        }
    }

    pub fn with_citations(mut self, citations: Vec<Citation>) -> Self {
        self.citations = citations;
        self
    }

    pub fn add_citation(&mut self, citation: Citation) {
        self.citations.push(citation);
    }

    pub fn with_limitations(
        mut self,
        limitations: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.limitations = limitations.into_iter().map(Into::into).collect();
        self
    }

    pub fn add_limitation(&mut self, limitation: impl Into<String>) {
        self.limitations.push(limitation.into());
    }

    pub fn with_metadata(mut self, metadata: SynthesisMetadata) -> Self {
        self.synthesis_metadata = metadata;
        self
    }

    pub fn is_answerable(&self) -> bool {
        !matches!(self.confidence, Confidence::Insufficient)
    }
}

mod duration_millis {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_millis().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_citation() {
        let source_id = SourceId::new();
        let citation = Citation::new("Rust is memory-safe", source_id.clone())
            .with_quote("Rust guarantees memory safety without garbage collection");

        assert_eq!(citation.source_id, source_id);
        assert!(citation.quote.is_some());
    }

    #[test]
    fn creates_answer() {
        let answer = ResearchAnswer::new(
            "Yes, Rust is memory-safe.",
            "Detailed explanation...",
            Confidence::High,
            "gpt-4",
        );

        assert!(answer.is_answerable());
        assert!(answer.citations.is_empty());
    }

    #[test]
    fn insufficient_confidence_not_answerable() {
        let answer = ResearchAnswer::new(
            "Unable to determine.",
            "Not enough sources.",
            Confidence::Insufficient,
            "gpt-4",
        );

        assert!(!answer.is_answerable());
    }

    #[test]
    fn serializes_confidence() {
        let json = serde_json::to_string(&Confidence::High).unwrap();
        assert_eq!(json, "\"high\"");
    }

    #[test]
    fn serializes_answer() {
        let answer = ResearchAnswer::new("Summary", "Detail", Confidence::Medium, "gpt-4")
            .with_limitations(["Limited data"]);

        let json = serde_json::to_string(&answer).unwrap();
        assert!(json.contains("medium"));
        assert!(json.contains("Limited data"));
    }

    #[test]
    fn adds_citations() {
        let mut answer = ResearchAnswer::new("Summary", "Detail", Confidence::High, "gpt-4");
        answer.add_citation(Citation::new("Claim 1", SourceId::new()));
        answer.add_citation(Citation::new("Claim 2", SourceId::new()));

        assert_eq!(answer.citations.len(), 2);
    }
}
