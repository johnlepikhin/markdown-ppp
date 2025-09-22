//! Comprehensive LaTeX printer usage examples
//!
//! This example demonstrates all features of the LaTeX printer including
//! different configuration options and element types.

use markdown_ppp::ast::*;
use markdown_ppp::latex_printer::{config::*, render_latex};

fn main() {
    println!("=== LaTeX Printer Examples ===\n");

    // Example 1: Basic usage
    basic_example();

    // Example 2: Table styles
    table_styles_example();

    // Example 3: Code block styles
    code_block_styles_example();

    // Example 4: Complex document
    complex_document_example();

    // Example 5: LaTeX escaping
    escaping_example();
}

fn basic_example() {
    println!("## Basic Usage\n");

    let doc = Document {
        blocks: vec![
            Block::Heading(Heading {
                kind: HeadingKind::Atx(1),
                content: vec![Inline::Text("Getting Started".to_string())],
            }),
            Block::Paragraph(vec![
                Inline::Text("This is a ".to_string()),
                Inline::Strong(vec![Inline::Text("simple".to_string())]),
                Inline::Text(" example of LaTeX generation.".to_string()),
            ]),
        ],
    };

    let latex = render_latex(&doc, Config::default());
    println!("```latex\n{latex}\n```\n");
}

fn table_styles_example() {
    println!("## Table Style Comparison\n");

    let table_doc = Document {
        blocks: vec![Block::Table(Table {
            rows: vec![
                vec![
                    vec![Inline::Text("Feature".to_string())],
                    vec![Inline::Text("Supported".to_string())],
                ],
                vec![
                    vec![Inline::Text("Tables".to_string())],
                    vec![Inline::Text("✓".to_string())],
                ],
                vec![
                    vec![Inline::Text("Code Blocks".to_string())],
                    vec![Inline::Text("✓".to_string())],
                ],
            ],
            alignments: vec![Alignment::Left, Alignment::Center],
        })],
    };

    let styles = vec![
        (TableStyle::Tabular, "Tabular (default)"),
        (TableStyle::Longtabu, "Longtabu (page breaks)"),
        (TableStyle::Booktabs, "Booktabs (beautiful)"),
    ];

    for (style, description) in styles {
        println!("### {description}\n");
        let config = Config::default().with_table_style(style);
        let latex = render_latex(&table_doc, config);
        println!("```latex\n{latex}\n```\n");
    }
}

fn code_block_styles_example() {
    println!("## Code Block Style Comparison\n");

    let code_doc = Document {
        blocks: vec![Block::CodeBlock(CodeBlock {
            kind: CodeBlockKind::Fenced {
                info: Some("rust".to_string()),
            },
            literal: "fn fibonacci(n: u32) -> u32 {\n    match n {\n        0 => 0,\n        1 => 1,\n        _ => fibonacci(n - 1) + fibonacci(n - 2),\n    }\n}".to_string(),
        })],
    };

    let styles = vec![
        (CodeBlockStyle::Verbatim, "Verbatim (no highlighting)"),
        (CodeBlockStyle::Listings, "Listings (basic highlighting)"),
        (CodeBlockStyle::Minted, "Minted (advanced highlighting)"),
    ];

    for (style, description) in styles {
        println!("### {description}\n");
        let config = Config::default().with_code_block_style(style);
        let latex = render_latex(&code_doc, config);
        println!("```latex\n{latex}\n```\n");
    }
}

fn complex_document_example() {
    println!("## Complex Document Example\n");

    let doc = Document {
        blocks: vec![
            Block::Heading(Heading {
                kind: HeadingKind::Atx(1),
                content: vec![Inline::Text("LaTeX Printer Guide".to_string())],
            }),
            Block::Paragraph(vec![
                Inline::Text("The LaTeX printer supports ".to_string()),
                Inline::Emphasis(vec![Inline::Text("all".to_string())]),
                Inline::Text(" CommonMark and GFM features.".to_string()),
            ]),
            Block::Heading(Heading {
                kind: HeadingKind::Atx(2),
                content: vec![Inline::Text("Features".to_string())],
            }),
            Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Star),
                items: vec![
                    ListItem {
                        task: Some(TaskState::Complete),
                        blocks: vec![Block::Paragraph(vec![
                            Inline::Text("Text formatting: ".to_string()),
                            Inline::Strong(vec![Inline::Text("bold".to_string())]),
                            Inline::Text(", ".to_string()),
                            Inline::Emphasis(vec![Inline::Text("italic".to_string())]),
                            Inline::Text(", and ".to_string()),
                            Inline::Strikethrough(vec![Inline::Text("strikethrough".to_string())]),
                        ])],
                    },
                    ListItem {
                        task: Some(TaskState::Complete),
                        blocks: vec![Block::Paragraph(vec![
                            Inline::Text("Code: ".to_string()),
                            Inline::Code("println!(\"Hello, LaTeX!\")".to_string()),
                        ])],
                    },
                    ListItem {
                        task: Some(TaskState::Complete),
                        blocks: vec![Block::Paragraph(vec![
                            Inline::Text("Links: ".to_string()),
                            Inline::Link(Link {
                                destination: "https://latex-project.org".to_string(),
                                title: Some("LaTeX Project".to_string()),
                                children: vec![Inline::Text("LaTeX".to_string())],
                            }),
                        ])],
                    },
                ],
            }),
            Block::BlockQuote(vec![Block::Paragraph(vec![Inline::Text(
                "LaTeX is a high-quality typesetting system.".to_string(),
            )])]),
            Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Note,
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "This is rendered as a footnote in LaTeX.".to_string(),
                )])],
            }),
        ],
    };

    let config = Config::default()
        .with_width(100)
        .with_table_style(TableStyle::Booktabs)
        .with_code_block_style(CodeBlockStyle::Listings);

    let latex = render_latex(&doc, config);
    println!("```latex\n{latex}\n```\n");
}

fn escaping_example() {
    println!("## LaTeX Character Escaping\n");

    let doc = Document {
        blocks: vec![
            Block::Heading(Heading {
                kind: HeadingKind::Atx(2),
                content: vec![Inline::Text("Special Characters".to_string())],
            }),
            Block::Paragraph(vec![Inline::Text(
                "These characters are automatically escaped: $ & % # ^ _ { } ~ \\".to_string(),
            )]),
            Block::Paragraph(vec![
                Inline::Text("Math expressions like ".to_string()),
                Inline::Code("x^2 + y_1".to_string()),
                Inline::Text(" are safely rendered.".to_string()),
            ]),
            Block::Paragraph(vec![Inline::Text(
                "Percentages like 50% and prices like $100 work correctly.".to_string(),
            )]),
        ],
    };

    let latex = render_latex(&doc, Config::default());
    println!("```latex\n{latex}\n```\n");
}
