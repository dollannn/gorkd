use gorkd_core::Source;

use crate::types::Message;

pub const SYNTHESIS_SYSTEM_PROMPT: &str = r#"You are a research assistant that synthesizes information from multiple sources to answer questions accurately and with citations.

Your task is to analyze the provided sources and create a well-researched answer.

Guidelines:
1. ONLY use information from the provided sources - never make up facts
2. ALWAYS cite your sources using [source_id] format for each claim
3. If sources conflict, acknowledge the disagreement and present both views
4. If sources are insufficient to answer the question, say so clearly
5. Be concise but thorough - prioritize accuracy over brevity

Response format (JSON):
{
  "summary": "A 1-2 sentence direct answer to the question",
  "detail": "A detailed explanation with inline citations [src_xxx]",
  "citations": [
    {"claim": "Specific claim made", "source_id": "src_xxx", "quote": "Optional direct quote from source"}
  ],
  "confidence": "high|medium|low|insufficient",
  "limitations": ["Any caveats or limitations about the answer"]
}"#;

pub fn build_synthesis_messages(query: &str, sources: &[Source]) -> Vec<Message> {
    let sources_text = format_sources(sources);
    let user_prompt = format!(
        "Question: {}\n\nSources:\n{}\n\nProvide your analysis in the specified JSON format.",
        query, sources_text
    );

    vec![
        Message::system(SYNTHESIS_SYSTEM_PROMPT),
        Message::user(user_prompt),
    ]
}

fn format_sources(sources: &[Source]) -> String {
    sources
        .iter()
        .map(|s| {
            format!(
                "[{}] {}\nURL: {}\nContent:\n{}\n",
                s.id.as_str(),
                s.title,
                s.url,
                s.content
            )
        })
        .collect::<Vec<_>>()
        .join("\n---\n")
}

pub fn estimate_token_count(text: &str) -> usize {
    text.len() / 4
}

pub fn estimate_messages_tokens(messages: &[Message]) -> usize {
    messages
        .iter()
        .map(|m| estimate_token_count(&m.content))
        .sum::<usize>()
        + messages.len() * 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_synthesis_messages() {
        let sources = vec![
            Source::new(
                "https://example.com/a",
                "Article A",
                "Content about topic A",
            ),
            Source::new(
                "https://example.com/b",
                "Article B",
                "Content about topic B",
            ),
        ];

        let messages = build_synthesis_messages("What is the topic?", &sources);

        assert_eq!(messages.len(), 2);
        assert!(messages[0].content.contains("research assistant"));
        assert!(messages[1].content.contains("What is the topic?"));
        assert!(messages[1].content.contains("Article A"));
        assert!(messages[1].content.contains("Article B"));
    }

    #[test]
    fn formats_sources() {
        let sources = vec![Source::new(
            "https://rust-lang.org",
            "Rust Language",
            "Rust is a systems programming language.",
        )];

        let formatted = format_sources(&sources);

        assert!(formatted.contains("Rust Language"));
        assert!(formatted.contains("https://rust-lang.org"));
        assert!(formatted.contains("systems programming"));
        assert!(formatted.contains("src_"));
    }

    #[test]
    fn estimates_token_count() {
        let text = "This is a test message";
        let estimate = estimate_token_count(text);
        assert!(estimate > 0);
        assert!(estimate < text.len());
    }

    #[test]
    fn system_prompt_includes_citation_instructions() {
        assert!(SYNTHESIS_SYSTEM_PROMPT.contains("cite"));
        assert!(SYNTHESIS_SYSTEM_PROMPT.contains("[source_id]"));
        assert!(SYNTHESIS_SYSTEM_PROMPT.contains("confidence"));
    }
}
