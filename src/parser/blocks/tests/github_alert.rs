use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};
use crate::printer::{render_markdown, config::Config as PrinterConfig};

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

#[test]
fn github_alert_custom_simple() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> [!CUSTOM]\n> This is a custom alert",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Custom("CUSTOM".to_string()),
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "This is a custom alert".to_string()
                )])],
            })],
        }
    );
}

#[test]
fn github_alert_custom_with_numbers() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> [!ALERT123]\n> Custom alert with numbers",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Custom("ALERT123".to_string()),
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "Custom alert with numbers".to_string()
                )])],
            })],
        }
    );
}

#[test]
fn github_alert_custom_with_underscores() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> [!MY_CUSTOM_ALERT]\n> Custom alert with underscores",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Custom("MY_CUSTOM_ALERT".to_string()),
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "Custom alert with underscores".to_string()
                )])],
            })],
        }
    );
}

#[test]
fn github_alert_custom_case_insensitive() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> [!custom]\n> lowercase custom alert",
    )
    .unwrap();

    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Custom("CUSTOM".to_string()),
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "lowercase custom alert".to_string()
                )])],
            })],
        }
    );
}

#[test]
fn github_alert_invalid_custom_starts_with_number() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> [!123INVALID]\n> Should not be parsed as alert",
    )
    .unwrap();

    // Should be parsed as regular blockquote since it doesn't start with a letter
    // The [!123INVALID] gets parsed as a reference link since it's not a valid alert
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::BlockQuote(vec![Block::Paragraph(vec![
                Inline::LinkReference(crate::ast::LinkReference {
                    label: vec![Inline::Text("!123INVALID".to_string())],
                    text: vec![Inline::Text("!123INVALID".to_string())],
                }),
                Inline::Text("\nShould not be parsed as alert".to_string())
            ])])],
        }
    );
}

#[test]
fn github_alert_invalid_custom_with_special_chars() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> [!CUSTOM-ALERT]\n> Should not be parsed as alert",
    )
    .unwrap();

    // Should be parsed as regular blockquote since it contains a dash
    // The [!CUSTOM-ALERT] gets parsed as a reference link since it's not a valid alert
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::BlockQuote(vec![Block::Paragraph(vec![
                Inline::LinkReference(crate::ast::LinkReference {
                    label: vec![Inline::Text("!CUSTOM-ALERT".to_string())],
                    text: vec![Inline::Text("!CUSTOM-ALERT".to_string())],
                }),
                Inline::Text("\nShould not be parsed as alert".to_string())
            ])])],
        }
    );
}

// Tests for printer (markdown rendering)

#[test]
fn github_alert_custom_printer_simple() {
    let doc = Document {
        blocks: vec![Block::GitHubAlert(GitHubAlert {
            alert_type: GitHubAlertType::Custom("CUSTOM".to_string()),
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "This is a custom alert".to_string()
            )])],
        })],
    };

    let rendered = render_markdown(&doc, PrinterConfig::default());
    assert_eq!(rendered, "> [!CUSTOM]\n> This is a custom alert");
}

#[test]
fn github_alert_custom_printer_with_underscores() {
    let doc = Document {
        blocks: vec![Block::GitHubAlert(GitHubAlert {
            alert_type: GitHubAlertType::Custom("MY_CUSTOM_ALERT".to_string()),
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "Custom alert with underscores".to_string()
            )])],
        })],
    };

    let rendered = render_markdown(&doc, PrinterConfig::default());
    assert_eq!(rendered, "> [!MY_CUSTOM_ALERT]\n> Custom alert with underscores");
}

#[test]
fn github_alert_custom_printer_with_numbers() {
    let doc = Document {
        blocks: vec![Block::GitHubAlert(GitHubAlert {
            alert_type: GitHubAlertType::Custom("ALERT123".to_string()),
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "Custom alert with numbers".to_string()
            )])],
        })],
    };

    let rendered = render_markdown(&doc, PrinterConfig::default());
    assert_eq!(rendered, "> [!ALERT123]\n> Custom alert with numbers");
}

#[test]
fn github_alert_custom_printer_multiline() {
    let doc = Document {
        blocks: vec![Block::GitHubAlert(GitHubAlert {
            alert_type: GitHubAlertType::Custom("MULTILINE".to_string()),
            blocks: vec![
                Block::Paragraph(vec![Inline::Text("First paragraph".to_string())]),
                Block::Paragraph(vec![Inline::Text("Second paragraph".to_string())]),
            ],
        })],
    };

    let rendered = render_markdown(&doc, PrinterConfig::default());
    assert_eq!(rendered, "> [!MULTILINE]\n> First paragraph\n>\n> Second paragraph");
}

// Roundtrip tests (parse -> render -> parse)

#[test]
fn github_alert_custom_roundtrip_simple() {
    let input = "> [!CUSTOM]\n> This is a custom alert";

    let doc1 = parse_markdown(MarkdownParserState::default(), input).unwrap();
    let rendered = render_markdown(&doc1, PrinterConfig::default());
    let doc2 = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    assert_eq!(doc1, doc2);
    assert_eq!(rendered, input);
}

#[test]
fn github_alert_custom_roundtrip_with_formatting() {
    let input = "> [!MY_ALERT]\n> Use **bold** and *italic* text";

    let doc1 = parse_markdown(MarkdownParserState::default(), input).unwrap();
    let rendered = render_markdown(&doc1, PrinterConfig::default());
    let doc2 = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    assert_eq!(doc1, doc2);
}

#[test]
fn github_alert_custom_roundtrip_multiline() {
    let input = "> [!COMPLEX_ALERT]\n> Line 1 Line 2\n> \n> Line 3";

    let doc1 = parse_markdown(MarkdownParserState::default(), input).unwrap();
    let rendered = render_markdown(&doc1, PrinterConfig::default());
    let doc2 = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    assert_eq!(doc1, doc2);
}

#[test]
fn github_alert_standard_types_roundtrip() {
    let test_cases = [
        ("> [!NOTE]\n> This is a note", GitHubAlertType::Note),
        ("> [!TIP]\n> This is a tip", GitHubAlertType::Tip),
        ("> [!IMPORTANT]\n> This is important", GitHubAlertType::Important),
        ("> [!WARNING]\n> This is a warning", GitHubAlertType::Warning),
        ("> [!CAUTION]\n> This is a caution", GitHubAlertType::Caution),
    ];

    for (input, expected_type) in test_cases {
        let doc1 = parse_markdown(MarkdownParserState::default(), input).unwrap();

        // Check the parsed alert type
        if let Some(Block::GitHubAlert(alert)) = doc1.blocks.first() {
            assert_eq!(alert.alert_type, expected_type);
        } else {
            panic!("Expected GitHubAlert, got: {:?}", doc1.blocks.first());
        }

        let rendered = render_markdown(&doc1, PrinterConfig::default());
        let doc2 = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

        assert_eq!(doc1, doc2, "Roundtrip failed for input: {}", input);
    }
}
