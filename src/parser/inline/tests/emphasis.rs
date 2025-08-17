use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn emphasis1() {
    let doc = parse_markdown(MarkdownParserState::default(), "*foo bar*").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Emphasis(vec![
                Inline::Text("foo bar".to_string())
            ])])],
        }
    );
}

#[test]
fn emphasis2() {
    let doc = parse_markdown(MarkdownParserState::default(), "* a *").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Star),
                items: vec![ListItem {
                    task: None,
                    blocks: vec![Block::Paragraph(vec![Inline::Text("a *".to_owned())])]
                }]
            })]
        }
    );
}

#[test]
fn emphasis3() {
    let doc = parse_markdown(MarkdownParserState::default(), "foo ___bar___").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Text("foo ".to_owned()),
                Inline::Strong(vec![Inline::Emphasis(vec![Inline::Text("bar".to_owned())])])
            ])]
        }
    );
}

#[test]
fn emphasis4() {
    let doc = parse_markdown(MarkdownParserState::default(), "**foo ___bar___ baz**").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Strong(vec![
                Inline::Text("foo ".to_owned()),
                Inline::Strong(vec![Inline::Emphasis(vec![Inline::Text("bar".to_owned())])]),
                Inline::Text(" baz".to_owned())
            ])])]
        }
    );
}

#[test]
#[ignore = "Known issue: underscores in words like PKG_CONFIG_PATH can be incorrectly parsed as emphasis"]
fn emphasis_with_underscores_in_words() {
    // Test case: PKG_CONFIG_PATH should not be parsed as PKG*CONFIG_PATH
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "Note that we set PKG_CONFIG_PATH only if it's not _already_ set",
    )
    .unwrap();

    // Debug output
    println!("Parsed document: {:?}", doc);

    // Expected: _already_ should be emphasized, PKG_CONFIG_PATH should remain as is
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Text("Note that we set PKG_CONFIG_PATH only if it's not ".to_string()),
                Inline::Emphasis(vec![Inline::Text("already".to_string())]),
                Inline::Text(" set".to_string())
            ])],
        }
    );
}

#[test]
fn test_simple_underscore() {
    let doc = parse_markdown(MarkdownParserState::default(), "_already_").unwrap();

    println!("Simple underscore: {:?}", doc);

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Emphasis(vec![
                Inline::Text("already".to_string())
            ])])],
        }
    );
}

#[test]
fn test_pkg_config() {
    let doc = parse_markdown(MarkdownParserState::default(), "PKG_CONFIG_PATH").unwrap();

    println!("PKG_CONFIG_PATH: {:?}", doc);

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "PKG_CONFIG_PATH".to_string()
            )])],
        }
    );
}
