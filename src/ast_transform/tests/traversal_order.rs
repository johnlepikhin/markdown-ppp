#[cfg(test)]
mod traversal_order_tests {
    use crate::ast::*;
    use crate::ast_transform::{TransformWith, Transformer, VisitWith, Visitor};

    // Test case for reproducing traversal order issue
    #[derive(Debug)]
    struct OrderTracker {
        order: Vec<String>,
    }

    impl OrderTracker {
        fn new() -> Self {
            Self { order: Vec::new() }
        }
    }

    // Visitor implementation that tracks visit order
    impl Visitor for OrderTracker {
        fn visit_inline(&mut self, inline: &Inline) {
            match inline {
                Inline::Text(text) => {
                    self.order.push(format!("Text({text})"));
                }
                Inline::Strong(_) => {
                    self.order.push("Strong".to_string());
                }
                Inline::Emphasis(_) => {
                    self.order.push("Emphasis".to_string());
                }
                Inline::Link(_) => {
                    self.order.push("Link".to_string());
                }
                Inline::LinkReference(_) => {
                    self.order.push("LinkReference".to_string());
                }
                _ => {}
            }
            self.walk_inline(inline);
        }

        fn visit_block(&mut self, block: &Block) {
            match block {
                Block::Paragraph(_) => {
                    self.order.push("Paragraph".to_string());
                }
                Block::Heading(_) => {
                    self.order.push("Heading".to_string());
                }
                Block::List(_) => {
                    self.order.push("List".to_string());
                }
                _ => {}
            }
            self.walk_block(block);
        }
    }

    // Transformer implementation that tracks transform order
    struct OrderTrackingTransformer {
        order: Vec<String>,
    }

    impl OrderTrackingTransformer {
        fn new() -> Self {
            Self { order: Vec::new() }
        }
    }

    impl Transformer for OrderTrackingTransformer {
        fn transform_inline(&mut self, inline: Inline) -> Inline {
            match &inline {
                Inline::Text(text) => {
                    self.order.push(format!("Text({text})"));
                }
                Inline::Strong(_) => {
                    self.order.push("Strong".to_string());
                }
                Inline::Emphasis(_) => {
                    self.order.push("Emphasis".to_string());
                }
                Inline::Link(_) => {
                    self.order.push("Link".to_string());
                }
                Inline::LinkReference(_) => {
                    self.order.push("LinkReference".to_string());
                }
                _ => {}
            }
            self.walk_transform_inline(inline)
        }

        fn transform_block(&mut self, block: Block) -> Block {
            match &block {
                Block::Paragraph(_) => {
                    self.order.push("Paragraph".to_string());
                }
                Block::Heading(_) => {
                    self.order.push("Heading".to_string());
                }
                Block::List(_) => {
                    self.order.push("List".to_string());
                }
                _ => {}
            }
            self.walk_transform_block(block)
        }
    }

    #[test]
    fn test_traversal_order_consistency() {
        // Create a complex AST with nested structures
        let doc = Document {
            blocks: vec![
                Block::Heading(Heading {
                    kind: HeadingKind::Atx(1),
                    content: vec![
                        Inline::Text("Title".to_string()),
                        Inline::Strong(vec![Inline::Text("Bold".to_string())]),
                    ],
                }),
                Block::Paragraph(vec![
                    Inline::Text("Text1".to_string()),
                    Inline::Emphasis(vec![
                        Inline::Text("Italic".to_string()),
                        Inline::Strong(vec![Inline::Text("BoldItalic".to_string())]),
                    ]),
                    Inline::Text("Text2".to_string()),
                    Inline::Link(Link {
                        destination: "https://example.com".to_string(),
                        title: None,
                        children: vec![Inline::Text("Link".to_string())],
                    }),
                    Inline::Text("Text3".to_string()),
                ]),
                Block::List(List {
                    kind: ListKind::Bullet(ListBulletKind::Dash),
                    items: vec![
                        ListItem {
                            task: None,
                            blocks: vec![Block::Paragraph(vec![
                                Inline::Text("Item1".to_string()),
                                Inline::Strong(vec![Inline::Text("Strong1".to_string())]),
                            ])],
                        },
                        ListItem {
                            task: None,
                            blocks: vec![Block::Paragraph(vec![
                                Inline::Text("Item2".to_string()),
                                Inline::Emphasis(vec![Inline::Text("Emph1".to_string())]),
                            ])],
                        },
                    ],
                }),
            ],
        };

        // Test visitor traversal order
        let mut visitor = OrderTracker::new();
        doc.visit_with(&mut visitor);
        let visitor_order = visitor.order;

        // Test transformer traversal order
        let mut transformer = OrderTrackingTransformer::new();
        let _transformed_doc = doc.clone().transform_with(&mut transformer);
        let transformer_order = transformer.order;

        // Print both orders for comparison
        println!("Visitor order:");
        for (i, item) in visitor_order.iter().enumerate() {
            println!("  {i}: {item}");
        }

        println!("\nTransformer order:");
        for (i, item) in transformer_order.iter().enumerate() {
            println!("  {i}: {item}");
        }

        // Compare orders
        if visitor_order != transformer_order {
            println!("\n❌ TRAVERSAL ORDER MISMATCH DETECTED!");
            println!(
                "Visitor length: {}, Transformer length: {}",
                visitor_order.len(),
                transformer_order.len()
            );

            // Find first difference
            for (i, (v, t)) in visitor_order
                .iter()
                .zip(transformer_order.iter())
                .enumerate()
            {
                if v != t {
                    println!("First difference at index {i}: '{v}' vs '{t}'");
                    break;
                }
            }
        } else {
            println!("\n✅ Traversal orders are identical");
        }

        // This assertion will fail if orders are different, proving the bug exists
        assert_eq!(
            visitor_order, transformer_order,
            "Traversal orders should be identical"
        );
    }

    #[test]
    fn test_complex_traversal_with_reference_links() {
        // Test with LinkReference which was mentioned in the bug report
        let doc = Document {
            blocks: vec![
                Block::Paragraph(vec![
                    Inline::Text("See ".to_string()),
                    Inline::LinkReference(LinkReference {
                        label: vec![Inline::Text("ref1".to_string())],
                        text: vec![
                            Inline::Text("link ".to_string()),
                            Inline::Strong(vec![Inline::Text("text".to_string())]),
                        ],
                    }),
                    Inline::Text(" and ".to_string()),
                    Inline::LinkReference(LinkReference {
                        label: vec![
                            Inline::Text("ref2".to_string()),
                            Inline::Emphasis(vec![Inline::Text("label".to_string())]),
                        ],
                        text: vec![Inline::Text("more text".to_string())],
                    }),
                ]),
                Block::Definition(LinkDefinition {
                    label: vec![Inline::Text("ref1".to_string())],
                    destination: "https://example.com".to_string(),
                    title: Some("Title".to_string()),
                }),
            ],
        };

        // Test visitor traversal order
        let mut visitor = OrderTracker::new();
        doc.visit_with(&mut visitor);
        let visitor_order = visitor.order;

        // Test transformer traversal order
        let mut transformer = OrderTrackingTransformer::new();
        let _transformed_doc = doc.clone().transform_with(&mut transformer);
        let transformer_order = transformer.order;

        println!("Complex test - Visitor order: {visitor_order:?}");
        println!("Complex test - Transformer order: {transformer_order:?}");

        assert_eq!(
            visitor_order, transformer_order,
            "Complex traversal orders should be identical"
        );
    }
}
