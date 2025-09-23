//! Specialized AST types for element ID tracking
//!
//! This module provides pre-defined specialized versions of the generic AST
//! for element identification scenarios.
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
