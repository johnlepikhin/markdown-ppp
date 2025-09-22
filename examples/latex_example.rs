use markdown_ppp::ast::*;
use markdown_ppp::latex_printer::{config::*, render_latex};

fn main() {
    // Create a simple document
    let doc = Document {
        blocks: vec![
            Block::Heading(Heading {
                kind: HeadingKind::Atx(1),
                content: vec![Inline::Text("Sample Document".to_string())],
            }),
            Block::Paragraph(vec![
                Inline::Text("This is a ".to_string()),
                Inline::Strong(vec![Inline::Text("bold".to_string())]),
                Inline::Text(" and ".to_string()),
                Inline::Emphasis(vec![Inline::Text("italic".to_string())]),
                Inline::Text(" text with special characters: $ & % # ^ _ { } ~ \\".to_string()),
            ]),
            Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Fenced {
                    info: Some("rust".to_string()),
                },
                literal: "fn main() {\n    println!(\"Hello, LaTeX!\");\n}".to_string(),
            }),
            Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Star),
                items: vec![
                    ListItem {
                        task: None,
                        blocks: vec![Block::Paragraph(vec![Inline::Text(
                            "First item".to_string(),
                        )])],
                    },
                    ListItem {
                        task: Some(TaskState::Complete),
                        blocks: vec![Block::Paragraph(vec![Inline::Text(
                            "Completed task".to_string(),
                        )])],
                    },
                ],
            }),
        ],
    };

    // Render with different configurations
    println!("=== Verbatim Code Blocks ===");
    let config1 = Config::default().with_code_block_style(CodeBlockStyle::Verbatim);
    println!("{}", render_latex(&doc, config1));

    println!("\n=== Listings Code Blocks ===");
    let config2 = Config::default().with_code_block_style(CodeBlockStyle::Listings);
    println!("{}", render_latex(&doc, config2));

    println!("\n=== Minted Code Blocks ===");
    let config3 = Config::default().with_code_block_style(CodeBlockStyle::Minted);
    println!("{}", render_latex(&doc, config3));
}
