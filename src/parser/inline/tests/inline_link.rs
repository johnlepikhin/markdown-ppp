use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn inline_link_with_nested_image() {
    // Common GitHub README badge pattern: [![alt](img-url)](link-url)
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "[![userstyles](https://img.shields.io/badge/userstyles-green)](https://userstyles.world/user/Paul-16098)",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
                destination: "https://userstyles.world/user/Paul-16098".to_owned(),
                title: None,
                children: vec![Inline::Image(Image {
                    destination: "https://img.shields.io/badge/userstyles-green".to_owned(),
                    title: None,
                    alt: "userstyles".to_owned(),
                })]
            })])]
        }
    );
}

#[test]
fn inline_link1() {
    let doc = parse_markdown(MarkdownParserState::default(), "[foo](/url \"title\")").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
                destination: "/url".to_owned(),
                title: Some("title".to_owned()),
                children: vec![Inline::Text("foo".to_owned())]
            })])]
        }
    );
}

#[test]
fn inline_link2() {
    let doc = parse_markdown(MarkdownParserState::default(), "[foo](train.jpg)").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
                destination: "train.jpg".to_owned(),
                title: None,
                children: vec![Inline::Text("foo".to_owned())]
            })])]
        }
    );
}

#[test]
fn inline_link3() {
    let doc = parse_markdown(MarkdownParserState::default(), "[foo](<url>)").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
                destination: "url".to_owned(),
                title: None,
                children: vec![Inline::Text("foo".to_owned())]
            })])]
        }
    );
}

#[test]
fn inline_link_badge_pattern() {
    // Common GitHub README badge pattern
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "[![Build Status](https://travis-ci.org/user/repo.svg)](https://travis-ci.org/user/repo)",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
                destination: "https://travis-ci.org/user/repo".to_owned(),
                title: None,
                children: vec![Inline::Image(Image {
                    destination: "https://travis-ci.org/user/repo.svg".to_owned(),
                    title: None,
                    alt: "Build Status".to_owned(),
                })]
            })])]
        }
    );
}

#[test]
fn inline_link_with_nested_brackets() {
    // Nested brackets without image syntax - [nested] is parsed as LinkReference
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "[text [nested] more](https://example.com)",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
                destination: "https://example.com".to_owned(),
                title: None,
                children: vec![
                    Inline::Text("text ".to_owned()),
                    Inline::LinkReference(LinkReference {
                        label: vec![Inline::Text("nested".to_owned())],
                        text: vec![Inline::Text("nested".to_owned())],
                    }),
                    Inline::Text(" more".to_owned()),
                ]
            })])]
        }
    );
}

#[test]
fn inline_link_deeply_nested_brackets() {
    // Deep nesting of brackets - inner brackets parsed as LinkReferences
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "[a [b [c] d] e](https://example.com)",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
                destination: "https://example.com".to_owned(),
                title: None,
                children: vec![
                    Inline::Text("a ".to_owned()),
                    Inline::LinkReference(LinkReference {
                        label: vec![
                            Inline::Text("b ".to_owned()),
                            Inline::LinkReference(LinkReference {
                                label: vec![Inline::Text("c".to_owned())],
                                text: vec![Inline::Text("c".to_owned())],
                            }),
                            Inline::Text(" d".to_owned()),
                        ],
                        text: vec![
                            Inline::Text("b ".to_owned()),
                            Inline::LinkReference(LinkReference {
                                label: vec![Inline::Text("c".to_owned())],
                                text: vec![Inline::Text("c".to_owned())],
                            }),
                            Inline::Text(" d".to_owned()),
                        ],
                    }),
                    Inline::Text(" e".to_owned()),
                ]
            })])]
        }
    );
}

#[test]
fn inline_link_multiple_images() {
    // Multiple images inside a single link
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "[![a](url1) ![b](url2)](main-url)",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
                destination: "main-url".to_owned(),
                title: None,
                children: vec![
                    Inline::Image(Image {
                        destination: "url1".to_owned(),
                        title: None,
                        alt: "a".to_owned(),
                    }),
                    Inline::Text(" ".to_owned()),
                    Inline::Image(Image {
                        destination: "url2".to_owned(),
                        title: None,
                        alt: "b".to_owned(),
                    }),
                ]
            })])]
        }
    );
}

#[test]
fn inline_link_with_escaped_closing_bracket() {
    // Escaped ] allows including literal ] in link text
    let doc = parse_markdown(
        MarkdownParserState::default(),
        r"[text with \] bracket](https://example.com)",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
                destination: "https://example.com".to_owned(),
                title: None,
                children: vec![Inline::Text("text with ] bracket".to_owned())]
            })])]
        }
    );
}

#[test]
fn inline_link_label_at_length_boundary() {
    // Label with 999 characters should succeed
    let label_999 = "a".repeat(999);
    let input = format!("[{label_999}](url)");
    let doc = parse_markdown(MarkdownParserState::default(), &input).unwrap();

    assert!(matches!(
        &doc.blocks[0],
        Block::Paragraph(inlines) if matches!(inlines.first(), Some(Inline::Link(_)))
    ));

    // Label with 1000 characters should fail to parse as link (falls back to text)
    let label_1000 = "a".repeat(1000);
    let input = format!("[{label_1000}](url)");
    let doc = parse_markdown(MarkdownParserState::default(), &input).unwrap();

    // Should not be parsed as a link
    assert!(matches!(
        &doc.blocks[0],
        Block::Paragraph(inlines) if !matches!(inlines.first(), Some(Inline::Link(_)))
    ));
}

#[test]
fn inline_link_unbalanced_opening_bracket() {
    // Unbalanced opening bracket - should not parse as link
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "[text [ unbalanced](https://example.com)",
    )
    .unwrap();

    // Should not be parsed as a single link due to unbalanced brackets
    // The parser will interpret this differently
    assert!(matches!(
        &doc.blocks[0],
        Block::Paragraph(inlines) if inlines.len() > 1 || !matches!(inlines.first(), Some(Inline::Link(link)) if link.destination == "https://example.com")
    ));
}

#[test]
fn inline_link_moderate_nesting_depth() {
    // Test that moderate nesting (under MAX_BRACKET_DEPTH = 32) works
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "[a [b [c [d [e](url1)](url2)](url3)](url4)](url5)",
    )
    .unwrap();

    // Should parse as nested structure
    assert!(matches!(&doc.blocks[0], Block::Paragraph(inlines) if !inlines.is_empty()));
}

#[test]
fn inline_link_empty_nested_brackets() {
    // Empty nested brackets are preserved as literal text
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "[text [] more](https://example.com)",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
                destination: "https://example.com".to_owned(),
                title: None,
                children: vec![Inline::Text("text [] more".to_owned())]
            })])]
        }
    );
}
