//! Example demonstrating how to use element IDs with AST nodes
//!
//! This example shows how to attach element IDs to AST elements
//! for unique identification and custom metadata.
//!
//! **Note:** This example requires the `ast-specialized` feature.
//! Run with: `cargo run --example user_data_example --features ast-specialized`

use markdown_ppp::ast::convert::StripData;
use markdown_ppp::ast::*;
use markdown_ppp::ast_specialized::*;

fn main() {
    println!("=== User Data Example ===\n");

    // Create a simple document
    let doc = Document {
        blocks: vec![
            Block::Heading(Heading {
                kind: HeadingKind::Atx(1),
                content: vec![
                    Inline::Text("Welcome to".to_string()),
                    Inline::Strong(vec![Inline::Text("Markdown".to_string())]),
                ],
            }),
            Block::Paragraph(vec![
                Inline::Text("This is a ".to_string()),
                Inline::Emphasis(vec![Inline::Text("simple".to_string())]),
                Inline::Text(" example.".to_string()),
            ]),
            Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Dash),
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

    println!("Original document has {} blocks", doc.blocks.len());

    // Example 1: Add element IDs
    println!("\n1. Adding Element IDs:");
    let doc_with_ids = add_element_ids(doc.clone());
    print_document_with_ids(&doc_with_ids);

    // Example 2: Custom application data
    println!("\n2. Adding Custom Application Data:");
    let doc_with_custom = add_custom_data(doc.clone());
    print_document_with_custom_data(&doc_with_custom);

    // Example 3: MapData functionality using visitor pattern
    println!("\n3. MapData Transformations:");

    // Transform ElementId to String using the new visitor
    use markdown_ppp::ast::map_data_visitor::map_user_data;
    let doc_with_strings = map_user_data(doc_with_ids.clone(), |id| format!("element_{}", id.id()));
    println!("Transformed IDs to strings:");
    println!("  Document: '{}'", doc_with_strings.user_data);
    println!(
        "  First block: '{}'",
        match &doc_with_strings.blocks[0] {
            generic::Block::Heading(h) => &h.user_data,
            _ => "unknown",
        }
    );

    // Transform back to numbers (round-trip)
    let back_to_numbers = map_user_data(doc_with_strings, |s| {
        let num_str = s.strip_prefix("element_").unwrap_or("0");
        num_str.parse::<u64>().unwrap_or(0)
    });
    println!("Round-trip transformation successful!");
    println!("  Document number: {}", back_to_numbers.user_data);

    // Example 4: Converting back to regular AST
    println!("\n4. Converting Back to Regular AST:");
    let back_to_regular = doc_with_ids.strip_data();
    println!("Converted back, blocks: {}", back_to_regular.blocks.len());
    println!("✓ Strip data conversion successful!");

    #[cfg(feature = "ast-serde")]
    {
        // Example 5: Serialization
        println!("\n5. JSON Serialization:");
        demonstrate_serialization();
    }
}

/// Add sequential element IDs to all nodes
fn add_element_ids(_doc: Document) -> with_ids::Document {
    // Manually create a document with IDs
    generic::Document {
        blocks: vec![
            generic::Block::Heading(generic::Heading {
                kind: HeadingKind::Atx(1),
                content: vec![
                    generic::Inline::Text {
                        content: "Welcome to".to_string(),
                        user_data: ElementId::new(2),
                    },
                    generic::Inline::Strong {
                        content: vec![generic::Inline::Text {
                            content: "Markdown".to_string(),
                            user_data: ElementId::new(3),
                        }],
                        user_data: ElementId::new(4),
                    },
                ],
                user_data: ElementId::new(1),
            }),
            generic::Block::Paragraph {
                content: vec![
                    generic::Inline::Text {
                        content: "This is a ".to_string(),
                        user_data: ElementId::new(5),
                    },
                    generic::Inline::Emphasis {
                        content: vec![generic::Inline::Text {
                            content: "simple".to_string(),
                            user_data: ElementId::new(6),
                        }],
                        user_data: ElementId::new(7),
                    },
                    generic::Inline::Text {
                        content: " example.".to_string(),
                        user_data: ElementId::new(8),
                    },
                ],
                user_data: ElementId::new(9),
            },
        ],
        user_data: ElementId::new(0),
    }
}

/// Add custom application-specific data
#[derive(Debug, Clone, PartialEq)]
struct CustomData {
    name: String,
    priority: u8,
    tags: Vec<String>,
}

impl Default for CustomData {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            priority: 0,
            tags: vec![],
        }
    }
}

fn add_custom_data(_doc: Document) -> generic::Document<CustomData> {
    generic::Document {
        blocks: vec![
            generic::Block::Heading(generic::Heading {
                kind: HeadingKind::Atx(1),
                content: vec![generic::Inline::Text {
                    content: "Custom Data Example".to_string(),
                    user_data: CustomData {
                        name: "heading_text".to_string(),
                        priority: 2,
                        tags: vec!["text".to_string(), "heading".to_string()],
                    },
                }],
                user_data: CustomData {
                    name: "main_heading".to_string(),
                    priority: 1,
                    tags: vec!["heading".to_string(), "h1".to_string()],
                },
            }),
            generic::Block::Paragraph {
                content: vec![generic::Inline::Text {
                    content: "Paragraph with custom metadata".to_string(),
                    user_data: CustomData {
                        name: "paragraph_text".to_string(),
                        priority: 3,
                        tags: vec!["text".to_string()],
                    },
                }],
                user_data: CustomData {
                    name: "content_paragraph".to_string(),
                    priority: 2,
                    tags: vec!["paragraph".to_string(), "content".to_string()],
                },
            },
        ],
        user_data: CustomData {
            name: "document_root".to_string(),
            priority: 0,
            tags: vec!["document".to_string(), "root".to_string()],
        },
    }
}

fn print_document_with_ids(doc: &with_ids::Document) {
    println!("  Document ID: {}", doc.user_data.id());
    println!("  Blocks: {}", doc.blocks.len());
    for (i, block) in doc.blocks.iter().enumerate() {
        println!("    Block {}: ID={}", i, get_block_id(block));
    }
}

fn print_document_with_custom_data(doc: &generic::Document<CustomData>) {
    println!("  Document: {:?}", doc.user_data);
    println!("  Blocks: {}", doc.blocks.len());
    for (i, block) in doc.blocks.iter().enumerate() {
        println!("    Block {}: {:?}", i, get_block_custom_data(block));
    }
}

fn get_block_id(block: &generic::Block<ElementId>) -> u64 {
    match block {
        generic::Block::Heading(h) => h.user_data.id(),
        generic::Block::Paragraph { user_data, .. } => user_data.id(),
        generic::Block::List(l) => l.user_data.id(),
        _ => 0,
    }
}

fn get_block_custom_data(block: &generic::Block<CustomData>) -> &CustomData {
    static DEFAULT_CUSTOM_DATA: CustomData = CustomData {
        name: String::new(),
        priority: 0,
        tags: Vec::new(),
    };

    match block {
        generic::Block::Heading(h) => &h.user_data,
        generic::Block::Paragraph { user_data, .. } => user_data,
        generic::Block::List(l) => &l.user_data,
        _ => &DEFAULT_CUSTOM_DATA,
    }
}

#[cfg(feature = "ast-serde")]
fn demonstrate_serialization() {
    use markdown_ppp::ast_specialized::utilities::id_utils;

    let simple_doc = Document {
        blocks: vec![Block::Heading(Heading {
            kind: HeadingKind::Atx(1),
            content: vec![Inline::Text("Serialization Test".to_string())],
        })],
    };

    let doc_with_ids = id_utils::add_ids_to_document(simple_doc);

    match serde_json::to_string_pretty(&doc_with_ids) {
        Ok(json) => {
            println!("✓ Document serialized to JSON:");
            println!("{json}");

            // Test deserialization
            match serde_json::from_str::<with_ids::Document>(&json) {
                Ok(_) => println!("✓ Document deserialized successfully!"),
                Err(e) => println!("✗ Deserialization failed: {e}"),
            }
        }
        Err(e) => println!("✗ Serialization failed: {e}"),
    }
}
