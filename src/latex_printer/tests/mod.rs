mod comprehensive;
mod config_combinations;
mod edge_cases;

use crate::ast::*;
use crate::latex_printer::{config::*, render_latex};

#[test]
fn test_simple_paragraph() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(
            "Hello, world!".to_string(),
        )])],
    };

    let result = render_latex(&doc, Config::default());
    assert_eq!(result.trim(), "Hello, world!");
}

#[test]
fn test_latex_escaping() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(
            "Special chars: $ & % # ^ _ { } ~ \\".to_string(),
        )])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\$"));
    assert!(result.contains(r"\&"));
    assert!(result.contains(r"\%"));
    assert!(result.contains(r"\#"));
    assert!(result.contains(r"\textasciicircum{}"));
    assert!(result.contains(r"\_"));
    assert!(result.contains(r"\{"));
    assert!(result.contains(r"\}"));
    assert!(result.contains(r"\textasciitilde{}"));
    assert!(result.contains(r"\textbackslash{}"));
}

#[test]
fn test_headings() {
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
                kind: HeadingKind::Setext(SetextHeading::Level1),
                content: vec![Inline::Text("Setext 1".to_string())],
            }),
        ],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\section{Level 1}"));
    assert!(result.contains(r"\subsection{Level 2}"));
    assert!(result.contains(r"\section{Setext 1}"));
}

#[test]
fn test_emphasis() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Normal ".to_string()),
            Inline::Emphasis(vec![Inline::Text("italic".to_string())]),
            Inline::Text(" and ".to_string()),
            Inline::Strong(vec![Inline::Text("bold".to_string())]),
            Inline::Text(" text.".to_string()),
        ])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\textit{italic}"));
    assert!(result.contains(r"\textbf{bold}"));
}

#[test]
fn test_code_block_verbatim() {
    let doc = Document {
        blocks: vec![Block::CodeBlock(CodeBlock {
            kind: CodeBlockKind::Fenced {
                info: Some("rust".to_string()),
            },
            literal: "fn main() {\n    println!(\"Hello!\");\n}".to_string(),
        })],
    };

    let config = Config::default().with_code_block_style(CodeBlockStyle::Verbatim);
    let result = render_latex(&doc, config);
    assert!(result.contains(r"\begin{verbatim}"));
    assert!(result.contains(r"\end{verbatim}"));
    assert!(result.contains("fn main()"));
}

#[test]
fn test_code_block_listings() {
    let doc = Document {
        blocks: vec![Block::CodeBlock(CodeBlock {
            kind: CodeBlockKind::Fenced {
                info: Some("python".to_string()),
            },
            literal: "print('Hello, world!')".to_string(),
        })],
    };

    let config = Config::default().with_code_block_style(CodeBlockStyle::Listings);
    let result = render_latex(&doc, config);
    assert!(result.contains(r"\begin{lstlisting}[language=python]"));
    assert!(result.contains(r"\end{lstlisting}"));
}

#[test]
fn test_code_block_minted() {
    let doc = Document {
        blocks: vec![Block::CodeBlock(CodeBlock {
            kind: CodeBlockKind::Fenced {
                info: Some("javascript".to_string()),
            },
            literal: "console.log('Hello!');".to_string(),
        })],
    };

    let config = Config::default().with_code_block_style(CodeBlockStyle::Minted);
    let result = render_latex(&doc, config);
    assert!(result.contains(r"\begin{minted}{javascript}"));
    assert!(result.contains(r"\end{minted}"));
}

#[test]
fn test_lists() {
    let doc = Document {
        blocks: vec![
            Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Star),
                items: vec![
                    ListItem {
                        task: None,
                        blocks: vec![Block::Paragraph(vec![Inline::Text("Item 1".to_string())])],
                    },
                    ListItem {
                        task: Some(TaskState::Complete),
                        blocks: vec![Block::Paragraph(vec![Inline::Text(
                            "Done item".to_string(),
                        )])],
                    },
                ],
            }),
            Block::List(List {
                kind: ListKind::Ordered(ListOrderedKindOptions { start: 1 }),
                items: vec![ListItem {
                    task: None,
                    blocks: vec![Block::Paragraph(vec![Inline::Text("Numbered".to_string())])],
                }],
            }),
        ],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\begin{itemize}"));
    assert!(result.contains(r"\begin{enumerate}"));
    assert!(result.contains(r"\item Item 1"));
    assert!(result.contains(r"$\boxtimes$"));
}

#[test]
fn test_table_tabular() {
    let doc = Document {
        blocks: vec![Block::Table(Table {
            rows: vec![
                vec![
                    vec![Inline::Text("Header 1".to_string())],
                    vec![Inline::Text("Header 2".to_string())],
                ],
                vec![
                    vec![Inline::Text("Cell 1".to_string())],
                    vec![Inline::Text("Cell 2".to_string())],
                ],
            ],
            alignments: vec![Alignment::Left, Alignment::Right],
        })],
    };

    let config = Config::default().with_table_style(TableStyle::Tabular);
    let result = render_latex(&doc, config);
    assert!(result.contains(r"\begin{tabular}[lr]"));
    assert!(result.contains("Header 1"));
    assert!(result.contains("Cell 1 & Cell 2"));
    assert!(result.contains(r"\\"));
}

#[test]
fn test_blockquote() {
    let doc = Document {
        blocks: vec![Block::BlockQuote(vec![Block::Paragraph(vec![
            Inline::Text("This is a quote.".to_string()),
        ])])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\begin{quote}"));
    assert!(result.contains("This is a quote."));
    assert!(result.contains(r"\end{quote}"));
}

#[test]
fn test_links() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Visit ".to_string()),
            Inline::Link(Link {
                destination: "https://example.com".to_string(),
                title: Some("Example Site".to_string()),
                children: vec![Inline::Text("this link".to_string())],
            }),
            Inline::Text(".".to_string()),
        ])],
    };

    let result = render_latex(&doc, Config::default());
    assert!(result.contains(r"\href"));
    assert!(result.contains("https://example.com"));
    assert!(result.contains("this link"));
    assert!(result.contains(r"\footnote"));
}

#[test]
fn test_config_width() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(
            "This is a very long line that should be wrapped at the specified width limit and it contains many words that exceed forty characters"
                .to_string(),
        )])],
    };

    let config_narrow = Config::default().with_width(40);
    let config_wide = Config::default().with_width(120);

    let result_narrow = render_latex(&doc, config_narrow);
    let result_wide = render_latex(&doc, config_wide);

    println!("Narrow (width=40):\n{}", result_narrow);
    println!("Wide (width=120):\n{}", result_wide);

    // The narrow config should have more line breaks if width works
    // BUT this currently fails because Inline::Text doesn't respect width!
    assert!(
        result_narrow.lines().count() > result_wide.lines().count(),
        "Width control is not working: narrow={} lines, wide={} lines",
        result_narrow.lines().count(),
        result_wide.lines().count()
    );
}

#[test]
fn test_width_works_for_complex_content() {
    // This test verifies that width control works for other elements (not just Text)
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Start ".to_string()),
            Inline::Strong(vec![Inline::Text(
                "bold very long text that should wrap".to_string(),
            )]),
            Inline::Text(" middle ".to_string()),
            Inline::Emphasis(vec![Inline::Text(
                "italic very long text that should also wrap".to_string(),
            )]),
            Inline::Text(" end".to_string()),
        ])],
    };

    let config_narrow = Config::default().with_width(30);
    let config_wide = Config::default().with_width(120);

    let result_narrow = render_latex(&doc, config_narrow);
    let result_wide = render_latex(&doc, config_wide);

    println!("Complex narrow (width=30):\n{}", result_narrow);
    println!("Complex wide (width=120):\n{}", result_wide);

    // This should work because pretty-printer handles complex structures
    // The issue is specifically with Inline::Text nodes
    assert!(result_narrow.lines().count() >= result_wide.lines().count());
}

#[test]
fn test_code_blocks_no_wrapping() {
    // Code blocks should NOT wrap regardless of width setting
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("This is a much longer text before the code block that should definitely wrap on narrow width settings because it contains many words and exceeds thirty characters".to_string()),
            Inline::Code("this is a very long code snippet with spaces that should not wrap regardless of width setting".to_string()),
            Inline::Text(" and this is also a longer text after the code block that should wrap".to_string()),
        ])],
    };

    let config_narrow = Config::default().with_width(20);
    let config_wide = Config::default().with_width(120);

    let result_narrow = render_latex(&doc, config_narrow);
    let result_wide = render_latex(&doc, config_wide);

    println!("Code narrow (width=20):\n{}", result_narrow);
    println!("Code wide (width=120):\n{}", result_wide);

    // Code should remain on same line in both cases (only text should wrap)
    let narrow_lines: Vec<&str> = result_narrow.lines().collect();
    let wide_lines: Vec<&str> = result_wide.lines().collect();

    // Check that the code snippet appears as a single unit in both outputs
    let code_pattern = r"\texttt{this is a very long code snippet with spaces that should not wrap regardless of width setting}";

    // Both outputs should contain the code pattern on a single line
    assert!(
        result_narrow.contains(code_pattern),
        "Code should not be broken in narrow output"
    );
    assert!(
        result_wide.contains(code_pattern),
        "Code should not be broken in wide output"
    );

    // The text should wrap in narrow but not in wide
    assert!(
        narrow_lines.len() > wide_lines.len(),
        "Text should wrap more in narrow width: narrow={} lines, wide={} lines",
        narrow_lines.len(),
        wide_lines.len()
    );
}
