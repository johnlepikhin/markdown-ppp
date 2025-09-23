//! Generic transformer support for AST nodes with user data
//!
//! This module provides transformer traits that work with the generic AST types
//! that support user-defined data. This allows for powerful transformations
//! while preserving or modifying user data attached to AST nodes.
//!
//! # Example
//!
//! ```rust
//! use markdown_ppp::ast::generic::*;
//! use markdown_ppp::ast_transform::{GenericTransformer, GenericExpandWith};
//!
//! #[derive(Debug, Clone, PartialEq, Default)]
//! struct NodeId(u32);
//!
//! struct IdAssigner {
//!     next_id: u32,
//! }
//!
//! impl GenericTransformer<NodeId> for IdAssigner {
//!     fn expand_block(&mut self, block: Block<NodeId>) -> Vec<Block<NodeId>> {
//!         // Assign new ID and potentially split block
//!         match block {
//!             Block::Paragraph { content, .. } if content.len() > 5 => {
//!                 // Split long paragraphs into two
//!                 let mid = content.len() / 2;
//!                 let (first_half, second_half) = content.split_at(mid);
//!                 vec![
//!                     Block::Paragraph {
//!                         content: first_half.to_vec(),
//!                         user_data: NodeId(self.next_id)
//!                     },
//!                     Block::Paragraph {
//!                         content: second_half.to_vec(),
//!                         user_data: NodeId(self.next_id + 1)
//!                     },
//!                 ]
//!             }
//!             other => vec![self.walk_transform_block(other)]
//!         }
//!     }
//! }
//! ```

use crate::ast::generic::*;

/// Generic transformer trait for AST nodes with user data
///
/// This trait provides the same transformation capabilities as the regular
/// Transformer trait, but works with generic AST nodes that contain user data.
pub trait GenericTransformer<T> {
    /// Transform a document with user data
    fn transform_document(&mut self, doc: Document<T>) -> Document<T> {
        self.walk_transform_document(doc)
    }

    /// Transform a block with user data
    fn transform_block(&mut self, block: Block<T>) -> Block<T> {
        self.walk_transform_block(block)
    }

    /// Transform an inline with user data
    fn transform_inline(&mut self, inline: Inline<T>) -> Inline<T> {
        self.walk_transform_inline(inline)
    }

    /// Transform a table cell with user data
    fn transform_table_cell(&mut self, cell: TableCell<T>) -> TableCell<T> {
        self.walk_transform_table_cell(cell)
    }

    /// Transform a list item with user data
    fn transform_list_item(&mut self, item: ListItem<T>) -> ListItem<T> {
        self.walk_transform_list_item(item)
    }

    /// Transform a table row with user data
    fn transform_table_row(&mut self, row: TableRow<T>) -> TableRow<T> {
        self.walk_transform_table_row(row)
    }

    /// Transform a heading with user data
    fn transform_heading(&mut self, heading: Heading<T>) -> Heading<T> {
        self.walk_transform_heading(heading)
    }

    /// Transform a link with user data
    fn transform_link(&mut self, link: Link<T>) -> Link<T> {
        self.walk_transform_link(link)
    }

    /// Transform an image with user data
    fn transform_image(&mut self, image: Image<T>) -> Image<T> {
        self.walk_transform_image(image)
    }

    /// Transform a code block with user data
    fn transform_code_block(&mut self, code_block: CodeBlock<T>) -> CodeBlock<T> {
        self.walk_transform_code_block(code_block)
    }

    /// Transform a footnote definition with user data
    fn transform_footnote_definition(
        &mut self,
        footnote: FootnoteDefinition<T>,
    ) -> FootnoteDefinition<T> {
        self.walk_transform_footnote_definition(footnote)
    }

    /// Transform a GitHub alert with user data
    fn transform_github_alert(&mut self, alert: GitHubAlertNode<T>) -> GitHubAlertNode<T> {
        self.walk_transform_github_alert(alert)
    }

    // ——————————————————————————————————————————————————————————————————————————
    // Expandable transformation methods (1-to-many) for generic types
    // ——————————————————————————————————————————————————————————————————————————

    /// Transform a document with possibility to expand into multiple documents
    fn expand_document(&mut self, doc: Document<T>) -> Vec<Document<T>> {
        vec![self.transform_document(doc)]
    }

    /// Transform a block with possibility to expand into multiple blocks
    fn expand_block(&mut self, block: Block<T>) -> Vec<Block<T>> {
        vec![self.transform_block(block)]
    }

    /// Transform an inline with possibility to expand into multiple inlines
    fn expand_inline(&mut self, inline: Inline<T>) -> Vec<Inline<T>> {
        vec![self.transform_inline(inline)]
    }

    /// Transform a table cell with possibility to expand into multiple cells
    fn expand_table_cell(&mut self, cell: TableCell<T>) -> Vec<TableCell<T>> {
        vec![self.transform_table_cell(cell)]
    }

    /// Transform a list item with possibility to expand into multiple items
    fn expand_list_item(&mut self, item: ListItem<T>) -> Vec<ListItem<T>> {
        vec![self.transform_list_item(item)]
    }

    /// Transform a table row with possibility to expand into multiple rows
    fn expand_table_row(&mut self, row: TableRow<T>) -> Vec<TableRow<T>> {
        vec![self.transform_table_row(row)]
    }

    /// Transform a heading with possibility to expand into multiple headings
    fn expand_heading(&mut self, heading: Heading<T>) -> Vec<Heading<T>> {
        vec![self.transform_heading(heading)]
    }

    /// Transform a link with possibility to expand into multiple links
    fn expand_link(&mut self, link: Link<T>) -> Vec<Link<T>> {
        vec![self.transform_link(link)]
    }

    /// Transform an image with possibility to expand into multiple images
    fn expand_image(&mut self, image: Image<T>) -> Vec<Image<T>> {
        vec![self.transform_image(image)]
    }

    /// Transform a code block with possibility to expand into multiple code blocks
    fn expand_code_block(&mut self, code_block: CodeBlock<T>) -> Vec<CodeBlock<T>> {
        vec![self.transform_code_block(code_block)]
    }

    /// Transform a footnote definition with possibility to expand into multiple definitions
    fn expand_footnote_definition(
        &mut self,
        footnote: FootnoteDefinition<T>,
    ) -> Vec<FootnoteDefinition<T>> {
        vec![self.transform_footnote_definition(footnote)]
    }

    /// Transform a GitHub alert with possibility to expand into multiple alerts
    fn expand_github_alert(&mut self, alert: GitHubAlertNode<T>) -> Vec<GitHubAlertNode<T>> {
        vec![self.transform_github_alert(alert)]
    }

    // ——————————————————————————————————————————————————————————————————————————
    // Default implementations for transformations
    // ——————————————————————————————————————————————————————————————————————————

    /// Default transformation for document with user data
    fn walk_transform_document(&mut self, mut doc: Document<T>) -> Document<T> {
        doc.blocks = doc
            .blocks
            .into_iter()
            .map(|block| self.transform_block(block))
            .collect();
        doc
    }

    /// Default transformation for block nodes with user data
    fn walk_transform_block(&mut self, block: Block<T>) -> Block<T> {
        match block {
            Block::Paragraph { content, user_data } => Block::Paragraph {
                content: content
                    .into_iter()
                    .map(|inline| self.transform_inline(inline))
                    .collect(),
                user_data,
            },
            Block::Heading(heading) => Block::Heading(self.transform_heading(heading)),
            Block::BlockQuote { blocks, user_data } => Block::BlockQuote {
                blocks: blocks
                    .into_iter()
                    .map(|block| self.transform_block(block))
                    .collect(),
                user_data,
            },
            Block::List(list) => Block::List(self.transform_list_item_container(list)),
            Block::Table(table) => Block::Table(self.transform_table(table)),
            Block::FootnoteDefinition(footnote) => {
                Block::FootnoteDefinition(self.transform_footnote_definition(footnote))
            }
            Block::GitHubAlert(alert) => Block::GitHubAlert(self.transform_github_alert(alert)),
            Block::Definition(mut def) => {
                def.label = def
                    .label
                    .into_iter()
                    .map(|inline| self.transform_inline(inline))
                    .collect();
                Block::Definition(def)
            }
            Block::CodeBlock(code_block) => Block::CodeBlock(self.transform_code_block(code_block)),
            // Terminal nodes - no transformation needed
            other => other,
        }
    }

    /// Default transformation for inline nodes with user data
    fn walk_transform_inline(&mut self, inline: Inline<T>) -> Inline<T> {
        match inline {
            Inline::Emphasis { content, user_data } => Inline::Emphasis {
                content: content
                    .into_iter()
                    .map(|inline| self.transform_inline(inline))
                    .collect(),
                user_data,
            },
            Inline::Strong { content, user_data } => Inline::Strong {
                content: content
                    .into_iter()
                    .map(|inline| self.transform_inline(inline))
                    .collect(),
                user_data,
            },
            Inline::Strikethrough { content, user_data } => Inline::Strikethrough {
                content: content
                    .into_iter()
                    .map(|inline| self.transform_inline(inline))
                    .collect(),
                user_data,
            },
            Inline::Link(link) => Inline::Link(self.transform_link(link)),
            Inline::LinkReference(mut link_ref) => {
                link_ref.label = link_ref
                    .label
                    .into_iter()
                    .map(|inline| self.transform_inline(inline))
                    .collect();
                link_ref.text = link_ref
                    .text
                    .into_iter()
                    .map(|inline| self.transform_inline(inline))
                    .collect();
                Inline::LinkReference(link_ref)
            }
            Inline::Image(image) => Inline::Image(self.transform_image(image)),
            // Terminal nodes - no transformation needed
            other => other,
        }
    }

    /// Default transformation for table cells with user data
    fn walk_transform_table_cell(&mut self, cell: TableCell<T>) -> TableCell<T> {
        cell.into_iter()
            .map(|inline| self.transform_inline(inline))
            .collect()
    }

    /// Default transformation for list items with user data
    fn walk_transform_list_item(&mut self, mut item: ListItem<T>) -> ListItem<T> {
        item.blocks = item
            .blocks
            .into_iter()
            .map(|block| self.transform_block(block))
            .collect();
        item
    }

    /// Default transformation for table rows with user data
    fn walk_transform_table_row(&mut self, row: TableRow<T>) -> TableRow<T> {
        row.into_iter()
            .map(|cell| self.transform_table_cell(cell))
            .collect()
    }

    /// Default transformation for headings with user data
    fn walk_transform_heading(&mut self, mut heading: Heading<T>) -> Heading<T> {
        heading.content = heading
            .content
            .into_iter()
            .map(|inline| self.transform_inline(inline))
            .collect();
        heading
    }

    /// Default transformation for links with user data
    fn walk_transform_link(&mut self, mut link: Link<T>) -> Link<T> {
        link.children = link
            .children
            .into_iter()
            .map(|inline| self.transform_inline(inline))
            .collect();
        link
    }

    /// Default transformation for images with user data
    fn walk_transform_image(&mut self, image: Image<T>) -> Image<T> {
        // Images are terminal nodes
        image
    }

    /// Default transformation for code blocks with user data
    fn walk_transform_code_block(&mut self, code_block: CodeBlock<T>) -> CodeBlock<T> {
        // Code blocks are terminal nodes
        code_block
    }

    /// Default transformation for footnote definitions with user data
    fn walk_transform_footnote_definition(
        &mut self,
        mut footnote: FootnoteDefinition<T>,
    ) -> FootnoteDefinition<T> {
        footnote.blocks = footnote
            .blocks
            .into_iter()
            .map(|block| self.transform_block(block))
            .collect();
        footnote
    }

    /// Default transformation for GitHub alerts with user data
    fn walk_transform_github_alert(&mut self, mut alert: GitHubAlertNode<T>) -> GitHubAlertNode<T> {
        alert.blocks = alert
            .blocks
            .into_iter()
            .map(|block| self.transform_block(block))
            .collect();
        alert
    }

    // Helper methods for composite structures

    /// Helper to transform list container
    fn transform_list_item_container(&mut self, mut list: List<T>) -> List<T> {
        list.items = list
            .items
            .into_iter()
            .map(|item| self.transform_list_item(item))
            .collect();
        list
    }

    /// Helper to transform table
    fn transform_table(&mut self, mut table: Table<T>) -> Table<T> {
        table.rows = table
            .rows
            .into_iter()
            .map(|row| self.transform_table_row(row))
            .collect();
        table
    }

    // ——————————————————————————————————————————————————————————————————————————
    // Walk methods for expandable transformations with user data
    // ——————————————————————————————————————————————————————————————————————————

    /// Default expandable transformation for document using flat_map
    fn walk_expand_document(&mut self, mut doc: Document<T>) -> Vec<Document<T>> {
        doc.blocks = doc
            .blocks
            .into_iter()
            .flat_map(|block| self.walk_expand_block(block))
            .collect();
        vec![doc]
    }

    /// Default expandable transformation for block nodes using flat_map
    fn walk_expand_block(&mut self, block: Block<T>) -> Vec<Block<T>> {
        let transformed_block = match block {
            Block::Paragraph { content, user_data } => Block::Paragraph {
                content: content
                    .into_iter()
                    .flat_map(|inline| self.walk_expand_inline(inline))
                    .collect(),
                user_data,
            },
            Block::Heading(heading) => {
                let expanded_headings = self.expand_heading(heading);
                return expanded_headings.into_iter().map(Block::Heading).collect();
            }
            Block::BlockQuote { blocks, user_data } => Block::BlockQuote {
                blocks: blocks
                    .into_iter()
                    .flat_map(|block| self.walk_expand_block(block))
                    .collect(),
                user_data,
            },
            Block::List(list) => {
                let expanded_list = self.expand_list_container(list);
                return expanded_list.into_iter().map(Block::List).collect();
            }
            Block::Table(table) => {
                let expanded_table = self.expand_table_container(table);
                return expanded_table.into_iter().map(Block::Table).collect();
            }
            Block::FootnoteDefinition(footnote) => {
                let expanded_footnotes = self.expand_footnote_definition(footnote);
                return expanded_footnotes
                    .into_iter()
                    .map(Block::FootnoteDefinition)
                    .collect();
            }
            Block::GitHubAlert(alert) => {
                let expanded_alerts = self.expand_github_alert(alert);
                return expanded_alerts
                    .into_iter()
                    .map(Block::GitHubAlert)
                    .collect();
            }
            Block::Definition(mut def) => {
                def.label = def
                    .label
                    .into_iter()
                    .flat_map(|inline| self.walk_expand_inline(inline))
                    .collect();
                Block::Definition(def)
            }
            Block::CodeBlock(code_block) => {
                let expanded_code_blocks = self.expand_code_block(code_block);
                return expanded_code_blocks
                    .into_iter()
                    .map(Block::CodeBlock)
                    .collect();
            }
            // Terminal nodes - no transformation needed
            other => other,
        };
        vec![transformed_block]
    }

    /// Default expandable transformation for inline nodes using flat_map
    fn walk_expand_inline(&mut self, inline: Inline<T>) -> Vec<Inline<T>> {
        let transformed_inline = match inline {
            Inline::Emphasis { content, user_data } => Inline::Emphasis {
                content: content
                    .into_iter()
                    .flat_map(|inline| self.walk_expand_inline(inline))
                    .collect(),
                user_data,
            },
            Inline::Strong { content, user_data } => Inline::Strong {
                content: content
                    .into_iter()
                    .flat_map(|inline| self.walk_expand_inline(inline))
                    .collect(),
                user_data,
            },
            Inline::Strikethrough { content, user_data } => Inline::Strikethrough {
                content: content
                    .into_iter()
                    .flat_map(|inline| self.walk_expand_inline(inline))
                    .collect(),
                user_data,
            },
            Inline::Link(link) => {
                let expanded_links = self.expand_link(link);
                return expanded_links.into_iter().map(Inline::Link).collect();
            }
            Inline::LinkReference(mut link_ref) => {
                link_ref.label = link_ref
                    .label
                    .into_iter()
                    .flat_map(|inline| self.walk_expand_inline(inline))
                    .collect();
                link_ref.text = link_ref
                    .text
                    .into_iter()
                    .flat_map(|inline| self.walk_expand_inline(inline))
                    .collect();
                Inline::LinkReference(link_ref)
            }
            Inline::Image(image) => {
                let expanded_images = self.expand_image(image);
                return expanded_images.into_iter().map(Inline::Image).collect();
            }
            // Terminal nodes - no transformation needed
            other => other,
        };
        vec![transformed_inline]
    }

    // Helper methods for expandable transformations

    /// Helper to expand list container
    fn expand_list_container(&mut self, mut list: List<T>) -> Vec<List<T>> {
        list.items = list
            .items
            .into_iter()
            .flat_map(|item| self.expand_list_item(item))
            .collect();
        vec![list]
    }

    /// Helper to expand table container
    fn expand_table_container(&mut self, mut table: Table<T>) -> Vec<Table<T>> {
        table.rows = table
            .rows
            .into_iter()
            .flat_map(|row| self.expand_table_row(row))
            .collect();
        vec![table]
    }
}

/// Extension trait for generic transformations
pub trait GenericTransformWith<T> {
    /// Apply a generic transformer to this AST node
    fn transform_with<Tr: GenericTransformer<T>>(self, transformer: &mut Tr) -> Self;
}

impl<T> GenericTransformWith<T> for Document<T> {
    fn transform_with<Tr: GenericTransformer<T>>(self, transformer: &mut Tr) -> Self {
        transformer.transform_document(self)
    }
}

impl<T> GenericTransformWith<T> for Block<T> {
    fn transform_with<Tr: GenericTransformer<T>>(self, transformer: &mut Tr) -> Self {
        transformer.transform_block(self)
    }
}

impl<T> GenericTransformWith<T> for Inline<T> {
    fn transform_with<Tr: GenericTransformer<T>>(self, transformer: &mut Tr) -> Self {
        transformer.transform_inline(self)
    }
}

/// Extension trait for generic expandable transformations
pub trait GenericExpandWith<T> {
    /// Apply a generic expandable transformer to this AST node, returning multiple nodes
    fn expand_with<Tr: GenericTransformer<T>>(self, transformer: &mut Tr) -> Vec<Self>
    where
        Self: Sized;
}

impl<T> GenericExpandWith<T> for Document<T> {
    fn expand_with<Tr: GenericTransformer<T>>(self, transformer: &mut Tr) -> Vec<Self> {
        transformer.walk_expand_document(self)
    }
}

impl<T> GenericExpandWith<T> for Block<T> {
    fn expand_with<Tr: GenericTransformer<T>>(self, transformer: &mut Tr) -> Vec<Self> {
        transformer.walk_expand_block(self)
    }
}

impl<T> GenericExpandWith<T> for Inline<T> {
    fn expand_with<Tr: GenericTransformer<T>>(self, transformer: &mut Tr) -> Vec<Self> {
        transformer.walk_expand_inline(self)
    }
}
