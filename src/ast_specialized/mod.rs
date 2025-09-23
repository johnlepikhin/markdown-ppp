//! Specialized AST types for element ID tracking
//!
//! This module provides pre-defined specialized versions of the generic AST
//! for element identification scenarios.
//!
//! # Quick Start
//!
//! Enable the feature in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! markdown-ppp = { version = "2.6.0", features = ["ast-specialized"] }
//! ```
//!
//! # Example Usage
//!
//! ```rust
//! use markdown_ppp::ast_specialized::{ElementId, with_ids, utilities::id_utils};
//! use markdown_ppp::ast::{Document, Block, Heading, HeadingKind, Inline};
//!
//! // Create a regular document
//! let doc = Document {
//!     blocks: vec![
//!         Block::Heading(Heading {
//!             kind: HeadingKind::Atx(1),
//!             content: vec![Inline::Text("Hello World".to_string())],
//!         })
//!     ],
//! };
//!
//! // Add sequential IDs to all elements
//! let doc_with_ids: with_ids::Document = id_utils::add_ids_to_document(doc);
//!
//! // Access element IDs
//! println!("Document ID: {}", doc_with_ids.user_data.id());
//! ```
//!
//! # Organization
//!
//! - `element_id` - Element ID support and related functionality
//! - `type_aliases` - Convenient type aliases for specialized AST types
//! - `utilities` - Helper functions and utilities

pub mod element_id;
pub mod type_aliases;
pub mod utilities;

// Re-export main types for convenience
pub use element_id::ElementId;

// Re-export type alias modules
pub use type_aliases::with_ids;

// Re-export utility modules
pub use utilities::id_utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_id() {
        let id = ElementId::new(42);
        assert_eq!(id.id(), 42);
        assert_eq!(u64::from(id.clone()), 42);
        assert_eq!(ElementId::from(42), id);
    }
}
