//! Transformer pattern for AST modifications
//!
//! This module provides the Transformer trait for modifying AST nodes in place.
//! Unlike the visitor pattern which is read-only, transformers consume and
//! return modified AST nodes.
//!
//! # Example
//!
//! ```rust
//! use markdown_ppp::ast::*;
//! use markdown_ppp::ast_transform::{Transformer, TransformWith};
//!
//! struct UppercaseTransformer;
//!
//! impl Transformer for UppercaseTransformer {
//!     fn transform_inline(&mut self, inline: Inline) -> Inline {
//!         match inline {
//!             Inline::Text(text) => Inline::Text(text.to_uppercase()),
//!             other => self.walk_transform_inline(other),
//!         }
//!     }
//! }
//!
//! let doc = Document {
//!     blocks: vec![Block::Paragraph(vec![Inline::Text("hello".to_string())])],
//! };
//!
//! let result = doc.transform_with(&mut UppercaseTransformer);
//! ```

use crate::ast::*;

/// Transformer trait for modifying AST nodes
///
/// Provides default implementations that recursively transform child nodes.
/// Override specific methods to implement custom transformation logic.
///
/// # Example
///
/// ```rust
/// use markdown_ppp::ast::*;
/// use markdown_ppp::ast_transform::Transformer;
///
/// struct UppercaseTransformer;
///
/// impl Transformer for UppercaseTransformer {
///     fn transform_inline(&mut self, inline: Inline) -> Inline {
///         match inline {
///             Inline::Text(text) => Inline::Text(text.to_uppercase()),
///             other => self.walk_transform_inline(other),
///         }
///     }
/// }
/// ```
pub trait Transformer {
    /// Transform a document node
    fn transform_document(&mut self, doc: Document) -> Document {
        self.walk_transform_document(doc)
    }

    /// Transform a block node
    fn transform_block(&mut self, block: Block) -> Block {
        self.walk_transform_block(block)
    }

    /// Transform an inline node
    fn transform_inline(&mut self, inline: Inline) -> Inline {
        self.walk_transform_inline(inline)
    }

    /// Transform a table cell
    fn transform_table_cell(&mut self, cell: TableCell) -> TableCell {
        self.walk_transform_table_cell(cell)
    }

    /// Transform a list item
    fn transform_list_item(&mut self, item: ListItem) -> ListItem {
        self.walk_transform_list_item(item)
    }

    /// Transform a table row
    fn transform_table_row(&mut self, row: TableRow) -> TableRow {
        self.walk_transform_table_row(row)
    }

    /// Transform a heading
    fn transform_heading(&mut self, heading: Heading) -> Heading {
        self.walk_transform_heading(heading)
    }

    /// Transform a link
    fn transform_link(&mut self, link: Link) -> Link {
        self.walk_transform_link(link)
    }

    /// Transform an image
    fn transform_image(&mut self, image: Image) -> Image {
        self.walk_transform_image(image)
    }

    /// Transform a code block
    fn transform_code_block(&mut self, code_block: CodeBlock) -> CodeBlock {
        self.walk_transform_code_block(code_block)
    }

    /// Transform text content
    fn transform_text(&mut self, text: String) -> String {
        self.walk_transform_text(text)
    }

    /// Transform a footnote definition
    fn transform_footnote_definition(
        &mut self,
        footnote: FootnoteDefinition,
    ) -> FootnoteDefinition {
        self.walk_transform_footnote_definition(footnote)
    }

    /// Transform a GitHub alert
    fn transform_github_alert(&mut self, alert: GitHubAlert) -> GitHubAlert {
        self.walk_transform_github_alert(alert)
    }

    /// Default transformation for document
    fn walk_transform_document(&mut self, mut doc: Document) -> Document {
        doc.blocks = doc
            .blocks
            .into_iter()
            .map(|block| self.transform_block(block))
            .collect();
        doc
    }

    /// Default transformation for block nodes
    fn walk_transform_block(&mut self, block: Block) -> Block {
        match block {
            Block::Paragraph(inlines) => Block::Paragraph(
                inlines
                    .into_iter()
                    .map(|inline| self.transform_inline(inline))
                    .collect(),
            ),
            Block::Heading(heading) => Block::Heading(self.transform_heading(heading)),
            Block::BlockQuote(blocks) => Block::BlockQuote(
                blocks
                    .into_iter()
                    .map(|block| self.transform_block(block))
                    .collect(),
            ),
            Block::List(mut list) => {
                list.items = list
                    .items
                    .into_iter()
                    .map(|item| self.transform_list_item(item))
                    .collect();
                Block::List(list)
            }
            Block::Table(mut table) => {
                table.rows = table
                    .rows
                    .into_iter()
                    .map(|row| self.transform_table_row(row))
                    .collect();
                Block::Table(table)
            }
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

    /// Default transformation for inline nodes
    fn walk_transform_inline(&mut self, inline: Inline) -> Inline {
        match inline {
            Inline::Emphasis(inlines) => Inline::Emphasis(
                inlines
                    .into_iter()
                    .map(|inline| self.transform_inline(inline))
                    .collect(),
            ),
            Inline::Strong(inlines) => Inline::Strong(
                inlines
                    .into_iter()
                    .map(|inline| self.transform_inline(inline))
                    .collect(),
            ),
            Inline::Strikethrough(inlines) => Inline::Strikethrough(
                inlines
                    .into_iter()
                    .map(|inline| self.transform_inline(inline))
                    .collect(),
            ),
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
            Inline::Text(text) => Inline::Text(self.transform_text(text)),
            // Terminal nodes - no transformation needed
            other => other,
        }
    }

    /// Default transformation for table cells
    fn walk_transform_table_cell(&mut self, cell: TableCell) -> TableCell {
        cell.into_iter()
            .map(|inline| self.transform_inline(inline))
            .collect()
    }

    /// Default transformation for list items
    fn walk_transform_list_item(&mut self, mut item: ListItem) -> ListItem {
        item.blocks = item
            .blocks
            .into_iter()
            .map(|block| self.transform_block(block))
            .collect();
        item
    }

    /// Default transformation for table rows
    fn walk_transform_table_row(&mut self, row: TableRow) -> TableRow {
        row.into_iter()
            .map(|cell| self.transform_table_cell(cell))
            .collect()
    }

    /// Default transformation for headings
    fn walk_transform_heading(&mut self, mut heading: Heading) -> Heading {
        heading.content = heading
            .content
            .into_iter()
            .map(|inline| self.transform_inline(inline))
            .collect();
        heading
    }

    /// Default transformation for links
    fn walk_transform_link(&mut self, mut link: Link) -> Link {
        link.children = link
            .children
            .into_iter()
            .map(|inline| self.transform_inline(inline))
            .collect();
        link
    }

    /// Default transformation for images
    fn walk_transform_image(&mut self, image: Image) -> Image {
        // Images are terminal nodes
        image
    }

    /// Default transformation for code blocks
    fn walk_transform_code_block(&mut self, code_block: CodeBlock) -> CodeBlock {
        // Code blocks are terminal nodes
        code_block
    }

    /// Default transformation for text
    fn walk_transform_text(&mut self, text: String) -> String {
        // Text is a terminal node
        text
    }

    // ——————————————————————————————————————————————————————————————————————————
    // Expandable transformation methods (1-to-many)
    // ——————————————————————————————————————————————————————————————————————————

    /// Transform a document with possibility to expand into multiple documents
    ///
    /// Default implementation delegates to the regular transform_document method
    fn expand_document(&mut self, doc: Document) -> Vec<Document> {
        vec![self.transform_document(doc)]
    }

    /// Transform a block with possibility to expand into multiple blocks
    ///
    /// This enables scenarios like:
    /// - Split one paragraph into paragraph + list
    /// - Expand one block into multiple blocks
    /// - Transform one block based on content patterns
    ///
    /// Default implementation delegates to the regular transform_block method
    fn expand_block(&mut self, block: Block) -> Vec<Block> {
        vec![self.transform_block(block)]
    }

    /// Transform an inline with possibility to expand into multiple inlines
    ///
    /// This enables scenarios like:
    /// - Split text into multiple text nodes
    /// - Transform one inline element into several elements
    /// - Expand abbreviations or macros
    ///
    /// Default implementation delegates to the regular transform_inline method
    fn expand_inline(&mut self, inline: Inline) -> Vec<Inline> {
        vec![self.transform_inline(inline)]
    }

    /// Transform a table cell with possibility to expand into multiple cells
    fn expand_table_cell(&mut self, cell: TableCell) -> Vec<TableCell> {
        vec![self.transform_table_cell(cell)]
    }

    /// Transform a list item with possibility to expand into multiple items
    fn expand_list_item(&mut self, item: ListItem) -> Vec<ListItem> {
        vec![self.transform_list_item(item)]
    }

    /// Transform a table row with possibility to expand into multiple rows
    fn expand_table_row(&mut self, row: TableRow) -> Vec<TableRow> {
        vec![self.transform_table_row(row)]
    }

    /// Transform a heading with possibility to expand into multiple headings
    fn expand_heading(&mut self, heading: Heading) -> Vec<Heading> {
        vec![self.transform_heading(heading)]
    }

    /// Transform a link with possibility to expand into multiple links
    fn expand_link(&mut self, link: Link) -> Vec<Link> {
        vec![self.transform_link(link)]
    }

    /// Transform an image with possibility to expand into multiple images
    fn expand_image(&mut self, image: Image) -> Vec<Image> {
        vec![self.transform_image(image)]
    }

    /// Transform a code block with possibility to expand into multiple code blocks
    fn expand_code_block(&mut self, code_block: CodeBlock) -> Vec<CodeBlock> {
        vec![self.transform_code_block(code_block)]
    }

    /// Transform text with possibility to expand into multiple text strings
    fn expand_text(&mut self, text: String) -> Vec<String> {
        vec![self.transform_text(text)]
    }

    /// Transform a footnote definition with possibility to expand into multiple definitions
    fn expand_footnote_definition(
        &mut self,
        footnote: FootnoteDefinition,
    ) -> Vec<FootnoteDefinition> {
        vec![self.transform_footnote_definition(footnote)]
    }

    /// Transform a GitHub alert with possibility to expand into multiple alerts
    fn expand_github_alert(&mut self, alert: GitHubAlert) -> Vec<GitHubAlert> {
        vec![self.transform_github_alert(alert)]
    }

    // ——————————————————————————————————————————————————————————————————————————
    // Walk methods for expandable transformations
    // ——————————————————————————————————————————————————————————————————————————

    /// Walk document with expandable transformations
    ///
    /// Use this when you want to apply expandable transformations with the default
    /// behavior of using expand_* methods for child nodes.
    fn walk_expand_document(&mut self, mut doc: Document) -> Vec<Document> {
        doc.blocks = doc
            .blocks
            .into_iter()
            .flat_map(|block| self.walk_expand_block(block))
            .collect();
        vec![doc]
    }

    /// Walk block with expandable transformations
    ///
    /// Override this method to implement custom expandable block transformations.
    /// By default, delegates to transform_block (1-to-1 transformation) but processes
    /// child nodes with expandable transformations.
    fn walk_expand_block(&mut self, block: Block) -> Vec<Block> {
        match block {
            Block::Paragraph(inlines) => {
                let expanded_inlines: Vec<Inline> = inlines
                    .into_iter()
                    .flat_map(|inline| self.walk_expand_inline(inline))
                    .collect();
                vec![Block::Paragraph(expanded_inlines)]
            }
            Block::Heading(mut heading) => {
                heading.content = heading
                    .content
                    .into_iter()
                    .flat_map(|inline| self.walk_expand_inline(inline))
                    .collect();
                vec![Block::Heading(heading)]
            }
            Block::BlockQuote(blocks) => {
                let expanded_blocks: Vec<Block> = blocks
                    .into_iter()
                    .flat_map(|block| self.walk_expand_block(block))
                    .collect();
                vec![Block::BlockQuote(expanded_blocks)]
            }
            Block::List(mut list) => {
                list.items = list
                    .items
                    .into_iter()
                    .flat_map(|item| self.walk_expand_list_item(item))
                    .collect();
                vec![Block::List(list)]
            }
            Block::Table(mut table) => {
                table.rows = table
                    .rows
                    .into_iter()
                    .flat_map(|row| self.walk_expand_table_row(row))
                    .collect();
                vec![Block::Table(table)]
            }
            Block::FootnoteDefinition(mut footnote) => {
                footnote.blocks = footnote
                    .blocks
                    .into_iter()
                    .flat_map(|block| self.walk_expand_block(block))
                    .collect();
                vec![Block::FootnoteDefinition(footnote)]
            }
            Block::GitHubAlert(mut alert) => {
                alert.blocks = alert
                    .blocks
                    .into_iter()
                    .flat_map(|block| self.walk_expand_block(block))
                    .collect();
                vec![Block::GitHubAlert(alert)]
            }
            Block::Definition(mut def) => {
                def.label = def
                    .label
                    .into_iter()
                    .flat_map(|inline| self.walk_expand_inline(inline))
                    .collect();
                vec![Block::Definition(def)]
            }
            // Terminal nodes - no transformation needed
            other => vec![self.transform_block(other)],
        }
    }

    /// Walk inline with expandable transformations
    ///
    /// Override this method to implement custom expandable inline transformations.
    /// By default, delegates to transform_inline (1-to-1 transformation).
    fn walk_expand_inline(&mut self, inline: Inline) -> Vec<Inline> {
        vec![self.transform_inline(inline)]
    }

    /// Walk table cell with expandable transformations
    fn walk_expand_table_cell(&mut self, cell: TableCell) -> Vec<TableCell> {
        let expanded_cell = cell
            .into_iter()
            .flat_map(|inline| self.expand_inline(inline))
            .collect();
        vec![expanded_cell]
    }

    /// Walk list item with expandable transformations
    fn walk_expand_list_item(&mut self, mut item: ListItem) -> Vec<ListItem> {
        item.blocks = item
            .blocks
            .into_iter()
            .flat_map(|block| self.expand_block(block))
            .collect();
        vec![item]
    }

    /// Walk table row with expandable transformations
    fn walk_expand_table_row(&mut self, row: TableRow) -> Vec<TableRow> {
        let expanded_row = row
            .into_iter()
            .flat_map(|cell| self.expand_table_cell(cell))
            .collect();
        vec![expanded_row]
    }

    /// Walk heading with expandable transformations
    fn walk_expand_heading(&mut self, mut heading: Heading) -> Vec<Heading> {
        heading.content = heading
            .content
            .into_iter()
            .flat_map(|inline| self.expand_inline(inline))
            .collect();
        vec![heading]
    }

    /// Walk link with expandable transformations
    fn walk_expand_link(&mut self, mut link: Link) -> Vec<Link> {
        link.children = link
            .children
            .into_iter()
            .flat_map(|inline| self.expand_inline(inline))
            .collect();
        vec![link]
    }

    /// Walk image with expandable transformations
    fn walk_expand_image(&mut self, image: Image) -> Vec<Image> {
        // Images are terminal nodes
        vec![image]
    }

    /// Walk code block with expandable transformations
    fn walk_expand_code_block(&mut self, code_block: CodeBlock) -> Vec<CodeBlock> {
        // Code blocks are terminal nodes
        vec![code_block]
    }

    /// Walk text with expandable transformations
    fn walk_expand_text(&mut self, text: String) -> Vec<String> {
        // Text is a terminal node
        vec![text]
    }

    /// Walk footnote definition with expandable transformations
    fn walk_expand_footnote_definition(
        &mut self,
        mut footnote: FootnoteDefinition,
    ) -> Vec<FootnoteDefinition> {
        footnote.blocks = footnote
            .blocks
            .into_iter()
            .flat_map(|block| self.expand_block(block))
            .collect();
        vec![footnote]
    }

    /// Walk GitHub alert with expandable transformations
    fn walk_expand_github_alert(&mut self, mut alert: GitHubAlert) -> Vec<GitHubAlert> {
        alert.blocks = alert
            .blocks
            .into_iter()
            .flat_map(|block| self.expand_block(block))
            .collect();
        vec![alert]
    }

    /// Default transformation for footnote definitions
    fn walk_transform_footnote_definition(
        &mut self,
        mut footnote: FootnoteDefinition,
    ) -> FootnoteDefinition {
        footnote.blocks = footnote
            .blocks
            .into_iter()
            .map(|block| self.transform_block(block))
            .collect();
        footnote
    }

    /// Default transformation for GitHub alerts
    fn walk_transform_github_alert(&mut self, mut alert: GitHubAlert) -> GitHubAlert {
        alert.blocks = alert
            .blocks
            .into_iter()
            .map(|block| self.transform_block(block))
            .collect();
        alert
    }
}

/// Extension trait for transforming documents
pub trait TransformWith {
    /// Apply a transformer to this AST node
    fn transform_with<T: Transformer>(self, transformer: &mut T) -> Self;
}

impl TransformWith for Document {
    fn transform_with<T: Transformer>(self, transformer: &mut T) -> Self {
        transformer.transform_document(self)
    }
}

impl TransformWith for Block {
    fn transform_with<T: Transformer>(self, transformer: &mut T) -> Self {
        transformer.transform_block(self)
    }
}

impl TransformWith for Inline {
    fn transform_with<T: Transformer>(self, transformer: &mut T) -> Self {
        transformer.transform_inline(self)
    }
}

/// Extension trait for expandable transformations
pub trait ExpandWith {
    /// Apply an expandable transformer to this AST node, returning multiple nodes
    fn expand_with<T: Transformer>(self, transformer: &mut T) -> Vec<Self>
    where
        Self: Sized;
}

impl ExpandWith for Document {
    fn expand_with<T: Transformer>(self, transformer: &mut T) -> Vec<Self> {
        transformer.walk_expand_document(self)
    }
}

impl ExpandWith for Block {
    fn expand_with<T: Transformer>(self, transformer: &mut T) -> Vec<Self> {
        transformer.walk_expand_block(self)
    }
}

impl ExpandWith for Inline {
    fn expand_with<T: Transformer>(self, transformer: &mut T) -> Vec<Self> {
        transformer.walk_expand_inline(self)
    }
}

/// Composite transformer that applies multiple transformers in sequence
pub struct CompositeTransformer {
    transformers: Vec<Box<dyn Transformer>>,
}

impl CompositeTransformer {
    /// Create a new composite transformer
    pub fn new() -> Self {
        Self {
            transformers: Vec::new(),
        }
    }

    /// Add a transformer to the sequence
    pub fn add_transformer<T: Transformer + 'static>(mut self, transformer: T) -> Self {
        self.transformers.push(Box::new(transformer));
        self
    }
}

impl Default for CompositeTransformer {
    fn default() -> Self {
        Self::new()
    }
}

impl Transformer for CompositeTransformer {
    fn transform_document(&mut self, mut doc: Document) -> Document {
        for transformer in &mut self.transformers {
            doc = transformer.transform_document(doc);
        }
        doc
    }

    fn transform_block(&mut self, mut block: Block) -> Block {
        for transformer in &mut self.transformers {
            block = transformer.transform_block(block);
        }
        block
    }

    fn transform_inline(&mut self, mut inline: Inline) -> Inline {
        for transformer in &mut self.transformers {
            inline = transformer.transform_inline(inline);
        }
        inline
    }
}
