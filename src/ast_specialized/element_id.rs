//! Element ID support for AST nodes
//!
//! This module provides [`ElementId`] type for uniquely identifying AST elements
//! and related functionality like ID generation.
//!
//! # Example
//!
//! ```rust
//! use markdown_ppp::ast_specialized::element_id::{ElementId, IdGenerator};
//!
//! // Create element IDs
//! let id1 = ElementId::new(42);
//! let id2 = ElementId::from(100);
//!
//! // Generate sequential IDs
//! let mut generator = IdGenerator::new();
//! let id_a = generator.generate(); // ElementId(1)
//! let id_b = generator.generate(); // ElementId(2)
//!
//! // Start from specific value
//! let mut custom_gen = IdGenerator::starting_from(1000);
//! let id_custom = custom_gen.generate(); // ElementId(1000)
//! ```

/// Unique identifier for an AST element
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ElementId(pub u64);

impl ElementId {
    /// Create a new element ID
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the raw ID value
    pub fn id(&self) -> u64 {
        self.0
    }
}

impl From<u64> for ElementId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl From<ElementId> for u64 {
    fn from(element_id: ElementId) -> Self {
        element_id.0
    }
}

/// ID generator for creating unique element IDs
#[derive(Debug, Clone)]
pub struct IdGenerator {
    next_id: u64,
}

impl IdGenerator {
    /// Create a new ID generator starting from 1
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    /// Create a new ID generator starting from a specific value
    pub fn starting_from(start_id: u64) -> Self {
        Self { next_id: start_id }
    }

    /// Generate the next unique ID
    pub fn generate(&mut self) -> ElementId {
        let id = ElementId::new(self.next_id);
        self.next_id += 1;
        id
    }

    /// Peek at the next ID without consuming it
    pub fn peek(&self) -> ElementId {
        ElementId::new(self.next_id)
    }

    /// Reset the generator to start from 1 again
    pub fn reset(&mut self) {
        self.next_id = 1;
    }
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_id_basic() {
        let id = ElementId::new(42);
        assert_eq!(id.id(), 42);
        assert_eq!(u64::from(id.clone()), 42);
        assert_eq!(ElementId::from(42), id);
    }

    #[test]
    fn test_id_generator() {
        let mut gen = IdGenerator::new();
        assert_eq!(gen.generate().id(), 1);
        assert_eq!(gen.generate().id(), 2);
        assert_eq!(gen.peek().id(), 3);
        assert_eq!(gen.generate().id(), 3);

        gen.reset();
        assert_eq!(gen.generate().id(), 1);
    }

    #[test]
    fn test_id_generator_starting_from() {
        let mut gen = IdGenerator::starting_from(100);
        assert_eq!(gen.generate().id(), 100);
        assert_eq!(gen.generate().id(), 101);
    }
}
