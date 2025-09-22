use crate::ast::*;
use crate::ast_transform::{FilterTransform, Transform, Transformer};

fn create_test_doc() -> Document {
    Document {
        blocks: vec![
            Block::Paragraph(vec![
                Inline::Text("Hello ".to_string()),
                Inline::Text("world!".to_string()),
            ]),
            Block::Paragraph(vec![
                Inline::Image(Image {
                    destination: "/image.jpg".to_string(),
                    title: None,
                    alt: "test".to_string(),
                }),
                Inline::Text(" and ".to_string()),
                Inline::Link(Link {
                    destination: "http://example.com".to_string(),
                    title: None,
                    children: vec![Inline::Text("link".to_string())],
                }),
            ]),
        ],
    }
}

#[test]
fn test_transform_text() {
    let doc = create_test_doc();
    let result = doc.transform_text(|text| text.to_uppercase());

    // Check that text was transformed
    if let Block::Paragraph(inlines) = &result.blocks[0] {
        assert_eq!(inlines[0], Inline::Text("HELLO ".to_string()));
        assert_eq!(inlines[1], Inline::Text("WORLD!".to_string()));
    } else {
        panic!("Expected paragraph");
    }
}

#[test]
fn test_transform_image_urls() {
    let doc = create_test_doc();
    let result = doc.transform_image_urls(|url| format!("https://cdn.example.com{url}"));

    // Check that image URL was transformed
    if let Block::Paragraph(inlines) = &result.blocks[1] {
        if let Inline::Image(image) = &inlines[0] {
            assert_eq!(image.destination, "https://cdn.example.com/image.jpg");
        } else {
            panic!("Expected image");
        }
    } else {
        panic!("Expected paragraph");
    }
}

#[test]
fn test_transform_link_urls() {
    let doc = create_test_doc();
    let result = doc.transform_link_urls(|url| url.replace("http://", "https://"));

    // Check that link URL was transformed
    if let Block::Paragraph(inlines) = &result.blocks[1] {
        if let Inline::Link(link) = &inlines[2] {
            assert_eq!(link.destination, "https://example.com");
        } else {
            panic!("Expected link");
        }
    } else {
        panic!("Expected paragraph");
    }
}

#[test]
fn test_transform_autolink_urls() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Check out ".to_string()),
            Inline::Autolink("http://example.com".to_string()),
            Inline::Text(" and ".to_string()),
            Inline::Autolink("mailto:test@example.com".to_string()),
        ])],
    };

    let result = doc.transform_autolink_urls(|url| {
        if url.starts_with("http://") {
            url.replace("http://", "https://")
        } else if url.starts_with("mailto:") {
            url.replace("mailto:", "email:")
        } else {
            url
        }
    });

    // Check paragraph autolinks
    if let Block::Paragraph(inlines) = &result.blocks[0] {
        if let Inline::Autolink(url) = &inlines[1] {
            assert_eq!(url, "https://example.com");
        }
        if let Inline::Autolink(url) = &inlines[3] {
            assert_eq!(url, "email:test@example.com");
        }
    }
}

#[test]
fn test_transform_code() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Use ".to_string()),
            Inline::Code("println!()".to_string()),
            Inline::Text(" to print.".to_string()),
        ])],
    };

    let result = doc.transform_code(|code| format!("`{code}`"));

    // Check paragraph code spans
    if let Block::Paragraph(inlines) = &result.blocks[0] {
        if let Inline::Code(code) = &inlines[1] {
            assert_eq!(code, "`println!()`");
        }
    }
}

#[test]
fn test_transform_html() {
    let doc = Document {
        blocks: vec![
            Block::Paragraph(vec![
                Inline::Text("Some ".to_string()),
                Inline::Html("<em>inline HTML</em>".to_string()),
                Inline::Text(" here.".to_string()),
            ]),
            Block::HtmlBlock("<div class=\"content\">Block HTML</div>".to_string()),
        ],
    };

    let result = doc.transform_html(|html| format!("<!-- Processed -->{html}"));

    // Check inline HTML in paragraphs
    if let Block::Paragraph(inlines) = &result.blocks[0] {
        if let Inline::Html(html) = &inlines[1] {
            assert_eq!(html, "<!-- Processed --><em>inline HTML</em>");
        }
    }

    // Check HTML block
    if let Block::HtmlBlock(html) = &result.blocks[1] {
        assert_eq!(
            html,
            "<!-- Processed --><div class=\"content\">Block HTML</div>"
        );
    }
}

// Custom transformer for testing transform_with
struct CustomTestTransformer {
    multiplier: usize,
}

impl CustomTestTransformer {
    fn new(multiplier: usize) -> Self {
        Self { multiplier }
    }
}

impl Transformer for CustomTestTransformer {
    fn transform_inline(&mut self, inline: Inline) -> Inline {
        match inline {
            Inline::Text(text) => {
                let repeated = text.repeat(self.multiplier);
                Inline::Text(repeated)
            }
            other => self.walk_transform_inline(other),
        }
    }

    fn transform_block(&mut self, block: Block) -> Block {
        match block {
            Block::CodeBlock(mut code_block) => {
                code_block.literal = format!(
                    "// Multiplied by {}\n{}",
                    self.multiplier, code_block.literal
                );
                Block::CodeBlock(code_block)
            }
            other => self.walk_transform_block(other),
        }
    }
}

#[test]
fn test_transform_with_custom_transformer() {
    let doc = Document {
        blocks: vec![
            Block::Paragraph(vec![
                Inline::Text("Hi".to_string()),
                Inline::Emphasis(vec![Inline::Text(" there".to_string())]),
            ]),
            Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Fenced {
                    info: Some("rust".to_string()),
                },
                literal: "fn main() {}".to_string(),
            }),
        ],
    };

    let transformer = CustomTestTransformer::new(2);
    let result = doc.transform_with(transformer);

    // Check text repetition
    if let Block::Paragraph(inlines) = &result.blocks[0] {
        assert_eq!(inlines[0], Inline::Text("HiHi".to_string()));
        if let Inline::Emphasis(emphasis_content) = &inlines[1] {
            assert_eq!(
                emphasis_content[0],
                Inline::Text(" there there".to_string())
            );
        }
    }

    // Check code block modification
    if let Block::CodeBlock(code_block) = &result.blocks[1] {
        assert_eq!(code_block.literal, "// Multiplied by 2\nfn main() {}");
    }
}

#[test]
fn test_transform_if() {
    let doc = create_test_doc();

    // Should transform when condition is true
    let result1 = doc
        .clone()
        .transform_if(true, |d| d.transform_text(|s| s.to_uppercase()));
    if let Block::Paragraph(inlines) = &result1.blocks[0] {
        assert_eq!(inlines[0], Inline::Text("HELLO ".to_string()));
    }

    // Should not transform when condition is false
    let result2 = doc.transform_if(false, |d| d.transform_text(|s| s.to_uppercase()));
    if let Block::Paragraph(inlines) = &result2.blocks[0] {
        assert_eq!(inlines[0], Inline::Text("Hello ".to_string()));
    }
}

#[test]
fn test_remove_empty_paragraphs() {
    let doc = Document {
        blocks: vec![
            Block::Paragraph(vec![Inline::Text("Not empty".to_string())]),
            Block::Paragraph(vec![]), // Empty paragraph
            Block::Paragraph(vec![Inline::Text("Also not empty".to_string())]),
        ],
    };

    let result = doc.remove_empty_paragraphs();
    assert_eq!(result.blocks.len(), 2);
}

#[test]
fn test_normalize_whitespace() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(
            "  Hello   \n\t  world  ".to_string(),
        )])],
    };

    let result = doc.normalize_whitespace();
    if let Block::Paragraph(inlines) = &result.blocks[0] {
        assert_eq!(inlines[0], Inline::Text("Hello world".to_string()));
    }
}

#[test]
fn test_filter_blocks() {
    let doc = Document {
        blocks: vec![
            Block::Paragraph(vec![Inline::Text("Keep this".to_string())]),
            Block::ThematicBreak,
            Block::Paragraph(vec![Inline::Text("And this".to_string())]),
            Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Indented,
                literal: "Remove this".to_string(),
            }),
        ],
    };

    let result = doc.filter_blocks(|block| !matches!(block, Block::CodeBlock(_)));
    assert_eq!(result.blocks.len(), 3);
}

#[test]
fn test_remove_empty_text() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Valid text".to_string()),
            Inline::Text("   ".to_string()), // Only whitespace
            Inline::Text("Another valid".to_string()),
            Inline::Text("".to_string()), // Empty string
        ])],
    };

    let result = doc.remove_empty_text();

    // Check that empty/whitespace-only text elements were converted to Empty
    if let Block::Paragraph(inlines) = &result.blocks[0] {
        // The EmptyTextRemover converts empty text to Inline::Empty rather than removing it
        assert_eq!(inlines.len(), 4); // All elements still there but some converted to Empty
        assert_eq!(inlines[0], Inline::Text("Valid text".to_string()));
        assert_eq!(inlines[1], Inline::Empty); // Whitespace-only text becomes Empty
        assert_eq!(inlines[2], Inline::Text("Another valid".to_string()));
        assert_eq!(inlines[3], Inline::Empty); // Empty string becomes Empty
    }
}
