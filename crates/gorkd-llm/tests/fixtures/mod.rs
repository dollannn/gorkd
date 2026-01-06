//! Test fixtures for LLM integration tests.
//!
//! Provides reusable test data including sample sources, queries,
//! and expected response structures.

use gorkd_core::Source;

/// Simple query for quick tests.
pub const SIMPLE_QUERY: &str = "What is the Rust programming language?";

/// Query that should return insufficient confidence with empty sources.
pub const UNANSWERABLE_QUERY: &str = "What is the internal meeting schedule for OpenAI?";

/// Query requiring synthesis from multiple sources.
pub const MULTI_SOURCE_QUERY: &str = "What are the main advantages of Rust over C++?";

/// Creates a minimal set of sources for quick integration tests.
///
/// Returns 2 sources with short, focused content to minimize API costs.
pub fn minimal_sources() -> Vec<Source> {
    vec![
        Source::new(
            "https://www.rust-lang.org/",
            "The Rust Programming Language",
            "Rust is a systems programming language focused on safety, speed, and concurrency. \
             It achieves memory safety without garbage collection through its ownership system. \
             Rust prevents data races at compile time and provides zero-cost abstractions.",
        )
        .with_relevance_score(0.95),
        Source::new(
            "https://doc.rust-lang.org/book/",
            "The Rust Book - Introduction",
            "Rust is a language empowering everyone to build reliable and efficient software. \
             It combines low-level control with high-level ergonomics. Rust's compiler catches \
             many classes of bugs at compile time that would be runtime errors in other languages.",
        )
        .with_relevance_score(0.90),
    ]
}

/// Creates a larger set of sources for testing context handling.
///
/// Returns 10 sources to test that providers handle multiple sources correctly
/// without context overflow.
pub fn many_sources() -> Vec<Source> {
    vec![
        Source::new(
            "https://www.rust-lang.org/",
            "Rust Official Site",
            "Rust is a systems programming language that runs blazingly fast, prevents segfaults, \
             and guarantees thread safety.",
        )
        .with_relevance_score(0.95),
        Source::new(
            "https://doc.rust-lang.org/book/ch01-00-getting-started.html",
            "Getting Started with Rust",
            "This chapter covers how to install Rust, write a Hello World program, and use Cargo, \
             Rust's package manager and build system.",
        )
        .with_relevance_score(0.85),
        Source::new(
            "https://blog.rust-lang.org/2024/01/01/rust-2024.html",
            "Rust in 2024",
            "Rust continues to grow in adoption, particularly in systems programming, WebAssembly, \
             and embedded systems. The language maintains backward compatibility while adding features.",
        )
        .with_relevance_score(0.80),
        Source::new(
            "https://stackoverflow.blog/2024/rust-survey/",
            "Stack Overflow Developer Survey - Rust",
            "Rust remains the most loved programming language for the eighth year in a row, \
             with developers praising its memory safety and performance characteristics.",
        )
        .with_relevance_score(0.75),
        Source::new(
            "https://github.com/rust-lang/rust",
            "Rust GitHub Repository",
            "The official Rust repository contains the compiler, standard library, and tooling. \
             Rust is developed openly with contributions from thousands of developers worldwide.",
        )
        .with_relevance_score(0.70),
        Source::new(
            "https://crates.io/",
            "crates.io - Rust Package Registry",
            "crates.io is the official Rust package registry with over 100,000 crates available. \
             It provides dependency management and versioning for Rust projects.",
        )
        .with_relevance_score(0.65),
        Source::new(
            "https://www.rust-lang.org/learn",
            "Learn Rust",
            "Resources for learning Rust include the Rust Book, Rust by Example, and Rustlings. \
             The community provides extensive documentation and tutorials for all skill levels.",
        )
        .with_relevance_score(0.60),
        Source::new(
            "https://foundation.rust-lang.org/",
            "Rust Foundation",
            "The Rust Foundation is an independent non-profit organization supporting the Rust \
             programming language and ecosystem through funding, infrastructure, and governance.",
        )
        .with_relevance_score(0.55),
        Source::new(
            "https://www.rust-lang.org/community",
            "Rust Community",
            "The Rust community is welcoming and inclusive. Resources include the official forum, \
             Discord server, and numerous local meetups and conferences worldwide.",
        )
        .with_relevance_score(0.50),
        Source::new(
            "https://arewewebyet.org/",
            "Are We Web Yet?",
            "Rust has a growing web development ecosystem including frameworks like Actix, Axum, \
             and Rocket. WebAssembly support enables Rust in frontend development as well.",
        )
        .with_relevance_score(0.45),
    ]
}

/// Creates sources for testing citation validation.
///
/// Each source has distinct, verifiable content that should be cited.
pub fn citation_test_sources() -> Vec<Source> {
    vec![
        Source::new(
            "https://example.com/rust-memory",
            "Rust Memory Safety",
            "Rust achieves memory safety through its ownership system. The borrow checker \
             enforces strict rules: each value has exactly one owner, values are moved or \
             borrowed but never aliased mutably, and references must always be valid.",
        )
        .with_relevance_score(0.95),
        Source::new(
            "https://example.com/rust-concurrency",
            "Rust Concurrency Model",
            "Rust prevents data races at compile time. The Send and Sync traits mark types \
             as safe to transfer or share between threads. The compiler rejects code that \
             could cause concurrent memory access issues.",
        )
        .with_relevance_score(0.90),
        Source::new(
            "https://example.com/rust-performance",
            "Rust Performance Characteristics",
            "Rust provides zero-cost abstractions, meaning high-level features compile to \
             code as efficient as hand-written low-level code. There is no garbage collector, \
             and memory is managed deterministically through RAII.",
        )
        .with_relevance_score(0.85),
    ]
}

/// Empty sources for testing insufficient confidence scenarios.
pub fn empty_sources() -> Vec<Source> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_sources_have_valid_structure() {
        let sources = minimal_sources();
        assert_eq!(sources.len(), 2);

        for source in &sources {
            assert!(source.id.as_str().starts_with("src_"));
            assert!(!source.url.is_empty());
            assert!(!source.title.is_empty());
            assert!(!source.content.is_empty());
            assert!(source.relevance_score > 0.0);
        }
    }

    #[test]
    fn many_sources_returns_expected_count() {
        let sources = many_sources();
        assert_eq!(sources.len(), 10);

        let mut prev_score = f32::MAX;
        for source in &sources {
            assert!(source.relevance_score <= prev_score);
            prev_score = source.relevance_score;
        }
    }

    #[test]
    fn citation_sources_have_distinct_content() {
        let sources = citation_test_sources();
        assert_eq!(sources.len(), 3);

        assert!(sources[0].content.contains("ownership"));
        assert!(
            sources[1].content.contains("concurrency") || sources[1].content.contains("thread")
        );
        assert!(
            sources[2].content.contains("performance") || sources[2].content.contains("zero-cost")
        );
    }
}
