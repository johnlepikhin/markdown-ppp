use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn github_alert_note() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> [!NOTE]\n> This is a note",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Note,
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "This is a note".to_string()
                )])],
            })],
        }
    );
}

#[test]
fn github_alert_tip() {
    let doc = parse_markdown(MarkdownParserState::default(), "> [!TIP]\n> This is a tip").unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Tip,
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "This is a tip".to_string()
                )])],
            })],
        }
    );
}

#[test]
fn github_alert_important() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> [!IMPORTANT]\n> This is important",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Important,
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "This is important".to_string()
                )])],
            })],
        }
    );
}

#[test]
fn github_alert_warning() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> [!WARNING]\n> This is a warning",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Warning,
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "This is a warning".to_string()
                )])],
            })],
        }
    );
}

#[test]
fn github_alert_caution() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> [!CAUTION]\n> This is a caution",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Caution,
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "This is a caution".to_string()
                )])],
            })],
        }
    );
}

#[test]
fn github_alert_multiline() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> [!NOTE]\n> Line 1\n> Line 2\n> \n> Line 3",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Note,
                blocks: vec![
                    Block::Paragraph(vec![Inline::Text("Line 1\nLine 2".to_string())]),
                    Block::Paragraph(vec![Inline::Text("Line 3".to_string())])
                ],
            })],
        }
    );
}

#[test]
fn github_alert_with_formatting() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> [!TIP]\n> Use **bold** and *italic* text",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Tip,
                blocks: vec![Block::Paragraph(vec![
                    Inline::Text("Use ".to_string()),
                    Inline::Strong(vec![Inline::Text("bold".to_string())]),
                    Inline::Text(" and ".to_string()),
                    Inline::Emphasis(vec![Inline::Text("italic".to_string())]),
                    Inline::Text(" text".to_string())
                ])],
            })],
        }
    );
}

#[test]
fn github_alert_empty_content() {
    let doc = parse_markdown(MarkdownParserState::default(), "> [!WARNING]\n>").unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Warning,
                blocks: vec![],
            })],
        }
    );
}

#[test]
fn regular_blockquote_not_alert() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> This is not an alert\n> Just a regular blockquote",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::BlockQuote(vec![Block::Paragraph(vec![
                Inline::Text("This is not an alert\nJust a regular blockquote".to_string())
            ])])],
        }
    );
}

#[test]
fn github_alert_case_insensitive() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> [!note]\n> lowercase note",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Note,
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "lowercase note".to_string()
                )])],
            })],
        }
    );
}
