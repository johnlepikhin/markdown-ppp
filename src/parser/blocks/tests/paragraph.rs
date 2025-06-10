use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn minimal_paragraph() {
    let doc = parse_markdown(MarkdownParserState::default(), "a").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text("a".to_string())])]
        }
    );

    let doc = parse_markdown(MarkdownParserState::default(), "a b c").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text("a b c".to_string())])]
        }
    );

    let doc = parse_markdown(MarkdownParserState::default(), "a\nb\nc").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text("a\nb\nc".to_string())])]
        }
    );
}

#[test]
fn multi_paragraph1() {
    let doc = parse_markdown(MarkdownParserState::default(), "a\n\nb").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![
                Block::Paragraph(vec![Inline::Text("a".to_string())]),
                Block::Paragraph(vec![Inline::Text("b".to_string())]),
            ]
        }
    );
}

#[test]
fn multi_paragraph2() {
    let doc = parse_markdown(MarkdownParserState::default(), "a\n\n\n\n\nb").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![
                Block::Paragraph(vec![Inline::Text("a".to_string())]),
                Block::Paragraph(vec![Inline::Text("b".to_string())]),
            ]
        }
    );
}

#[test]
fn multi_paragraph3() {
    let doc = parse_markdown(MarkdownParserState::default(), "a\n\n  b").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![
                Block::Paragraph(vec![Inline::Text("a".to_string())]),
                Block::Paragraph(vec![Inline::Text("b".to_string())]),
            ]
        }
    );
}

#[test]
fn multi_paragraph4() {
    let doc = parse_markdown(MarkdownParserState::default(), "aaa\n\n===").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![
                Block::Paragraph(vec![Inline::Text("aaa".to_string())]),
                Block::Paragraph(vec![Inline::Text("===".to_string())]),
            ]
        }
    );
}

#[test]
fn paragraph_with_indented_line1() {
    let doc = parse_markdown(MarkdownParserState::default(), "a\n    b").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text("a\n b".to_string())])],
        }
    );
}
