use crate::ast::*;
use crate::latex_printer::{config::*, render_latex};

#[test]
fn test_all_table_styles() {
    let doc = Document {
        blocks: vec![Block::Table(Table {
            rows: vec![
                vec![
                    vec![Inline::Text("H1".to_string())],
                    vec![Inline::Text("H2".to_string())],
                ],
                vec![
                    vec![Inline::Text("R1C1".to_string())],
                    vec![Inline::Text("R1C2".to_string())],
                ],
            ],
            alignments: vec![Alignment::Left, Alignment::Center],
        })],
    };

    // Test Tabular style
    let config_tabular = Config::default().with_table_style(TableStyle::Tabular);
    let result_tabular = render_latex(&doc, config_tabular);
    assert!(result_tabular.contains(r"\begin{tabular}[lc]"));
    assert!(result_tabular.contains(r"\hline"));
    assert!(!result_tabular.contains(r"\toprule"));

    // Test Longtabu style
    let config_longtabu = Config::default().with_table_style(TableStyle::Longtabu);
    let result_longtabu = render_latex(&doc, config_longtabu);
    assert!(result_longtabu.contains(r"\begin{longtabu}"));
    assert!(result_longtabu.contains("X[l] to \\textwidth"));

    // Test Booktabs style
    let config_booktabs = Config::default().with_table_style(TableStyle::Booktabs);
    let result_booktabs = render_latex(&doc, config_booktabs);
    assert!(result_booktabs.contains(r"\toprule"));
    assert!(result_booktabs.contains(r"\midrule"));
    assert!(result_booktabs.contains(r"\bottomrule"));
    assert!(!result_booktabs.contains(r"\hline"));
}

#[test]
fn test_all_code_block_styles() {
    let doc = Document {
        blocks: vec![Block::CodeBlock(CodeBlock {
            kind: CodeBlockKind::Fenced {
                info: Some("python".to_string()),
            },
            literal: "print('hello world')".to_string(),
        })],
    };

    // Test Verbatim style
    let config_verbatim = Config::default().with_code_block_style(CodeBlockStyle::Verbatim);
    let result_verbatim = render_latex(&doc, config_verbatim);
    assert!(result_verbatim.contains(r"\begin{verbatim}"));
    assert!(!result_verbatim.contains("language="));
    assert!(!result_verbatim.contains(r"\begin{minted}"));

    // Test Listings style
    let config_listings = Config::default().with_code_block_style(CodeBlockStyle::Listings);
    let result_listings = render_latex(&doc, config_listings);
    assert!(result_listings.contains(r"\begin{lstlisting}[language=python]"));
    assert!(!result_listings.contains(r"\begin{verbatim}"));
    assert!(!result_listings.contains(r"\begin{minted}"));

    // Test Minted style
    let config_minted = Config::default().with_code_block_style(CodeBlockStyle::Minted);
    let result_minted = render_latex(&doc, config_minted);
    assert!(result_minted.contains(r"\begin{minted}{python}"));
    assert!(!result_minted.contains(r"\begin{verbatim}"));
    assert!(!result_minted.contains(r"\begin{lstlisting}"));
}

#[test]
fn test_code_block_without_language() {
    let doc = Document {
        blocks: vec![Block::CodeBlock(CodeBlock {
            kind: CodeBlockKind::Fenced { info: None },
            literal: "echo 'no language'".to_string(),
        })],
    };

    // Verbatim should be the same
    let config_verbatim = Config::default().with_code_block_style(CodeBlockStyle::Verbatim);
    let result_verbatim = render_latex(&doc, config_verbatim);
    assert!(result_verbatim.contains(r"\begin{verbatim}"));

    // Listings without language should not have language option
    let config_listings = Config::default().with_code_block_style(CodeBlockStyle::Listings);
    let result_listings = render_latex(&doc, config_listings);
    assert!(result_listings.contains(r"\begin{lstlisting}"));
    assert!(!result_listings.contains("language="));

    // Minted should default to "text"
    let config_minted = Config::default().with_code_block_style(CodeBlockStyle::Minted);
    let result_minted = render_latex(&doc, config_minted);
    assert!(result_minted.contains(r"\begin{minted}{text}"));
}

#[test]
fn test_width_configuration() {
    let long_text = "This is a very long line of text that should be wrapped at different widths depending on the configuration settings.";
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(long_text.to_string())])],
    };

    let widths = vec![40, 80, 120];
    let mut results = Vec::new();

    for width in widths {
        let config = Config::default().with_width(width);
        let result = render_latex(&doc, config);
        results.push(result);
    }

    // Narrower width should generally result in more lines
    let line_counts: Vec<usize> = results.iter().map(|r| r.lines().count()).collect();
    assert!(line_counts[0] >= line_counts[1]); // 40 width >= 80 width
    assert!(line_counts[1] >= line_counts[2]); // 80 width >= 120 width
}

#[test]
fn test_config_combinations() {
    let doc = Document {
        blocks: vec![
            Block::Table(Table {
                rows: vec![
                    vec![vec![Inline::Text("Table".to_string())]],
                    vec![vec![Inline::Text("Data".to_string())]],
                ],
                alignments: vec![Alignment::Center],
            }),
            Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Fenced {
                    info: Some("rust".to_string()),
                },
                literal: "fn main() {}".to_string(),
            }),
        ],
    };

    // Test all combinations of table and code styles
    let table_styles = vec![
        TableStyle::Tabular,
        TableStyle::Longtabu,
        TableStyle::Booktabs,
    ];
    let code_styles = vec![
        CodeBlockStyle::Verbatim,
        CodeBlockStyle::Listings,
        CodeBlockStyle::Minted,
    ];

    for table_style in &table_styles {
        for code_style in &code_styles {
            let config = Config::default()
                .with_table_style(table_style.clone())
                .with_code_block_style(code_style.clone())
                .with_width(100);

            let result = render_latex(&doc, config);

            // Verify both elements are rendered according to their respective styles
            match table_style {
                TableStyle::Tabular => assert!(result.contains(r"\begin{tabular}")),
                TableStyle::Longtabu => assert!(result.contains(r"\begin{longtabu}")),
                TableStyle::Booktabs => assert!(result.contains(r"\toprule")),
            }

            match code_style {
                CodeBlockStyle::Verbatim => assert!(result.contains(r"\begin{verbatim}")),
                CodeBlockStyle::Listings => assert!(result.contains(r"\begin{lstlisting}")),
                CodeBlockStyle::Minted => assert!(result.contains(r"\begin{minted}")),
            }
        }
    }
}

#[test]
fn test_config_builder_pattern() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text("Test".to_string())])],
    };

    // Test method chaining
    let config = Config::default()
        .with_width(60)
        .with_table_style(TableStyle::Booktabs)
        .with_code_block_style(CodeBlockStyle::Minted);

    let result = render_latex(&doc, config);
    assert!(result.contains("Test"));

    // Verify each setting took effect by testing with elements that use them
    let complex_doc = Document {
        blocks: vec![
            Block::Table(Table {
                rows: vec![vec![vec![Inline::Text("H".to_string())]]],
                alignments: vec![Alignment::Left],
            }),
            Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Fenced {
                    info: Some("python".to_string()),
                },
                literal: "pass".to_string(),
            }),
        ],
    };

    let config = Config::default()
        .with_table_style(TableStyle::Booktabs)
        .with_code_block_style(CodeBlockStyle::Minted);

    let result = render_latex(&complex_doc, config);
    assert!(result.contains(r"\toprule"));
    assert!(result.contains(r"\begin{minted}{python}"));
}

#[test]
fn test_default_config() {
    let doc = Document {
        blocks: vec![
            Block::Table(Table {
                rows: vec![vec![vec![Inline::Text("Test".to_string())]]],
                alignments: vec![Alignment::Left],
            }),
            Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Fenced {
                    info: Some("rust".to_string()),
                },
                literal: "test".to_string(),
            }),
        ],
    };

    let result = render_latex(&doc, Config::default());

    // Default should be Tabular for tables and Verbatim for code
    assert!(result.contains(r"\begin{tabular}"));
    assert!(result.contains(r"\begin{verbatim}"));
    assert!(!result.contains(r"\toprule"));
    assert!(!result.contains(r"\begin{lstlisting}"));
    assert!(!result.contains(r"\begin{minted}"));
}

#[test]
fn test_config_equality() {
    let config1 = Config::default()
        .with_width(80)
        .with_table_style(TableStyle::Tabular)
        .with_code_block_style(CodeBlockStyle::Verbatim);

    let _config2 = Config::default()
        .with_width(80)
        .with_table_style(TableStyle::Tabular)
        .with_code_block_style(CodeBlockStyle::Verbatim);

    let config3 = Config::default()
        .with_width(100)
        .with_table_style(TableStyle::Tabular)
        .with_code_block_style(CodeBlockStyle::Verbatim);

    // Same configurations should be equal (if we implement PartialEq)
    // Different configurations should not be equal
    assert_ne!(config1.width, config3.width);
}

#[test]
fn test_mixed_content_with_different_configs() {
    let doc = Document {
        blocks: vec![
            Block::Heading(Heading {
                kind: HeadingKind::Atx(1),
                content: vec![Inline::Text("Document Title".to_string())],
            }),
            Block::Paragraph(vec![
                Inline::Text("This document contains a ".to_string()),
                Inline::Strong(vec![Inline::Text("table".to_string())]),
                Inline::Text(" and a ".to_string()),
                Inline::Emphasis(vec![Inline::Text("code block".to_string())]),
                Inline::Text(".".to_string()),
            ]),
            Block::Table(Table {
                rows: vec![
                    vec![
                        vec![Inline::Text("Feature".to_string())],
                        vec![Inline::Text("Status".to_string())],
                    ],
                    vec![
                        vec![Inline::Text("Tables".to_string())],
                        vec![Inline::Text("✓".to_string())],
                    ],
                ],
                alignments: vec![Alignment::Left, Alignment::Center],
            }),
            Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Fenced {
                    info: Some("rust".to_string()),
                },
                literal: "fn render_latex() -> String {\n    // Implementation\n}".to_string(),
            }),
            Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Star),
                items: vec![
                    ListItem {
                        task: Some(TaskState::Complete),
                        blocks: vec![Block::Paragraph(vec![Inline::Text(
                            "LaTeX printer implemented".to_string(),
                        )])],
                    },
                    ListItem {
                        task: Some(TaskState::Incomplete),
                        blocks: vec![Block::Paragraph(vec![Inline::Text(
                            "Documentation updated".to_string(),
                        )])],
                    },
                ],
            }),
        ],
    };

    // Test with different configurations
    let configs = vec![
        Config::default(),
        Config::default()
            .with_table_style(TableStyle::Booktabs)
            .with_code_block_style(CodeBlockStyle::Listings),
        Config::default()
            .with_table_style(TableStyle::Longtabu)
            .with_code_block_style(CodeBlockStyle::Minted)
            .with_width(60),
    ];

    for config in configs {
        let result = render_latex(&doc, config);

        // All should contain basic elements
        assert!(result.contains(r"\section{Document Title}"));
        assert!(result.contains(r"\textbf{table}"));
        assert!(result.contains(r"\textit{code block}"));
        assert!(result.contains(r"\begin{itemize}"));
        assert!(result.contains(r"$\boxtimes$"));
        assert!(result.contains(r"$\square$"));

        // Should be valid non-empty output
        assert!(!result.trim().is_empty());
        assert!(result.lines().count() > 10);
    }
}
