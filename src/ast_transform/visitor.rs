//! Visitor pattern for read-only AST traversal
//!
//! This module provides the Visitor trait for read-only traversal of AST nodes.
//! Visitors are useful for collecting information, counting elements, or performing
//! analysis without modifying the AST structure.
//!
//! # Example
//!
//! ```rust
//! use markdown_ppp::ast::*;
//! use markdown_ppp::ast_transform::{Visitor, VisitWith};
//!
//! struct TextCollector {
//!     texts: Vec<String>,
//! }
//!
//! impl Visitor for TextCollector {
//!     fn visit_inline(&mut self, inline: &Inline) {
//!         if let Inline::Text(text) = inline {
//!             self.texts.push(text.clone());
//!         }
//!         self.walk_inline(inline);
//!     }
//! }
//!
//! let doc = Document {
//!     blocks: vec![Block::Paragraph(vec![Inline::Text("hello".to_string())])],
//! };
//!
//! let mut collector = TextCollector { texts: Vec::new() };
//! doc.visit_with(&mut collector);
//! assert_eq!(collector.texts, vec!["hello"]);
//! ```

use crate::ast::*;

/// Visitor trait for traversing AST nodes without modification
///
/// Provides default implementations that recursively visit child nodes.
/// Override specific methods to implement custom logic for different node types.
///
/// # Example
///
/// ```rust
/// use markdown_ppp::ast::*;
/// use markdown_ppp::ast_transform::Visitor;
///
/// struct TextCollector {
///     texts: Vec<String>,
/// }
///
/// impl Visitor for TextCollector {
///     fn visit_inline(&mut self, inline: &Inline) {
///         if let Inline::Text(text) = inline {
///             self.texts.push(text.clone());
///         }
///         // Continue with default traversal
///         self.walk_inline(inline);
///     }
/// }
/// ```
pub trait Visitor {
    /// Visit a document node
    fn visit_document(&mut self, doc: &Document) {
        self.walk_document(doc);
    }

    /// Visit a block node
    fn visit_block(&mut self, block: &Block) {
        self.walk_block(block);
    }

    /// Visit an inline node
    fn visit_inline(&mut self, inline: &Inline) {
        self.walk_inline(inline);
    }

    /// Default traversal for document
    fn walk_document(&mut self, doc: &Document) {
        for block in &doc.blocks {
            self.visit_block(block);
        }
    }

    /// Default traversal for block nodes
    fn walk_block(&mut self, block: &Block) {
        match block {
            Block::Paragraph(inlines) => {
                for inline in inlines {
                    self.visit_inline(inline);
                }
            }
            Block::Heading(heading) => {
                for inline in &heading.content {
                    self.visit_inline(inline);
                }
            }
            Block::BlockQuote(blocks) => {
                for block in blocks {
                    self.visit_block(block);
                }
            }
            Block::List(list) => {
                for item in &list.items {
                    for block in &item.blocks {
                        self.visit_block(block);
                    }
                }
            }
            Block::Table(table) => {
                for row in &table.rows {
                    for cell in row {
                        for inline in cell {
                            self.visit_inline(inline);
                        }
                    }
                }
            }
            Block::FootnoteDefinition(footnote) => {
                for block in &footnote.blocks {
                    self.visit_block(block);
                }
            }
            Block::GitHubAlert(alert) => {
                for block in &alert.blocks {
                    self.visit_block(block);
                }
            }
            Block::Definition(def) => {
                for inline in &def.label {
                    self.visit_inline(inline);
                }
            }
            // Terminal nodes - no traversal needed
            Block::ThematicBreak | Block::CodeBlock(_) | Block::HtmlBlock(_) | Block::Empty => {}
        }
    }

    /// Default traversal for inline nodes
    fn walk_inline(&mut self, inline: &Inline) {
        match inline {
            Inline::Emphasis(inlines)
            | Inline::Strong(inlines)
            | Inline::Strikethrough(inlines) => {
                for inline in inlines {
                    self.visit_inline(inline);
                }
            }
            Inline::Link(link) => {
                for inline in &link.children {
                    self.visit_inline(inline);
                }
            }
            Inline::LinkReference(link_ref) => {
                for inline in &link_ref.label {
                    self.visit_inline(inline);
                }
                for inline in &link_ref.text {
                    self.visit_inline(inline);
                }
            }
            // Terminal nodes - no traversal needed
            Inline::Text(_)
            | Inline::LineBreak
            | Inline::Code(_)
            | Inline::Html(_)
            | Inline::Image(_)
            | Inline::Autolink(_)
            | Inline::FootnoteReference(_)
            | Inline::Empty => {}
        }
    }
}

/// Extension trait for visiting documents
pub trait VisitWith {
    /// Apply a visitor to this AST node
    fn visit_with<V: Visitor>(&self, visitor: &mut V);
}

impl VisitWith for Document {
    fn visit_with<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_document(self);
    }
}

impl VisitWith for Block {
    fn visit_with<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_block(self);
    }
}

impl VisitWith for Inline {
    fn visit_with<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_inline(self);
    }
}
