use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn image1() {
    let doc = parse_markdown(MarkdownParserState::default(), "![foo](/url \"title\")").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
                destination: "/url".to_owned(),
                title: Some("title".to_owned()),
                alt: "foo".to_owned(),
            })])]
        }
    );
}

#[test]
fn image2() {
    let doc = parse_markdown(MarkdownParserState::default(), "![foo](train.jpg)").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
                destination: "train.jpg".to_owned(),
                title: None,
                alt: "foo".to_owned(),
            })])]
        }
    );
}

#[test]
fn image3() {
    let doc = parse_markdown(MarkdownParserState::default(), "![foo](<url>)").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
                destination: "url".to_owned(),
                title: None,
                alt: "foo".to_owned(),
            })])]
        }
    );
}

/// Backslash escapes in the description are resolved, per CommonMark §6.4.
#[test]
fn image_alt_escapes_are_resolved() {
    let doc = parse_markdown(MarkdownParserState::default(), r"![Face N \(TD+\)](u.jpg)").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
                destination: "u.jpg".to_owned(),
                title: None,
                alt: "Face N (TD+)".to_owned(),
            })])]
        }
    );
}

/// An escaped `]` belongs to the description and must not end it, so a consumer that
/// escapes user text before splicing it into `![...]` keeps the image intact.
#[test]
fn image_alt_escaped_close_bracket() {
    let doc = parse_markdown(MarkdownParserState::default(), r"![a\](x)](u.jpg)").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
                destination: "u.jpg".to_owned(),
                title: None,
                alt: "a](x)".to_owned(),
            })])]
        }
    );
}

/// Only ASCII punctuation is escapable; `\n` stays two literal characters.
#[test]
fn image_alt_keeps_non_punctuation_escape() {
    let doc = parse_markdown(MarkdownParserState::default(), r"![a\nb](u.jpg)").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
                destination: "u.jpg".to_owned(),
                title: None,
                alt: r"a\nb".to_owned(),
            })])]
        }
    );
}

/// The scanner used when the input is not covered by an `InlineIndex` has to agree
/// with the index on where the description ends.
#[test]
fn image_alt_scanner_skips_escaped_brackets() {
    use crate::parser::inline::image::unescaped_close_bracket;

    assert_eq!(unescaped_close_bracket("a]b]"), Some(1));
    assert_eq!(unescaped_close_bracket(r"a\](x)]"), Some(6));
    assert_eq!(unescaped_close_bracket(r"a\\](x)"), Some(3));
    assert_eq!(unescaped_close_bracket("no bracket"), None);
    assert_eq!(unescaped_close_bracket(r"trailing\"), None);
    assert_eq!(unescaped_close_bracket(r"ä\]ö]"), Some(6));
}

#[test]
fn image4() {
    let doc = parse_markdown(MarkdownParserState::default(), "![](train.jpg)").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
                destination: "train.jpg".to_owned(),
                title: None,
                alt: "".to_owned(),
            })])]
        }
    );
}
