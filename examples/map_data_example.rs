//! Example demonstrating the visitor-based MapData functionality
//!
//! This example shows how to transform user data in AST nodes using the
//! new visitor-based approach that doesn't hit compiler recursion limits.
//!
//! **Note:** This example requires the `ast-specialized` feature.
//! Run with: `cargo run --example map_data_example --features ast-specialized`

use markdown_ppp::ast::generic;
use markdown_ppp::ast::map_data_visitor::{map_user_data, MapDataVisitor};
use markdown_ppp::ast::*;
use markdown_ppp::ast_specialized::element_id::IdGenerator;
use markdown_ppp::ast_specialized::*;

fn main() {
    println!("=== MapData Visitor Example ===\n");

    // Create a complex document with ElementId user data
    let doc_with_ids = create_document_with_ids();
    println!("Original document with IDs:");
    print_document_summary(&doc_with_ids);

    // Example 1: Transform ElementId to String
    println!("\n1. Transform ElementId to String:");
    let doc_with_strings = map_user_data(doc_with_ids.clone(), |id| format!("element_{}", id.id()));
    print_string_document_summary(&doc_with_strings);

    // Example 2: Transform ElementId to Priority numbers
    println!("\n2. Transform ElementId to Priority (multiply by 10):");
    let doc_with_priorities = map_user_data(doc_with_ids.clone(), |id| id.id() * 10);
    print_priority_document_summary(&doc_with_priorities);

    // Example 3: Using utility functions
    println!("\n3. Using utility functions to assign fresh IDs:");
    let original_doc = Document {
        blocks: vec![
            Block::Heading(Heading {
                kind: HeadingKind::Atx(1),
                content: vec![
                    Inline::Text("Complex".to_string()),
                    Inline::Strong(vec![Inline::Text("Heading".to_string())]),
                ],
            }),
            Block::Paragraph(vec![
                Inline::Text("With ".to_string()),
                Inline::Emphasis(vec![Inline::Text("nested".to_string())]),
                Inline::Text(" content.".to_string()),
            ]),
        ],
    };

    let doc_with_new_ids = id_utils::add_ids_to_document(original_doc);
    println!("Document with newly assigned IDs:");
    print_document_summary(&doc_with_new_ids);

    // Example 4: Custom visitor for advanced transformations
    println!("\n4. Custom visitor - convert to metadata:");
    let doc_with_metadata = transform_to_metadata(doc_with_ids);
    print_metadata_document_summary(&doc_with_metadata);

    // Example 5: Round-trip transformation
    println!("\n5. Round-trip: ID -> String -> ID");
    let back_to_ids = map_user_data(doc_with_strings, |s| {
        // Extract number from "element_X" format
        let num_str = s.strip_prefix("element_").unwrap_or("0");
        let id_num = num_str.parse::<u64>().unwrap_or(0);
        ElementId::new(id_num)
    });
    print_document_summary(&back_to_ids);

    println!("\n✓ All MapData transformations completed successfully!");
    println!("✓ No compiler recursion limits encountered!");
}

fn create_document_with_ids() -> with_ids::Document {
    let mut id_gen = IdGenerator::new();

    generic::Document {
        blocks: vec![
            generic::Block::Heading(generic::Heading {
                kind: HeadingKind::Atx(1),
                content: vec![
                    generic::Inline::Text {
                        content: "Example".to_string(),
                        user_data: id_gen.generate(),
                    },
                    generic::Inline::Strong {
                        content: vec![generic::Inline::Text {
                            content: "Document".to_string(),
                            user_data: id_gen.generate(),
                        }],
                        user_data: id_gen.generate(),
                    },
                ],
                user_data: id_gen.generate(),
            }),
            generic::Block::Paragraph {
                content: vec![
                    generic::Inline::Text {
                        content: "This is a ".to_string(),
                        user_data: id_gen.generate(),
                    },
                    generic::Inline::Emphasis {
                        content: vec![generic::Inline::Text {
                            content: "complex".to_string(),
                            user_data: id_gen.generate(),
                        }],
                        user_data: id_gen.generate(),
                    },
                    generic::Inline::Text {
                        content: " example.".to_string(),
                        user_data: id_gen.generate(),
                    },
                ],
                user_data: id_gen.generate(),
            },
            generic::Block::List(generic::List {
                kind: generic::ListKind::Bullet(ListBulletKind::Dash),
                items: vec![
                    generic::ListItem {
                        task: None,
                        blocks: vec![generic::Block::Paragraph {
                            content: vec![generic::Inline::Text {
                                content: "List item 1".to_string(),
                                user_data: id_gen.generate(),
                            }],
                            user_data: id_gen.generate(),
                        }],
                        user_data: id_gen.generate(),
                    },
                    generic::ListItem {
                        task: Some(TaskState::Complete),
                        blocks: vec![generic::Block::Paragraph {
                            content: vec![generic::Inline::Text {
                                content: "Completed task".to_string(),
                                user_data: id_gen.generate(),
                            }],
                            user_data: id_gen.generate(),
                        }],
                        user_data: id_gen.generate(),
                    },
                ],
                user_data: id_gen.generate(),
            }),
        ],
        user_data: id_gen.generate(),
    }
}

// Custom metadata type
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ElementMetadata {
    original_id: u64,
    element_type: String,
    depth: u32,
}

struct MetadataVisitor {
    depth: u32,
}

impl MetadataVisitor {
    fn new() -> Self {
        Self { depth: 0 }
    }
}

impl MapDataVisitor<ElementId, ElementMetadata> for MetadataVisitor {
    fn map_data(&mut self, data: ElementId) -> ElementMetadata {
        ElementMetadata {
            original_id: data.id(),
            element_type: "unknown".to_string(),
            depth: self.depth,
        }
    }

    fn visit_block(&mut self, block: generic::Block<ElementId>) -> generic::Block<ElementMetadata> {
        self.depth += 1;
        let result = match block {
            generic::Block::Paragraph { content, user_data } => generic::Block::Paragraph {
                content: content.into_iter().map(|i| self.visit_inline(i)).collect(),
                user_data: ElementMetadata {
                    original_id: user_data.id(),
                    element_type: "paragraph".to_string(),
                    depth: self.depth,
                },
            },
            generic::Block::Heading(heading) => {
                let mut heading_result = self.visit_heading(heading);
                heading_result.user_data.element_type = "heading".to_string();
                generic::Block::Heading(heading_result)
            }
            generic::Block::List(list) => {
                let mut list_result = self.visit_list(list);
                list_result.user_data.element_type = "list".to_string();
                generic::Block::List(list_result)
            }
            _ => {
                // For other block types, use default transformation
                let mut temp_visitor = MetadataVisitor { depth: self.depth };
                let result = temp_visitor.visit_block(block);
                self.depth = temp_visitor.depth;
                result
            }
        };
        self.depth -= 1;
        result
    }

    fn visit_inline(
        &mut self,
        inline: generic::Inline<ElementId>,
    ) -> generic::Inline<ElementMetadata> {
        match inline {
            generic::Inline::Text { content, user_data } => generic::Inline::Text {
                content,
                user_data: ElementMetadata {
                    original_id: user_data.id(),
                    element_type: "text".to_string(),
                    depth: self.depth,
                },
            },
            generic::Inline::Strong { content, user_data } => generic::Inline::Strong {
                content: content.into_iter().map(|i| self.visit_inline(i)).collect(),
                user_data: ElementMetadata {
                    original_id: user_data.id(),
                    element_type: "strong".to_string(),
                    depth: self.depth,
                },
            },
            generic::Inline::Emphasis { content, user_data } => generic::Inline::Emphasis {
                content: content.into_iter().map(|i| self.visit_inline(i)).collect(),
                user_data: ElementMetadata {
                    original_id: user_data.id(),
                    element_type: "emphasis".to_string(),
                    depth: self.depth,
                },
            },
            _ => {
                // For other inline types, use default transformation
                let mut temp_visitor = MetadataVisitor { depth: self.depth };
                temp_visitor.visit_inline(inline)
            }
        }
    }
}

fn transform_to_metadata(doc: with_ids::Document) -> generic::Document<ElementMetadata> {
    let mut visitor = MetadataVisitor::new();
    visitor.visit_document(doc)
}

// Helper functions for printing
fn print_document_summary(doc: &with_ids::Document) {
    println!("  Document ID: {}", doc.user_data.id());
    println!("  Blocks: {}", doc.blocks.len());
    for (i, block) in doc.blocks.iter().enumerate() {
        match block {
            generic::Block::Heading(h) => {
                println!("    Block {}: Heading (ID: {})", i, h.user_data.id())
            }
            generic::Block::Paragraph { user_data, .. } => {
                println!("    Block {}: Paragraph (ID: {})", i, user_data.id())
            }
            generic::Block::List(l) => println!(
                "    Block {}: List (ID: {}, {} items)",
                i,
                l.user_data.id(),
                l.items.len()
            ),
            _ => println!("    Block {}: Other", i),
        }
    }
}

fn print_string_document_summary(doc: &generic::Document<String>) {
    println!("  Document: '{}'", doc.user_data);
    println!("  Blocks: {}", doc.blocks.len());
    for (i, block) in doc.blocks.iter().enumerate() {
        match block {
            generic::Block::Heading(h) => println!("    Block {}: Heading ('{}')", i, h.user_data),
            generic::Block::Paragraph { user_data, .. } => {
                println!("    Block {}: Paragraph ('{}')", i, user_data)
            }
            generic::Block::List(l) => println!(
                "    Block {}: List ('{}', {} items)",
                i,
                l.user_data,
                l.items.len()
            ),
            _ => println!("    Block {}: Other", i),
        }
    }
}

fn print_priority_document_summary(doc: &generic::Document<u64>) {
    println!("  Document Priority: {}", doc.user_data);
    println!("  Blocks: {}", doc.blocks.len());
    for (i, block) in doc.blocks.iter().enumerate() {
        match block {
            generic::Block::Heading(h) => {
                println!("    Block {}: Heading (Priority: {})", i, h.user_data)
            }
            generic::Block::Paragraph { user_data, .. } => {
                println!("    Block {}: Paragraph (Priority: {})", i, user_data)
            }
            generic::Block::List(l) => println!(
                "    Block {}: List (Priority: {}, {} items)",
                i,
                l.user_data,
                l.items.len()
            ),
            _ => println!("    Block {}: Other", i),
        }
    }
}

fn print_metadata_document_summary(doc: &generic::Document<ElementMetadata>) {
    println!("  Document: {:?}", doc.user_data);
    println!("  Blocks: {}", doc.blocks.len());
    for (i, block) in doc.blocks.iter().enumerate() {
        match block {
            generic::Block::Heading(h) => println!("    Block {}: Heading ({:?})", i, h.user_data),
            generic::Block::Paragraph { user_data, .. } => {
                println!("    Block {}: Paragraph ({:?})", i, user_data)
            }
            generic::Block::List(l) => println!(
                "    Block {}: List ({:?}, {} items)",
                i,
                l.user_data,
                l.items.len()
            ),
            _ => println!("    Block {}: Other", i),
        }
    }
}
