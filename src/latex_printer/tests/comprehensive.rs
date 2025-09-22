use crate::ast::*;
use crate::latex_printer::{config::*, render_latex};

#[test]
fn test_thematic_break() {
    let doc = Document {
        blocks: vec![Block::ThematicBreak],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\hrule"));
}

#[test]
fn test_html_block() {
    let doc = Document {
        blocks: vec![Block::HtmlBlock("<div>Raw HTML</div>".to_string())],
    };

    let result = render_latex(&doc, Config::default());
    // HTML should be escaped
    assert!(result.contains(r"<div>Raw HTML</div>"));
}

#[test]
fn test_definition_block() {
    let doc = Document {
        blocks: vec![Block::Definition(LinkDefinition {
            label: vec![Inline::Text("example".to_string())],
            destination: "https://example.com".to_string(),
            title: Some("Example Site".to_string()),
        })],
    };

    let result = render_latex(&doc, Config::default());
    // Definitions should produce no output (handled during inline processing)
    assert_eq!(result.trim(), "");
}

#[test]
fn test_footnote_definition() {
    let doc = Document {
        blocks: vec![Block::FootnoteDefinition(FootnoteDefinition {
            label: "note1".to_string(),
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "This is a footnote.".to_string(),
            )])],
        })],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\footnotetext"));
    assert!(result.contains("[1]"));
    assert!(result.contains("This is a footnote."));
}

#[test]
fn test_github_alerts() {
    let alert_types = vec![
        (GitHubAlertType::Note, "Note"),
        (GitHubAlertType::Tip, "Tip"),
        (GitHubAlertType::Important, "Important"),
        (GitHubAlertType::Warning, "Warning"),
        (GitHubAlertType::Caution, "Caution"),
    ];

    for (alert_type, expected_text) in alert_types {
        let doc = Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type,
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "Alert content".to_string(),
                )])],
            })],
        };

        let result = render_latex(&doc, Config::default());
        assert!(result.contains(r"\footnote"));
        assert!(result.contains(expected_text));
        assert!(result.contains("Alert content"));
    }
}

#[test]
fn test_empty_block() {
    let doc = Document {
        blocks: vec![
            Block::Paragraph(vec![Inline::Text("Before".to_string())]),
            Block::Empty,
            Block::Paragraph(vec![Inline::Text("After".to_string())]),
        ],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains("Before"));
    assert!(result.contains("After"));
    // Empty blocks should not add any content
    let lines: Vec<&str> = result
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    assert_eq!(lines.len(), 2);
}

#[test]
fn test_line_break() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Line 1".to_string()),
            Inline::LineBreak,
            Inline::Text("Line 2".to_string()),
        ])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"Line 1\\"));
    assert!(result.contains("Line 2"));
}

#[test]
fn test_inline_code() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Use ".to_string()),
            Inline::Code("println!()".to_string()),
            Inline::Text(" function.".to_string()),
        ])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\texttt{println!()}"));
}

#[test]
fn test_inline_html() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Some ".to_string()),
            Inline::Html("<em>HTML</em>".to_string()),
            Inline::Text(" content.".to_string()),
        ])],
    };

    let result = render_latex(&doc, Config::default());
    // HTML should be escaped
    assert!(result.contains("<em>HTML</em>"));
}

#[test]
fn test_link_reference() {
    let doc = Document {
        blocks: vec![
            Block::Definition(LinkDefinition {
                label: vec![Inline::Text("example".to_string())],
                destination: "https://example.com".to_string(),
                title: None,
            }),
            Block::Paragraph(vec![
                Inline::Text("Visit ".to_string()),
                Inline::LinkReference(LinkReference {
                    label: vec![Inline::Text("example".to_string())],
                    text: vec![Inline::Text("this site".to_string())],
                }),
                Inline::Text(".".to_string()),
            ]),
        ],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\href"));
    assert!(result.contains("https://example.com"));
    assert!(result.contains("this site"));
}

#[test]
fn test_link_reference_unresolved() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::LinkReference(
            LinkReference {
                label: vec![Inline::Text("missing".to_string())],
                text: vec![Inline::Text("broken link".to_string())],
            },
        )])],
    };

    let result = render_latex(&doc, Config::default());
    // Should fallback to text representation
    assert!(result.contains("[broken link][missing]"));
}

#[test]
fn test_image() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
            destination: "image.png".to_string(),
            title: Some("My Image".to_string()),
            alt: "Alt text".to_string(),
        })])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\includegraphics"));
    assert!(result.contains("image.png"));
    assert!(result.contains(r"\caption"));
    assert!(result.contains("Alt text"));
}

#[test]
fn test_image_no_alt() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
            destination: "image.png".to_string(),
            title: None,
            alt: "".to_string(),
        })])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\includegraphics"));
    assert!(result.contains("image.png"));
    assert!(!result.contains(r"\caption"));
}

#[test]
fn test_strikethrough() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("This is ".to_string()),
            Inline::Strikethrough(vec![Inline::Text("crossed out".to_string())]),
            Inline::Text(" text.".to_string()),
        ])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\sout{crossed out}"));
}

#[test]
fn test_autolink() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Visit ".to_string()),
            Inline::Autolink("https://example.com".to_string()),
            Inline::Text(".".to_string()),
        ])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\url{https://example.com}"));
}

#[test]
fn test_footnote_reference() {
    let doc = Document {
        blocks: vec![
            Block::FootnoteDefinition(FootnoteDefinition {
                label: "note1".to_string(),
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "Footnote content".to_string(),
                )])],
            }),
            Block::Paragraph(vec![
                Inline::Text("Text with footnote".to_string()),
                Inline::FootnoteReference("note1".to_string()),
                Inline::Text(".".to_string()),
            ]),
        ],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\footnotemark"));
    assert!(result.contains("[1]"));
    assert!(result.contains(r"\footnotetext"));
}

#[test]
fn test_footnote_reference_unresolved() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Text".to_string()),
            Inline::FootnoteReference("missing".to_string()),
            Inline::Text(".".to_string()),
        ])],
    };

    let result = render_latex(&doc, Config::default());
    // Should fallback to text representation
    assert!(result.contains("[^missing]"));
}

#[test]
fn test_empty_inline() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Before".to_string()),
            Inline::Empty,
            Inline::Text("After".to_string()),
        ])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains("BeforeAfter"));
}

#[test]
fn test_all_heading_levels() {
    let doc = Document {
        blocks: vec![
            Block::Heading(Heading {
                kind: HeadingKind::Atx(1),
                content: vec![Inline::Text("Level 1".to_string())],
            }),
            Block::Heading(Heading {
                kind: HeadingKind::Atx(2),
                content: vec![Inline::Text("Level 2".to_string())],
            }),
            Block::Heading(Heading {
                kind: HeadingKind::Atx(3),
                content: vec![Inline::Text("Level 3".to_string())],
            }),
            Block::Heading(Heading {
                kind: HeadingKind::Atx(4),
                content: vec![Inline::Text("Level 4".to_string())],
            }),
            Block::Heading(Heading {
                kind: HeadingKind::Atx(5),
                content: vec![Inline::Text("Level 5".to_string())],
            }),
            Block::Heading(Heading {
                kind: HeadingKind::Atx(6),
                content: vec![Inline::Text("Level 6".to_string())],
            }),
            Block::Heading(Heading {
                kind: HeadingKind::Setext(SetextHeading::Level2),
                content: vec![Inline::Text("Setext 2".to_string())],
            }),
        ],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\section{Level 1}"));
    assert!(result.contains(r"\subsection{Level 2}"));
    assert!(result.contains(r"\subsubsection{Level 3}"));
    assert!(result.contains(r"\paragraph{Level 4}"));
    assert!(result.contains(r"\subparagraph{Level 5}"));
    assert!(result.contains(r"\subparagraph{Level 6}"));
    assert!(result.contains(r"\subsection{Setext 2}"));
}

#[test]
fn test_indented_code_block() {
    let doc = Document {
        blocks: vec![Block::CodeBlock(CodeBlock {
            kind: CodeBlockKind::Indented,
            literal: "def hello():\n    print('Hello')".to_string(),
        })],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\begin{verbatim}"));
    assert!(result.contains("def hello():"));
}

#[test]
fn test_table_longtabu() {
    let doc = Document {
        blocks: vec![Block::Table(Table {
            rows: vec![
                vec![
                    vec![Inline::Text("A".to_string())],
                    vec![Inline::Text("B".to_string())],
                ],
                vec![
                    vec![Inline::Text("1".to_string())],
                    vec![Inline::Text("2".to_string())],
                ],
            ],
            alignments: vec![Alignment::Center, Alignment::Right],
        })],
    };

    let config = Config::default().with_table_style(TableStyle::Longtabu);
    let result = render_latex(&doc, config);
    assert!(result.contains(r"\begin{longtabu}"));
    assert!(result.contains("X[l] to \\textwidth"));
    assert!(result.contains("{cr}"));
}

#[test]
fn test_table_booktabs() {
    let doc = Document {
        blocks: vec![Block::Table(Table {
            rows: vec![
                vec![
                    vec![Inline::Text("Header".to_string())],
                    vec![Inline::Text("Value".to_string())],
                ],
                vec![
                    vec![Inline::Text("Row1".to_string())],
                    vec![Inline::Text("Data1".to_string())],
                ],
            ],
            alignments: vec![Alignment::Left, Alignment::Center],
        })],
    };

    let config = Config::default().with_table_style(TableStyle::Booktabs);
    let result = render_latex(&doc, config);
    assert!(result.contains(r"\toprule"));
    assert!(result.contains(r"\midrule"));
    assert!(result.contains(r"\bottomrule"));
    assert!(result.contains(r"\begin{tabular}[lc]"));
}

#[test]
fn test_nested_elements() {
    let doc = Document {
        blocks: vec![
            Block::BlockQuote(vec![
                Block::Paragraph(vec![Inline::Text("Quote paragraph".to_string())]),
                Block::List(List {
                    kind: ListKind::Bullet(ListBulletKind::Dash),
                    items: vec![ListItem {
                        task: None,
                        blocks: vec![Block::Paragraph(vec![
                            Inline::Text("Item with ".to_string()),
                            Inline::Strong(vec![Inline::Text("bold".to_string())]),
                            Inline::Text(" text".to_string()),
                        ])],
                    }],
                }),
            ]),
            Block::List(List {
                kind: ListKind::Ordered(ListOrderedKindOptions { start: 5 }),
                items: vec![ListItem {
                    task: Some(TaskState::Incomplete),
                    blocks: vec![
                        Block::Paragraph(vec![Inline::Text("Multi-block item".to_string())]),
                        Block::CodeBlock(CodeBlock {
                            kind: CodeBlockKind::Fenced {
                                info: Some("bash".to_string()),
                            },
                            literal: "echo 'nested code'".to_string(),
                        }),
                    ],
                }],
            }),
        ],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\begin{quote}"));
    assert!(result.contains(r"\begin{itemize}"));
    assert!(result.contains(r"\begin{enumerate}"));
    assert!(result.contains(r"\textbf{bold}"));
    assert!(result.contains(r"$\square$"));
    assert!(result.contains(r"\begin{verbatim}"));
    assert!(result.contains("echo 'nested code'"));
}

#[test]
fn test_complex_inline_combinations() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("This is ".to_string()),
            Inline::Strong(vec![
                Inline::Text("bold with ".to_string()),
                Inline::Emphasis(vec![Inline::Text("italic".to_string())]),
                Inline::Text(" inside".to_string()),
            ]),
            Inline::Text(" and ".to_string()),
            Inline::Strikethrough(vec![
                Inline::Text("struck ".to_string()),
                Inline::Code("code".to_string()),
            ]),
            Inline::Text(" text.".to_string()),
        ])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\textbf{bold with \textit{italic} inside}"));
    assert!(result.contains(r"\sout{struck \texttt{code}}"));
}
