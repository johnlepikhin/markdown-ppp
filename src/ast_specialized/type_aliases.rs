//! Type aliases for specialized AST types
//!
//! This module provides convenient type aliases for generic AST types
//! specialized with element IDs.

use super::ElementId;
use crate::ast::generic;

/// AST types with element IDs
pub mod with_ids {
    use super::*;

    /// Document with element IDs
    pub type Document = generic::Document<ElementId>;

    /// Block with element ID
    pub type Block = generic::Block<ElementId>;

    /// Inline element with element ID
    pub type Inline = generic::Inline<ElementId>;

    /// Heading with element ID
    pub type Heading = generic::Heading<ElementId>;

    /// List with element ID
    pub type List = generic::List<ElementId>;

    /// List item with element ID
    pub type ListItem = generic::ListItem<ElementId>;

    /// Code block with element ID
    pub type CodeBlock = generic::CodeBlock<ElementId>;

    /// Link definition with element ID
    pub type LinkDefinition = generic::LinkDefinition<ElementId>;

    /// Table with element ID
    pub type Table = generic::Table<ElementId>;

    /// Table row with element IDs
    pub type TableRow = generic::TableRow<ElementId>;

    /// Table cell with element IDs
    pub type TableCell = generic::TableCell<ElementId>;

    /// Footnote definition with element ID
    pub type FootnoteDefinition = generic::FootnoteDefinition<ElementId>;

    /// GitHub alert with element ID
    pub type GitHubAlert = generic::GitHubAlertNode<ElementId>;

    /// Link with element ID
    pub type Link = generic::Link<ElementId>;

    /// Image with element ID
    pub type Image = generic::Image<ElementId>;

    /// Link reference with element ID
    pub type LinkReference = generic::LinkReference<ElementId>;
}
