//! Tests for generic expandable transformations with user data

use crate::ast::generic::*;
use crate::ast_transform::{GenericExpandWith, GenericTransformer};

/// Test user data type for tracking node IDs
#[derive(Debug, Clone, PartialEq, Default)]
struct NodeId(u32);

/// Test user data type for tracking source locations
#[derive(Debug, Clone, PartialEq, Default)]
struct SourceLocation {
    line: u32,
    column: u32,
}

/// Transformer that assigns unique IDs to nodes and expands long paragraphs
struct IdAssignerExpander {
    next_id: u32,
}

impl IdAssignerExpander {
    fn new() -> Self {
        Self { next_id: 1 }
    }

    fn next_id(&mut self) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;
        id
    }
}

impl GenericTransformer<NodeId> for IdAssignerExpander {
    fn walk_expand_block(&mut self, block: Block<NodeId>) -> Vec<Block<NodeId>> {
        match block {
            Block::Paragraph { content, .. } if content.len() > 3 => {
                // Split long paragraphs into two with new IDs
                let mid = content.len() / 2;
                let (first_half, second_half) = content.split_at(mid);

                let mut result = Vec::new();

                // Process first half: assign block ID, then process content
                let first_block_id = self.next_id();
                let transformed_first_half: Vec<_> = first_half
                    .iter()
                    .flat_map(|inline| self.walk_expand_inline(inline.clone()))
                    .collect();
                result.push(Block::Paragraph {
                    content: transformed_first_half,
                    user_data: first_block_id,
                });

                // Process second half: assign block ID, then process content
                let second_block_id = self.next_id();
                let transformed_second_half: Vec<_> = second_half
                    .iter()
                    .flat_map(|inline| self.walk_expand_inline(inline.clone()))
                    .collect();
                result.push(Block::Paragraph {
                    content: transformed_second_half,
                    user_data: second_block_id,
                });

                result
            }
            Block::Paragraph { content, .. } => {
                // Short paragraphs get new ID but no split
                let block_id = self.next_id();
                let transformed_content: Vec<_> = content
                    .into_iter()
                    .flat_map(|inline| self.walk_expand_inline(inline))
                    .collect();
                vec![Block::Paragraph {
                    content: transformed_content,
                    user_data: block_id,
                }]
            }
            other => {
                // Apply default transformation logic
                vec![self.transform_block(other)]
            }
        }
    }

    fn walk_expand_inline(&mut self, inline: Inline<NodeId>) -> Vec<Inline<NodeId>> {
        let transformed_inline = match inline {
            Inline::Text { content, .. } => Inline::Text {
                content,
                user_data: self.next_id(),
            },
            Inline::Emphasis { content, .. } => Inline::Emphasis {
                content: content
                    .into_iter()
                    .flat_map(|inline| self.walk_expand_inline(inline))
                    .collect(),
                user_data: self.next_id(),
            },
            other => other,
        };
        vec![transformed_inline]
    }
}

#[test]
fn test_id_assigner_expander() {
    let doc = Document {
        blocks: vec![
            // Long paragraph (will be split)
            Block::Paragraph {
                content: vec![
                    Inline::Text {
                        content: "One".to_string(),
                        user_data: NodeId(0),
                    },
                    Inline::Text {
                        content: "Two".to_string(),
                        user_data: NodeId(0),
                    },
                    Inline::Text {
                        content: "Three".to_string(),
                        user_data: NodeId(0),
                    },
                    Inline::Text {
                        content: "Four".to_string(),
                        user_data: NodeId(0),
                    },
                ],
                user_data: NodeId(0),
            },
            // Short paragraph (won't be split)
            Block::Paragraph {
                content: vec![Inline::Text {
                    content: "Short".to_string(),
                    user_data: NodeId(0),
                }],
                user_data: NodeId(0),
            },
        ],
        user_data: NodeId(0),
    };

    let mut transformer = IdAssignerExpander::new();
    let result = transformer.walk_expand_document(doc);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].blocks.len(), 3); // Long paragraph split into 2 + short paragraph = 3

    // Check first split paragraph
    if let Block::Paragraph { content, user_data } = &result[0].blocks[0] {
        assert_eq!(content.len(), 2); // First half
        assert_eq!(user_data, &NodeId(1));
        // Check that text nodes got new IDs
        if let Inline::Text { user_data, .. } = &content[0] {
            assert_eq!(user_data, &NodeId(2));
        }
        if let Inline::Text { user_data, .. } = &content[1] {
            assert_eq!(user_data, &NodeId(3));
        }
    } else {
        panic!("Expected first block to be paragraph");
    }

    // Check second split paragraph
    if let Block::Paragraph { content, user_data } = &result[0].blocks[1] {
        assert_eq!(content.len(), 2); // Second half
        assert_eq!(user_data, &NodeId(4));
        // Check that text nodes got new IDs
        if let Inline::Text { user_data, .. } = &content[0] {
            assert_eq!(user_data, &NodeId(5));
        }
        if let Inline::Text { user_data, .. } = &content[1] {
            assert_eq!(user_data, &NodeId(6));
        }
    } else {
        panic!("Expected second block to be paragraph");
    }

    // Check short paragraph (not split)
    if let Block::Paragraph { content, user_data } = &result[0].blocks[2] {
        assert_eq!(content.len(), 1);
        assert_eq!(user_data, &NodeId(7));
        if let Inline::Text { user_data, .. } = &content[0] {
            assert_eq!(user_data, &NodeId(8));
        }
    } else {
        panic!("Expected third block to be paragraph");
    }
}

/// Transformer that works with source location data
struct SourceLocationExpander;

impl GenericTransformer<SourceLocation> for SourceLocationExpander {
    fn walk_expand_inline(
        &mut self,
        inline: Inline<SourceLocation>,
    ) -> Vec<Inline<SourceLocation>> {
        match inline {
            Inline::Text { content, user_data } if content.contains("SPLIT") => {
                // Split text at "SPLIT" with adjusted source locations
                let parts: Vec<&str> = content.split("SPLIT").collect();
                let mut result = Vec::new();
                let mut column_offset = 0;

                for (i, part) in parts.iter().enumerate() {
                    if !part.is_empty() {
                        result.push(Inline::Text {
                            content: part.to_string(),
                            user_data: SourceLocation {
                                line: user_data.line,
                                column: user_data.column + column_offset,
                            },
                        });
                    }
                    column_offset += part.len() as u32 + 5; // +5 for "SPLIT" length

                    // Add separator between parts (except after last)
                    if i < parts.len() - 1 && !parts[i + 1].is_empty() {
                        result.push(Inline::Emphasis {
                            content: vec![Inline::Text {
                                content: " | ".to_string(),
                                user_data: SourceLocation {
                                    line: user_data.line,
                                    column: user_data.column + column_offset - 5,
                                },
                            }],
                            user_data: SourceLocation {
                                line: user_data.line,
                                column: user_data.column + column_offset - 5,
                            },
                        });
                    }
                }

                result
            }
            other => {
                // For other types, use default transformation
                vec![self.transform_inline(other)]
            }
        }
    }
}

#[test]
fn test_source_location_expander() {
    let doc = Document {
        blocks: vec![Block::Paragraph {
            content: vec![Inline::Text {
                content: "HelloSPLITWorld".to_string(),
                user_data: SourceLocation {
                    line: 1,
                    column: 10,
                },
            }],
            user_data: SourceLocation { line: 1, column: 5 },
        }],
        user_data: SourceLocation { line: 1, column: 0 },
    };

    let mut transformer = SourceLocationExpander;
    let result = transformer.walk_expand_document(doc);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].blocks.len(), 1);

    if let Block::Paragraph { content, .. } = &result[0].blocks[0] {
        assert_eq!(content.len(), 3); // "Hello", " | ", "World"

        // Check first text node
        if let Inline::Text { content, user_data } = &content[0] {
            assert_eq!(content, "Hello");
            assert_eq!(user_data.line, 1);
            assert_eq!(user_data.column, 10); // Original column
        } else {
            panic!("Expected first inline to be text");
        }

        // Check separator
        if let Inline::Emphasis {
            content: emphasis_content,
            user_data,
        } = &content[1]
        {
            assert_eq!(user_data.line, 1);
            assert_eq!(user_data.column, 15); // 10 + 5 (Hello) + 5 (SPLIT) - 5 = 15
            if let Inline::Text { content, .. } = &emphasis_content[0] {
                assert_eq!(content, " | ");
            }
        } else {
            panic!("Expected second inline to be emphasis");
        }

        // Check second text node
        if let Inline::Text { content, user_data } = &content[2] {
            assert_eq!(content, "World");
            assert_eq!(user_data.line, 1);
            assert_eq!(user_data.column, 20); // 10 + 5 + 5 = 20
        } else {
            panic!("Expected third inline to be text");
        }
    } else {
        panic!("Expected paragraph");
    }
}

/// Test using GenericExpandWith trait
#[test]
fn test_generic_expand_with_trait() {
    let block = Block::Paragraph {
        content: vec![
            Inline::Text {
                content: "One".to_string(),
                user_data: NodeId(0),
            },
            Inline::Text {
                content: "Two".to_string(),
                user_data: NodeId(0),
            },
            Inline::Text {
                content: "Three".to_string(),
                user_data: NodeId(0),
            },
            Inline::Text {
                content: "Four".to_string(),
                user_data: NodeId(0),
            },
        ],
        user_data: NodeId(0),
    };

    let mut transformer = IdAssignerExpander::new();
    let result = block.expand_with(&mut transformer);

    assert_eq!(result.len(), 2); // Long paragraph should be split

    if let Block::Paragraph { content, user_data } = &result[0] {
        assert_eq!(content.len(), 2);
        assert_eq!(user_data, &NodeId(1));
    } else {
        panic!("Expected first result to be paragraph");
    }

    if let Block::Paragraph { content, user_data } = &result[1] {
        assert_eq!(content.len(), 2);
        assert_eq!(user_data, &NodeId(4));
    } else {
        panic!("Expected second result to be paragraph");
    }
}

/// Test transformer that doesn't expand but modifies user data
struct UserDataModifier;

impl GenericTransformer<NodeId> for UserDataModifier {
    fn walk_expand_block(&mut self, block: Block<NodeId>) -> Vec<Block<NodeId>> {
        let transformed_block = match block {
            Block::Paragraph { content, user_data } => Block::Paragraph {
                content: content
                    .into_iter()
                    .flat_map(|inline| self.walk_expand_inline(inline))
                    .collect(),
                user_data: NodeId(user_data.0 + 100), // Add 100 to ID
            },
            other => self.walk_transform_block(other),
        };
        vec![transformed_block]
    }

    fn walk_expand_inline(&mut self, inline: Inline<NodeId>) -> Vec<Inline<NodeId>> {
        let transformed_inline = match inline {
            Inline::Text { content, user_data } => Inline::Text {
                content,
                user_data: NodeId(user_data.0 + 100), // Add 100 to ID
            },
            other => self.walk_transform_inline(other),
        };
        vec![transformed_inline]
    }
}

#[test]
fn test_user_data_modification() {
    let doc = Document {
        blocks: vec![Block::Paragraph {
            content: vec![Inline::Text {
                content: "Test".to_string(),
                user_data: NodeId(5),
            }],
            user_data: NodeId(10),
        }],
        user_data: NodeId(1),
    };

    let mut transformer = UserDataModifier;
    let result = transformer.walk_expand_document(doc);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].blocks.len(), 1);

    // Document user data should be unchanged (we don't transform it)
    assert_eq!(result[0].user_data, NodeId(1));

    if let Block::Paragraph { content, user_data } = &result[0].blocks[0] {
        assert_eq!(user_data, &NodeId(110)); // 10 + 100

        if let Inline::Text { user_data, .. } = &content[0] {
            assert_eq!(user_data, &NodeId(105)); // 5 + 100
        } else {
            panic!("Expected text inline");
        }
    } else {
        panic!("Expected paragraph");
    }
}
