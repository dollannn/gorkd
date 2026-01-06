use std::collections::HashMap;

use gorkd_core::{Citation, Confidence, ResearchAnswer, Source, SourceId, SynthesisMetadata};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RawSynthesisResponse {
    summary: String,
    detail: String,
    citations: Vec<RawCitation>,
    confidence: String,
    #[serde(default)]
    limitations: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawCitation {
    claim: String,
    source_id: String,
    #[serde(default)]
    quote: Option<String>,
}

pub fn parse_synthesis_response(
    text: &str,
    sources: &[Source],
    model: &str,
    tokens_used: usize,
) -> Result<ResearchAnswer, ParseError> {
    let json_text = extract_json(text)?;
    let raw: RawSynthesisResponse =
        serde_json::from_str(&json_text).map_err(|e| ParseError::InvalidJson(e.to_string()))?;

    let source_map: HashMap<&str, &SourceId> =
        sources.iter().map(|s| (s.id.as_str(), &s.id)).collect();

    let citations = raw
        .citations
        .into_iter()
        .filter_map(|c| resolve_citation(c, &source_map))
        .collect();

    let confidence = parse_confidence(&raw.confidence);

    let metadata = SynthesisMetadata::new(model).with_tokens_used(tokens_used);

    Ok(
        ResearchAnswer::new(raw.summary, raw.detail, confidence, model)
            .with_citations(citations)
            .with_limitations(raw.limitations)
            .with_metadata(metadata),
    )
}

fn extract_json(text: &str) -> Result<String, ParseError> {
    let trimmed = text.trim();

    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        return Ok(trimmed.to_string());
    }

    if let Some(start) = trimmed.find("```json") {
        let after_marker = &trimmed[start + 7..];
        if let Some(end) = after_marker.find("```") {
            return Ok(after_marker[..end].trim().to_string());
        }
    }

    if let Some(start) = trimmed.find("```") {
        let after_marker = &trimmed[start + 3..];
        if let Some(end) = after_marker.find("```") {
            let inner = after_marker[..end].trim();
            if inner.starts_with('{') {
                return Ok(inner.to_string());
            }
        }
    }

    if let Some(start) = trimmed.find('{') {
        if let Some(end) = trimmed.rfind('}') {
            if start < end {
                return Ok(trimmed[start..=end].to_string());
            }
        }
    }

    Err(ParseError::NoJsonFound)
}

fn resolve_citation(raw: RawCitation, source_map: &HashMap<&str, &SourceId>) -> Option<Citation> {
    let source_id = source_map.get(raw.source_id.as_str()).copied()?;
    let mut citation = Citation::new(raw.claim, source_id.clone());
    if let Some(quote) = raw.quote {
        citation = citation.with_quote(quote);
    }
    Some(citation)
}

fn parse_confidence(s: &str) -> Confidence {
    match s.to_lowercase().as_str() {
        "high" => Confidence::High,
        "medium" => Confidence::Medium,
        "low" => Confidence::Low,
        "insufficient" => Confidence::Insufficient,
        _ => Confidence::Medium,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    NoJsonFound,
    InvalidJson(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoJsonFound => write!(f, "no JSON object found in response"),
            Self::InvalidJson(e) => write!(f, "invalid JSON: {}", e),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sources() -> Vec<Source> {
        vec![
            Source::new("https://example.com/a", "Source A", "Content A"),
            Source::new("https://example.com/b", "Source B", "Content B"),
        ]
    }

    #[test]
    fn parses_clean_json() {
        let sources = test_sources();
        let json = format!(
            r#"{{
                "summary": "Test summary",
                "detail": "Test detail with citation [{}]",
                "citations": [
                    {{"claim": "Test claim", "source_id": "{}", "quote": "exact quote"}}
                ],
                "confidence": "high",
                "limitations": ["Limited data"]
            }}"#,
            sources[0].id.as_str(),
            sources[0].id.as_str()
        );

        let answer = parse_synthesis_response(&json, &sources, "claude-sonnet-4", 100).unwrap();
        assert_eq!(answer.summary, "Test summary");
        assert_eq!(answer.confidence, Confidence::High);
        assert_eq!(answer.citations.len(), 1);
        assert!(answer.citations[0].quote.is_some());
        assert_eq!(answer.limitations.len(), 1);
    }

    #[test]
    fn parses_json_in_code_block() {
        let sources = test_sources();
        let text = format!(
            r#"Here is my analysis:

```json
{{
    "summary": "Summary",
    "detail": "Detail",
    "citations": [{{"claim": "Claim", "source_id": "{}"}}],
    "confidence": "medium",
    "limitations": []
}}
```"#,
            sources[0].id.as_str()
        );

        let answer = parse_synthesis_response(&text, &sources, "claude-sonnet-4", 50).unwrap();
        assert_eq!(answer.summary, "Summary");
        assert_eq!(answer.confidence, Confidence::Medium);
    }

    #[test]
    fn parses_json_with_surrounding_text() {
        let sources = test_sources();
        let text = format!(
            r#"Let me analyze that for you.

{{
    "summary": "Extracted summary",
    "detail": "Extracted detail",
    "citations": [],
    "confidence": "low",
    "limitations": []
}}

Hope this helps!"#
        );

        let answer = parse_synthesis_response(&text, &sources, "claude-sonnet-4", 75).unwrap();
        assert_eq!(answer.summary, "Extracted summary");
        assert_eq!(answer.confidence, Confidence::Low);
    }

    #[test]
    fn skips_citations_with_unknown_source_ids() {
        let sources = test_sources();
        let json = format!(
            r#"{{
                "summary": "Summary",
                "detail": "Detail",
                "citations": [
                    {{"claim": "Valid", "source_id": "{}"}},
                    {{"claim": "Invalid", "source_id": "src_nonexistent"}}
                ],
                "confidence": "high",
                "limitations": []
            }}"#,
            sources[0].id.as_str()
        );

        let answer = parse_synthesis_response(&json, &sources, "claude-sonnet-4", 100).unwrap();
        assert_eq!(answer.citations.len(), 1);
        assert_eq!(answer.citations[0].claim, "Valid");
    }

    #[test]
    fn handles_missing_limitations() {
        let sources = test_sources();
        let json = r#"{
            "summary": "Summary",
            "detail": "Detail",
            "citations": [],
            "confidence": "high"
        }"#;

        let answer = parse_synthesis_response(json, &sources, "claude-sonnet-4", 100).unwrap();
        assert!(answer.limitations.is_empty());
    }

    #[test]
    fn defaults_to_medium_confidence_for_unknown() {
        let sources = test_sources();
        let json = r#"{
            "summary": "Summary",
            "detail": "Detail",
            "citations": [],
            "confidence": "uncertain",
            "limitations": []
        }"#;

        let answer = parse_synthesis_response(json, &sources, "claude-sonnet-4", 100).unwrap();
        assert_eq!(answer.confidence, Confidence::Medium);
    }

    #[test]
    fn returns_error_for_no_json() {
        let sources = test_sources();
        let text = "This response contains no JSON at all.";

        let result = parse_synthesis_response(text, &sources, "claude-sonnet-4", 100);
        assert!(matches!(result, Err(ParseError::NoJsonFound)));
    }

    #[test]
    fn returns_error_for_invalid_json() {
        let sources = test_sources();
        let text = r#"{"summary": "Missing fields"}"#;

        let result = parse_synthesis_response(text, &sources, "claude-sonnet-4", 100);
        assert!(matches!(result, Err(ParseError::InvalidJson(_))));
    }

    #[test]
    fn extracts_json_correctly() {
        assert!(extract_json(r#"{"key": "value"}"#).is_ok());
        assert!(extract_json("```json\n{\"key\": \"value\"}\n```").is_ok());
        assert!(extract_json("```\n{\"key\": \"value\"}\n```").is_ok());
        assert!(extract_json("text before {\"key\": \"value\"} text after").is_ok());
        assert!(extract_json("no json here").is_err());
    }
}
