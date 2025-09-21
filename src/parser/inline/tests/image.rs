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
