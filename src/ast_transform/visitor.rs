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

    /// Visit a table cell
    fn visit_table_cell(&mut self, cell: &TableCell) {
        self.walk_table_cell(cell);
    }

    /// Visit a list item
    fn visit_list_item(&mut self, item: &ListItem) {
        self.walk_list_item(item);
    }

    /// Visit a table row
    fn visit_table_row(&mut self, row: &TableRow) {
        self.walk_table_row(row);
    }

    /// Visit a heading
    fn visit_heading(&mut self, heading: &Heading) {
        self.walk_heading(heading);
    }

    /// Visit a link
    fn visit_link(&mut self, link: &Link) {
        self.walk_link(link);
    }

    /// Visit an image
    fn visit_image(&mut self, image: &Image) {
        self.walk_image(image);
    }

    /// Visit a code block
    fn visit_code_block(&mut self, code_block: &CodeBlock) {
        self.walk_code_block(code_block);
    }

    /// Visit text content
    fn visit_text(&mut self, text: &str) {
        self.walk_text(text);
    }

    /// Visit a footnote definition
    fn visit_footnote_definition(&mut self, footnote: &FootnoteDefinition) {
        self.walk_footnote_definition(footnote);
    }

    /// Visit a GitHub alert
    fn visit_github_alert(&mut self, alert: &GitHubAlert) {
        self.walk_github_alert(alert);
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
                self.visit_heading(heading);
            }
            Block::BlockQuote(blocks) => {
                for block in blocks {
                    self.visit_block(block);
                }
            }
            Block::List(list) => {
                for item in &list.items {
                    self.visit_list_item(item);
                }
            }
            Block::Table(table) => {
                for row in &table.rows {
                    self.visit_table_row(row);
                }
            }
            Block::FootnoteDefinition(footnote) => {
                self.visit_footnote_definition(footnote);
            }
            Block::GitHubAlert(alert) => {
                self.visit_github_alert(alert);
            }
            Block::Definition(def) => {
                for inline in &def.label {
                    self.visit_inline(inline);
                }
            }
            Block::CodeBlock(code_block) => {
                self.visit_code_block(code_block);
            }
            // Terminal nodes - no traversal needed
            Block::ThematicBreak | Block::HtmlBlock(_) | Block::Empty => {}
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
                self.visit_link(link);
            }
            Inline::LinkReference(link_ref) => {
                for inline in &link_ref.label {
                    self.visit_inline(inline);
                }
                for inline in &link_ref.text {
                    self.visit_inline(inline);
                }
            }
            Inline::Image(image) => {
                self.visit_image(image);
            }
            Inline::Text(text) => {
                self.visit_text(text);
            }
            // Terminal nodes - no traversal needed
            Inline::LineBreak
            | Inline::Code(_)
            | Inline::Html(_)
            | Inline::Autolink(_)
            | Inline::FootnoteReference(_)
            | Inline::Empty => {}
        }
    }

    /// Default traversal for table cells
    fn walk_table_cell(&mut self, cell: &TableCell) {
        for inline in cell {
            self.visit_inline(inline);
        }
    }

    /// Default traversal for list items
    fn walk_list_item(&mut self, item: &ListItem) {
        for block in &item.blocks {
            self.visit_block(block);
        }
    }

    /// Default traversal for table rows
    fn walk_table_row(&mut self, row: &TableRow) {
        for cell in row {
            self.visit_table_cell(cell);
        }
    }

    /// Default traversal for headings
    fn walk_heading(&mut self, heading: &Heading) {
        for inline in &heading.content {
            self.visit_inline(inline);
        }
    }

    /// Default traversal for links
    fn walk_link(&mut self, link: &Link) {
        for inline in &link.children {
            self.visit_inline(inline);
        }
    }

    /// Default traversal for images
    fn walk_image(&mut self, _image: &Image) {
        // Images are terminal nodes with no child inlines to traverse
    }

    /// Default traversal for code blocks
    fn walk_code_block(&mut self, _code_block: &CodeBlock) {
        // Code blocks are terminal nodes
    }

    /// Default traversal for text
    fn walk_text(&mut self, _text: &str) {
        // Text is a terminal node
    }

    /// Default traversal for footnote definitions
    fn walk_footnote_definition(&mut self, footnote: &FootnoteDefinition) {
        for block in &footnote.blocks {
            self.visit_block(block);
        }
    }

    /// Default traversal for GitHub alerts
    fn walk_github_alert(&mut self, alert: &GitHubAlert) {
        for block in &alert.blocks {
            self.visit_block(block);
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
