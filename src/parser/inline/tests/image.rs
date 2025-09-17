use crate::ast::*;
use crate::parser::{ parse_markdown, MarkdownParserState };

#[test]
fn image1() {
  let doc = parse_markdown(MarkdownParserState::default(), "![foo](/url \"title\")").unwrap();
  assert_eq!(doc, Document {
    blocks: vec![
      Block::Paragraph(
        vec![
          Inline::Image(Image {
            destination: "/url".to_owned(),
            title: Some("title".to_owned()),
            alt: Some("foo".to_owned()),
          })
        ]
      )
    ],
  });
}

#[test]
fn image2() {
  let doc = parse_markdown(MarkdownParserState::default(), "![foo](train.jpg)").unwrap();
  assert_eq!(doc, Document {
    blocks: vec![
      Block::Paragraph(
        vec![
          Inline::Image(Image {
            destination: "train.jpg".to_owned(),
            title: None,
            alt: Some("foo".to_owned()),
          })
        ]
      )
    ],
  });
}

#[test]
fn image3() {
  let doc = parse_markdown(MarkdownParserState::default(), "![foo](<url>)").unwrap();
  assert_eq!(doc, Document {
    blocks: vec![
      Block::Paragraph(
        vec![
          Inline::Image(Image {
            destination: "url".to_owned(),
            title: None,
            alt: Some("foo".to_owned()),
          })
        ]
      )
    ],
  });
}
#[test]
fn no_alt_image1() {
  let doc = parse_markdown(MarkdownParserState::default(), "![](/url \"title\")").unwrap();
  assert_eq!(doc, Document {
    blocks: vec![
      Block::Paragraph(
        vec![
          Inline::Image(Image {
            destination: "/url".to_owned(),
            title: Some("title".to_owned()),
            alt: None,
          })
        ]
      )
    ],
  });
}

#[test]
fn no_alt_image2() {
  let doc = parse_markdown(MarkdownParserState::default(), "![](image.jpg)").unwrap();
  assert_eq!(doc, Document {
    blocks: vec![
      Block::Paragraph(
        vec![
          Inline::Image(Image {
            destination: "image.jpg".to_owned(),
            title: None,
            alt: None,
          })
        ]
      )
    ],
  });
}
#[test]
fn no_alt_image3() {
  let doc = parse_markdown(MarkdownParserState::default(), "![](<url>)").unwrap();
  assert_eq!(doc, Document {
    blocks: vec![
      Block::Paragraph(
        vec![
          Inline::Image(Image {
            destination: "url".to_owned(),
            title: None,
            alt: None,
          })
        ]
      )
    ],
  });
}
