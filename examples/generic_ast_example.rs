//! Example demonstrating basic generic AST functionality
//!
//! This example shows how to use the generic AST types with custom user data
//! without requiring the `ast-specialized` feature.

use markdown_ppp::ast::convert::{StripData, WithData};
use markdown_ppp::ast::generic;
use markdown_ppp::ast::map_data_visitor::map_user_data;
use markdown_ppp::ast::*;

fn main() {
    println!("=== Generic AST Example ===\n");

    // Create a regular document
    let doc = Document {
        blocks: vec![
            Block::Heading(Heading {
                kind: HeadingKind::Atx(1),
                content: vec![
                    Inline::Text("Welcome to".to_string()),
                    Inline::Strong(vec![Inline::Text("Generic AST".to_string())]),
                ],
            }),
            Block::Paragraph(vec![
                Inline::Text("This example shows basic ".to_string()),
                Inline::Emphasis(vec![Inline::Text("generic".to_string())]),
                Inline::Text(" functionality.".to_string()),
            ]),
        ],
    };

    println!("Original document has {} blocks", doc.blocks.len());

    // Example 1: Add unit type data (no-op, but demonstrates the pattern)
    println!("\n1. Adding Unit Type Data:");
    let doc_with_unit: generic::Document<()> = doc.clone().with_default_data();
    println!(
        "Document with unit data: {} blocks",
        doc_with_unit.blocks.len()
    );

    // Example 2: Add string metadata
    println!("\n2. Adding String Metadata:");
    let doc_with_strings: generic::Document<String> =
        doc.clone().with_data("main_document".to_string());
    println!("Document metadata: '{}'", doc_with_strings.user_data);

    // Example 3: Transform metadata using visitor
    println!("\n3. Transform String to Numbers:");
    let doc_with_numbers = map_user_data(doc_with_strings, |s| s.len() as u32);
    println!("Document metadata length: {}", doc_with_numbers.user_data);

    // Example 4: Custom metadata struct
    println!("\n4. Custom Metadata:");

    let doc_with_custom = create_document_with_custom_metadata();
    println!("Custom metadata: {:?}", doc_with_custom.user_data);

    // Example 5: Strip data back to regular AST
    println!("\n5. Converting Back to Regular AST:");
    let back_to_regular = doc_with_numbers.strip_data();
    println!("Converted back, blocks: {}", back_to_regular.blocks.len());

    // Example 6: Chain transformations
    println!("\n6. Chain Transformations:");
    let result = doc
        .with_data(42u32) // Document -> generic::Document<u32>
        .map_data(|n| format!("value_{}", n)) // -> generic::Document<String>
        .map_data(|s| s.chars().count()) // -> generic::Document<usize>
        .map_data(|n| n as f64 * 3.14); // -> generic::Document<f64>

    println!("Final transformed value: {}", result.user_data);

    println!("\n✓ Generic AST functionality works without specialized types!");
}

#[derive(Debug, Clone)]
struct CustomMeta {
    name: String,
    priority: u8,
    created_at: u64,
}

fn create_document_with_custom_metadata() -> generic::Document<CustomMeta> {
    generic::Document {
        blocks: vec![generic::Block::Paragraph {
            content: vec![generic::Inline::Text {
                content: "Example with custom metadata".to_string(),
                user_data: CustomMeta {
                    name: "text_element".to_string(),
                    priority: 5,
                    created_at: 1234567891,
                },
            }],
            user_data: CustomMeta {
                name: "paragraph_element".to_string(),
                priority: 7,
                created_at: 1234567892,
            },
        }],
        user_data: CustomMeta {
            name: "main_document".to_string(),
            priority: 10,
            created_at: 1234567890,
        },
    }
}

// Helper trait to chain map_data operations
trait MapDataChain<T>: Sized {
    fn map_data<U, F>(self, f: F) -> generic::Document<U>
    where
        F: FnMut(T) -> U;
}

impl<T> MapDataChain<T> for generic::Document<T> {
    fn map_data<U, F>(self, f: F) -> generic::Document<U>
    where
        F: FnMut(T) -> U,
    {
        map_user_data(self, f)
    }
}
