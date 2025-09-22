use crate::ast::*;
use crate::ast_transform::{FilterTransform, Transform, Transformer};

// Basic test with minimal AST structure to verify compilation
fn create_simple_doc() -> Document {
    Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(
            "Hello world".to_string(),
        )])],
    }
}

#[test]
fn test_basic_text_transform() {
    let doc = create_simple_doc();
    let result = doc.transform_text(|text| text.to_uppercase());

    if let Block::Paragraph(inlines) = &result.blocks[0] {
        assert_eq!(inlines[0], Inline::Text("HELLO WORLD".to_string()));
    } else {
        panic!("Expected paragraph");
    }
}

#[test]
fn test_basic_normalize_whitespace() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(
            "  hello  world  ".to_string(),
        )])],
    };

    let result = doc.normalize_whitespace();

    if let Block::Paragraph(inlines) = &result.blocks[0] {
        assert_eq!(inlines[0], Inline::Text("hello world".to_string()));
    }
}

#[test]
fn test_basic_remove_empty_paragraphs() {
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
fn test_basic_filter_blocks() {
    let doc = Document {
        blocks: vec![
            Block::Paragraph(vec![Inline::Text("Keep this".to_string())]),
            Block::ThematicBreak,
            Block::Paragraph(vec![Inline::Text("And this".to_string())]),
        ],
    };

    let result = doc.filter_blocks(|block| !matches!(block, Block::ThematicBreak));
    assert_eq!(result.blocks.len(), 2);
}

// Simple custom transformer for testing
struct SimpleTransformer;

impl Transformer for SimpleTransformer {
    fn transform_inline(&mut self, inline: Inline) -> Inline {
        match inline {
            Inline::Text(text) => Inline::Text(format!(">> {text}")),
            other => self.walk_transform_inline(other),
        }
    }
}

#[test]
fn test_basic_transform_with() {
    let doc = create_simple_doc();
    let result = doc.transform_with(SimpleTransformer);

    if let Block::Paragraph(inlines) = &result.blocks[0] {
        assert_eq!(inlines[0], Inline::Text(">> Hello world".to_string()));
    }
}
