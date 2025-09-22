use crate::ast::*;
use crate::latex_printer::{config::*, render_latex};

#[test]
fn test_empty_document() {
    let doc = Document { blocks: vec![] };

    let result = render_latex(&doc, Config::default());
    assert_eq!(result.trim(), "");
}

#[test]
fn test_empty_paragraph() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![])],
    };

    let result = render_latex(&doc, Config::default());
    assert_eq!(result.trim(), "");
}

#[test]
fn test_empty_heading() {
    let doc = Document {
        blocks: vec![Block::Heading(Heading {
            kind: HeadingKind::Atx(1),
            content: vec![],
        })],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\section{}"));
}

#[test]
fn test_empty_emphasis() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Text with ".to_string()),
            Inline::Emphasis(vec![]),
            Inline::Text(" empty emphasis.".to_string()),
        ])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\textit{}"));
}

#[test]
fn test_empty_list() {
    let doc = Document {
        blocks: vec![Block::List(List {
            kind: ListKind::Bullet(ListBulletKind::Star),
            items: vec![],
        })],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\begin{itemize}"));
    assert!(result.contains(r"\end{itemize}"));
}

#[test]
fn test_empty_list_item() {
    let doc = Document {
        blocks: vec![Block::List(List {
            kind: ListKind::Bullet(ListBulletKind::Star),
            items: vec![ListItem {
                task: None,
                blocks: vec![],
            }],
        })],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\item"));
}

#[test]
fn test_empty_table() {
    let doc = Document {
        blocks: vec![Block::Table(Table {
            rows: vec![],
            alignments: vec![],
        })],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\begin{tabular}"));
}

#[test]
fn test_empty_table_row() {
    let doc = Document {
        blocks: vec![Block::Table(Table {
            rows: vec![vec![]],
            alignments: vec![],
        })],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\\"));
}

#[test]
fn test_empty_table_cell() {
    let doc = Document {
        blocks: vec![Block::Table(Table {
            rows: vec![vec![vec![]]],
            alignments: vec![Alignment::Left],
        })],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\\"));
}

#[test]
fn test_empty_code_block() {
    let doc = Document {
        blocks: vec![Block::CodeBlock(CodeBlock {
            kind: CodeBlockKind::Fenced { info: None },
            literal: "".to_string(),
        })],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\begin{verbatim}"));
    assert!(result.contains(r"\end{verbatim}"));
}

#[test]
fn test_empty_blockquote() {
    let doc = Document {
        blocks: vec![Block::BlockQuote(vec![])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\begin{quote}"));
    assert!(result.contains(r"\end{quote}"));
}

#[test]
fn test_whitespace_only_text() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("   ".to_string()),
            Inline::Text("\t\n".to_string()),
        ])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains("   "));
    assert!(result.contains("\t ")); // \n gets replaced with space in LaTeX
}

#[test]
fn test_special_chars_in_urls() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
            destination: "https://example.com/path?q=a&b=c#fragment".to_string(),
            title: None,
            children: vec![Inline::Text("link".to_string())],
        })])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\href"));
    // URL should be properly escaped
    assert!(result.contains(r"https://example.com/path?q=a\&b=c\#fragment"));
}

#[test]
fn test_special_chars_in_code() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Code(
            "$ & % # ^ _ { } ~ \\".to_string(),
        )])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(
        r"\texttt{\$ \& \% \# \textasciicircum{} \_ \{ \} \textasciitilde{} \textbackslash{}}"
    ));
}

#[test]
fn test_unicode_characters() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(
            "Unicode: αβγ 中文 🚀 ñáéíóú".to_string(),
        )])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains("Unicode: αβγ 中文 🚀 ñáéíóú"));
}

#[test]
fn test_very_long_line() {
    // Use text with spaces so it can be wrapped
    let long_text = "word ".repeat(200); // Creates text with spaces
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(long_text.clone())])],
    };

    let config_narrow = Config::default().with_width(40);
    let config_wide = Config::default().with_width(120);

    let result_narrow = render_latex(&doc, config_narrow);
    let result_wide = render_latex(&doc, config_wide);

    // The narrow config should generally result in more lines (though exact behavior depends on pretty printer)
    let lines_narrow = result_narrow.lines().count();
    let lines_wide = result_wide.lines().count();

    // At minimum, both should produce output and narrow should not be shorter
    assert!(lines_narrow >= lines_wide);
    assert!(!result_narrow.trim().is_empty());
    assert!(!result_wide.trim().is_empty());
}

#[test]
fn test_deeply_nested_lists() {
    fn create_nested_list(depth: usize) -> Vec<Block> {
        if depth == 0 {
            vec![Block::Paragraph(vec![Inline::Text(
                "Deep item".to_string(),
            )])]
        } else {
            vec![Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Star),
                items: vec![ListItem {
                    task: None,
                    blocks: create_nested_list(depth - 1),
                }],
            })]
        }
    }

    let doc = Document {
        blocks: create_nested_list(5),
    };

    let result = render_latex(&doc, Config::default());
    // Should have multiple nested itemize environments
    let itemize_count = result.matches(r"\begin{itemize}").count();
    assert_eq!(itemize_count, 5);
}

#[test]
fn test_table_with_complex_cells() {
    let doc = Document {
        blocks: vec![Block::Table(Table {
            rows: vec![
                vec![
                    vec![Inline::Strong(vec![Inline::Text(
                        "Bold Header".to_string(),
                    )])],
                    vec![Inline::Emphasis(vec![Inline::Text(
                        "Italic Header".to_string(),
                    )])],
                ],
                vec![
                    vec![
                        Inline::Text("Cell with ".to_string()),
                        Inline::Code("code".to_string()),
                        Inline::Text(" and ".to_string()),
                        Inline::Link(Link {
                            destination: "https://example.com".to_string(),
                            title: None,
                            children: vec![Inline::Text("link".to_string())],
                        }),
                    ],
                    vec![
                        Inline::Strikethrough(vec![Inline::Text("crossed".to_string())]),
                        Inline::LineBreak,
                        Inline::Text("multiline".to_string()),
                    ],
                ],
            ],
            alignments: vec![Alignment::Left, Alignment::Right],
        })],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\textbf{Bold Header}"));
    assert!(result.contains(r"\textit{Italic Header}"));
    assert!(result.contains(r"\texttt{code}"));
    assert!(result.contains(r"\href"));
    assert!(result.contains(r"\sout{crossed}"));
    assert!(result.contains(r"\\"));
}

#[test]
fn test_task_list_states() {
    let doc = Document {
        blocks: vec![Block::List(List {
            kind: ListKind::Bullet(ListBulletKind::Plus),
            items: vec![
                ListItem {
                    task: Some(TaskState::Complete),
                    blocks: vec![Block::Paragraph(vec![Inline::Text(
                        "Completed task".to_string(),
                    )])],
                },
                ListItem {
                    task: Some(TaskState::Incomplete),
                    blocks: vec![Block::Paragraph(vec![Inline::Text(
                        "Incomplete task".to_string(),
                    )])],
                },
                ListItem {
                    task: None,
                    blocks: vec![Block::Paragraph(vec![Inline::Text(
                        "Regular item".to_string(),
                    )])],
                },
            ],
        })],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"$\boxtimes$ Completed task"));
    assert!(result.contains(r"$\square$ Incomplete task"));
    assert!(result.contains(r"\item Regular item"));
}

#[test]
fn test_all_list_bullet_kinds() {
    let bullet_kinds = vec![
        ListBulletKind::Star,
        ListBulletKind::Dash,
        ListBulletKind::Plus,
    ];

    for kind in bullet_kinds {
        let doc = Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Bullet(kind),
                items: vec![ListItem {
                    task: None,
                    blocks: vec![Block::Paragraph(vec![Inline::Text("Item".to_string())])],
                }],
            })],
        };

        let result = render_latex(&doc, Config::default());
        assert!(result.contains(r"\begin{itemize}"));
        assert!(result.contains(r"\item Item"));
    }
}

#[test]
fn test_ordered_list_with_custom_start() {
    let doc = Document {
        blocks: vec![Block::List(List {
            kind: ListKind::Ordered(ListOrderedKindOptions { start: 42 }),
            items: vec![
                ListItem {
                    task: None,
                    blocks: vec![Block::Paragraph(vec![Inline::Text("First".to_string())])],
                },
                ListItem {
                    task: None,
                    blocks: vec![Block::Paragraph(vec![Inline::Text("Second".to_string())])],
                },
            ],
        })],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\begin{enumerate}"));
    assert!(result.contains(r"\item First"));
    assert!(result.contains(r"\item Second"));
    // Note: LaTeX enumerate doesn't directly support custom start in this implementation
    // This is a limitation we could address in future versions
}

#[test]
fn test_table_alignment_edge_cases() {
    let doc = Document {
        blocks: vec![Block::Table(Table {
            rows: vec![vec![
                vec![Inline::Text("A".to_string())],
                vec![Inline::Text("B".to_string())],
                vec![Inline::Text("C".to_string())],
            ]],
            alignments: vec![Alignment::None, Alignment::Left], // Fewer alignments than columns
        })],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\begin{tabular}[ll]")); // Should default to left for missing alignments
}
