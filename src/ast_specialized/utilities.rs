//! Utility functions for working with specialized AST types
//!
//! This module provides helper functions and utilities for common operations
//! with specialized AST types.

use super::element_id::IdGenerator;
use super::type_aliases::with_ids;
use super::ElementId;

/// Utility functions for adding IDs to AST nodes
pub mod id_utils {
    use super::*;
    use crate::ast::convert::WithData;
    use crate::ast::generic;
    use crate::ast::map_data_visitor::{map_user_data, MapDataVisitor};

    /// Add sequential IDs to all nodes in a document
    pub fn add_ids_to_document(doc: crate::ast::Document) -> with_ids::Document {
        let doc_with_unit: generic::Document<()> = doc.with_default_data();
        let mut id_gen = IdGenerator::new();
        map_user_data(doc_with_unit, |_| id_gen.generate())
    }

    /// Add sequential IDs starting from a specific value
    pub fn add_ids_from(doc: crate::ast::Document, start_id: u64) -> with_ids::Document {
        let doc_with_unit: generic::Document<()> = doc.with_default_data();
        let mut id_gen = IdGenerator::starting_from(start_id);
        map_user_data(doc_with_unit, |_| id_gen.generate())
    }

    /// Custom visitor for adding IDs with more control
    pub struct IdAssignmentVisitor {
        id_gen: IdGenerator,
    }

    impl IdAssignmentVisitor {
        pub fn new() -> Self {
            Self {
                id_gen: IdGenerator::new(),
            }
        }

        pub fn starting_from(start_id: u64) -> Self {
            Self {
                id_gen: IdGenerator::starting_from(start_id),
            }
        }
    }

    impl Default for IdAssignmentVisitor {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> MapDataVisitor<T, ElementId> for IdAssignmentVisitor {
        fn map_data(&mut self, _data: T) -> ElementId {
            self.id_gen.generate()
        }
    }

    /// Add IDs to any generic document
    pub fn add_ids_to_generic_document<T>(doc: generic::Document<T>) -> with_ids::Document {
        let mut visitor = IdAssignmentVisitor::new();
        visitor.visit_document(doc)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::ast::{Block, Heading, HeadingKind, Inline};

        #[test]
        fn test_add_ids_to_document() {
            let doc = crate::ast::Document {
                blocks: vec![Block::Heading(Heading {
                    kind: HeadingKind::Atx(1),
                    content: vec![Inline::Text("Test".to_string())],
                })],
            };

            let doc_with_ids = add_ids_to_document(doc);

            // Should have ID assigned to document
            assert!(doc_with_ids.user_data.id() > 0);

            // Should have IDs assigned to all blocks
            assert_eq!(doc_with_ids.blocks.len(), 1);
            match &doc_with_ids.blocks[0] {
                generic::Block::Heading(h) => {
                    assert!(h.user_data.id() > 0);
                    assert_eq!(h.content.len(), 1);
                    match &h.content[0] {
                        generic::Inline::Text { user_data, .. } => {
                            assert!(user_data.id() > 0);
                        }
                        _ => panic!("Expected text inline"),
                    }
                }
                _ => panic!("Expected heading"),
            }
        }

        #[test]
        fn test_add_ids_from() {
            let doc = crate::ast::Document {
                blocks: vec![Block::Heading(Heading {
                    kind: HeadingKind::Atx(1),
                    content: vec![Inline::Text("Test".to_string())],
                })],
            };

            let doc_with_ids = add_ids_from(doc, 100);

            // Should start from the specified ID
            assert!(doc_with_ids.user_data.id() >= 100);
        }

        #[test]
        fn test_id_assignment_visitor() {
            let mut visitor = IdAssignmentVisitor::starting_from(50);
            let id1 = visitor.map_data(());
            let id2 = visitor.map_data(());

            assert_eq!(id1.id(), 50);
            assert_eq!(id2.id(), 51);
        }
    }
}
