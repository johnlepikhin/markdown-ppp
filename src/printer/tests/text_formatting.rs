#![cfg(test)]
use rstest::rstest;

#[test]
fn text_with_newlines_formats_to_single_line() {
    let input = r#"line1 line1 line1
line2 line2 line2 line2 line2"#;

    let expected = "line1 line1 line1 line2 line2 line2 line2 line2";

    let config = crate::printer::config::Config::default();
    let doc = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();

    let result = crate::printer::render_markdown(&doc, config);
    assert_eq!(expected, result);
}

#[rstest(
    input,
    expected,
    case("line1\nline2", "line1 line2"),
    case("word1 word2\nword3 word4", "word1 word2 word3 word4"),
    case("first\nsecond\nthird", "first second third")
)]
fn text_newlines_normalize_to_spaces(input: &str, expected: &str) {
    let config = crate::printer::config::Config::default();
    let doc = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();

    let result = crate::printer::render_markdown(&doc, config);
    assert_eq!(expected, result);
}
