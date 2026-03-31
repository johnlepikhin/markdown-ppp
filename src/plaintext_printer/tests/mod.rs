#![cfg(test)]
use rstest::rstest;

#[rstest]
// Basic inline formatting
#[case("Hello, world!", "Hello, world!")]
#[case("Hello, **world**!", "Hello, world!")]
#[case("Hello, *world*!", "Hello, world!")]
#[case("Hello, __world__!", "Hello, world!")]
#[case("Hello, _world_!", "Hello, world!")]
#[case("Hello, ~~world~~!", "Hello, world!")]
#[case("`code`", "code")]
// Nested emphasis
#[case("**bold *and italic* here**", "bold and italic here")]
// Code block
#[case("```rust\nfn main() {}\n```", "fn main() {}")]
// Links and images
#[case("[Google](https://google.com)", "Google")]
#[case("![alt text](https://example.com/image.png)", "alt text")]
#[case("<https://example.com>", "https://example.com")]
// Lists
#[case(
    "1. Item 1\n2. Item 2",
    "1. Item 1\n2. Item 2"
)]
#[case(
    "* Item 1\n* Item 2",
    "- Item 1\n- Item 2"
)]
// Task lists
#[case(
    "- [x] Done\n- [ ] Todo",
    "- [x] Done\n- [ ] Todo"
)]
// Heading + paragraph
#[case(
    "# Heading\n\nParagraph",
    "Heading\n\nParagraph"
)]
// Thematic break
#[case(
    "Above\n\n---\n\nBelow",
    "Above\n\n---\n\nBelow"
)]
// Block quote
#[case(
    "> Quoted text",
    "Quoted text"
)]
// Footnotes
#[case(
    "Hello[^1]\n\n[^1]: This is a footnote.",
    "Hello[1]\n\n[1] This is a footnote."
)]
// Link references
#[case(
    "[Google][1]\n\n[1]: https://www.google.com 'Search engine'",
    "Google"
)]
// Tables
#[case(
    "| Header 1 | Header 2 |\n| --- | --- |\n| Cell 1 | Cell 2 |",
    "Header 1 | Header 2\nCell 1 | Cell 2"
)]
fn render_to_plaintext(#[case] input: &str, #[case] expected: &str) {
    let config = crate::plaintext_printer::config::Config::default();
    let ast = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();
    println!("{input:?} => {ast:#?}");
    let result = crate::plaintext_printer::render_plaintext(&ast, config);
    assert_eq!(expected, result);
}

#[test]
fn empty_document() {
    let doc = crate::ast::Document { blocks: vec![] };
    let result =
        crate::plaintext_printer::render_plaintext(&doc, crate::plaintext_printer::config::Config::default());
    assert_eq!("", result);
}

#[test]
fn html_block_is_skipped() {
    let config = crate::plaintext_printer::config::Config::default();
    let ast = crate::parser::parse_markdown(
        crate::parser::MarkdownParserState::default(),
        "Before\n\n<div>html</div>\n\nAfter",
    )
    .unwrap();
    let result = crate::plaintext_printer::render_plaintext(&ast, config);
    assert_eq!("Before\n\nAfter", result);
}

#[test]
fn unicode_text() {
    let config = crate::plaintext_printer::config::Config::default();
    let ast = crate::parser::parse_markdown(
        crate::parser::MarkdownParserState::default(),
        "Привет, **мир**! 🌍",
    )
    .unwrap();
    let result = crate::plaintext_printer::render_plaintext(&ast, config);
    assert_eq!("Привет, мир! 🌍", result);
}

#[test]
fn config_width_affects_output() {
    let config = crate::plaintext_printer::config::Config::default().with_width(20);
    let doc = crate::ast::Document {
        blocks: vec![crate::ast::Block::Paragraph(vec![
            crate::ast::Inline::Text("Short".to_string()),
        ])],
    };
    let result = crate::plaintext_printer::render_plaintext(&doc, config);
    assert_eq!("Short", result);
}

#[test]
fn github_alert() {
    let config = crate::plaintext_printer::config::Config::default();
    let ast = crate::parser::parse_markdown(
        crate::parser::MarkdownParserState::default(),
        "> [!NOTE]\n> Important info here",
    )
    .unwrap();
    let result = crate::plaintext_printer::render_plaintext(&ast, config);
    assert_eq!("[Note]\nImportant info here", result);
}
