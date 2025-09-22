//! Query API for finding elements in AST
//!
//! This module provides query methods for finding elements in the AST that match certain conditions.
//!
//! # Example
//!
//! ```rust
//! use markdown_ppp::ast::*;
//! use markdown_ppp::ast_transform::Query;
//!
//! let doc = Document {
//!     blocks: vec![
//!         Block::Paragraph(vec![
//!             Inline::Text("hello".to_string()),
//!             Inline::Autolink("https://example.com".to_string()),
//!         ]),
//!     ],
//! };
//!
//! // Find all autolinks
//! let autolinks = doc.find_all_inlines(|inline| {
//!     matches!(inline, Inline::Autolink(_))
//! });
//! assert_eq!(autolinks.len(), 1);
//!
//! // Count text nodes
//! let text_count = doc.count_inlines(|inline| {
//!     matches!(inline, Inline::Text(_))
//! });
//! assert_eq!(text_count, 1);
//! ```

use crate::ast::*;

/// Query trait for finding elements in AST structures
pub trait Query {
    /// Find all inline elements matching a predicate
    fn find_all_inlines<F>(&self, predicate: F) -> Vec<&Inline>
    where
        F: Fn(&Inline) -> bool;

    /// Find all block elements matching a predicate
    fn find_all_blocks<F>(&self, predicate: F) -> Vec<&Block>
    where
        F: Fn(&Block) -> bool;

    /// Find the first inline element matching a predicate
    fn find_first_inline<F>(&self, predicate: F) -> Option<&Inline>
    where
        F: Fn(&Inline) -> bool;

    /// Find the first block element matching a predicate
    fn find_first_block<F>(&self, predicate: F) -> Option<&Block>
    where
        F: Fn(&Block) -> bool;

    /// Count inline elements matching a predicate
    fn count_inlines<F>(&self, predicate: F) -> usize
    where
        F: Fn(&Inline) -> bool,
    {
        self.find_all_inlines(predicate).len()
    }

    /// Count block elements matching a predicate
    fn count_blocks<F>(&self, predicate: F) -> usize
    where
        F: Fn(&Block) -> bool,
    {
        self.find_all_blocks(predicate).len()
    }

    /// Check if any inline element matches a predicate
    fn any_inline<F>(&self, predicate: F) -> bool
    where
        F: Fn(&Inline) -> bool,
    {
        self.find_first_inline(predicate).is_some()
    }

    /// Check if any block element matches a predicate
    fn any_block<F>(&self, predicate: F) -> bool
    where
        F: Fn(&Block) -> bool,
    {
        self.find_first_block(predicate).is_some()
    }
}

impl Query for Document {
    fn find_all_inlines<F>(&self, predicate: F) -> Vec<&Inline>
    where
        F: Fn(&Inline) -> bool,
    {
        let mut results = Vec::new();
        for block in &self.blocks {
            results.extend(block.find_all_inlines(&predicate));
        }
        results
    }

    fn find_all_blocks<F>(&self, predicate: F) -> Vec<&Block>
    where
        F: Fn(&Block) -> bool,
    {
        let mut results = Vec::new();
        for block in &self.blocks {
            results.extend(block.find_all_blocks(&predicate));
        }
        results
    }

    fn find_first_inline<F>(&self, predicate: F) -> Option<&Inline>
    where
        F: Fn(&Inline) -> bool,
    {
        for block in &self.blocks {
            if let Some(inline) = block.find_first_inline(&predicate) {
                return Some(inline);
            }
        }
        None
    }

    fn find_first_block<F>(&self, predicate: F) -> Option<&Block>
    where
        F: Fn(&Block) -> bool,
    {
        for block in &self.blocks {
            if let Some(found) = block.find_first_block(&predicate) {
                return Some(found);
            }
        }
        None
    }
}

impl Query for Block {
    fn find_all_inlines<F>(&self, predicate: F) -> Vec<&Inline>
    where
        F: Fn(&Inline) -> bool,
    {
        let mut results = Vec::new();
        collect_inlines_from_block(self, &predicate, &mut results);
        results
    }

    fn find_all_blocks<F>(&self, predicate: F) -> Vec<&Block>
    where
        F: Fn(&Block) -> bool,
    {
        let mut results = Vec::new();
        collect_blocks_from_block(self, &predicate, &mut results);
        results
    }

    fn find_first_inline<F>(&self, predicate: F) -> Option<&Inline>
    where
        F: Fn(&Inline) -> bool,
    {
        find_first_inline_in_block(self, &predicate)
    }

    fn find_first_block<F>(&self, predicate: F) -> Option<&Block>
    where
        F: Fn(&Block) -> bool,
    {
        find_first_block_in_block(self, &predicate)
    }
}

impl Query for Vec<Inline> {
    fn find_all_inlines<F>(&self, predicate: F) -> Vec<&Inline>
    where
        F: Fn(&Inline) -> bool,
    {
        let mut results = Vec::new();
        for inline in self {
            collect_inlines_from_inline(inline, &predicate, &mut results);
        }
        results
    }

    fn find_all_blocks<F>(&self, _predicate: F) -> Vec<&Block>
    where
        F: Fn(&Block) -> bool,
    {
        Vec::new() // Inline elements don't contain blocks
    }

    fn find_first_inline<F>(&self, predicate: F) -> Option<&Inline>
    where
        F: Fn(&Inline) -> bool,
    {
        for inline in self {
            if let Some(found) = find_first_inline_in_inline(inline, &predicate) {
                return Some(found);
            }
        }
        None
    }

    fn find_first_block<F>(&self, _predicate: F) -> Option<&Block>
    where
        F: Fn(&Block) -> bool,
    {
        None // Inline elements don't contain blocks
    }
}

// Helper functions for recursive collection

fn collect_inlines_from_block<'a, F>(block: &'a Block, predicate: &F, results: &mut Vec<&'a Inline>)
where
    F: Fn(&Inline) -> bool,
{
    match block {
        Block::Paragraph(inlines) => {
            for inline in inlines {
                collect_inlines_from_inline(inline, predicate, results);
            }
        }
        Block::Heading(heading) => {
            for inline in &heading.content {
                collect_inlines_from_inline(inline, predicate, results);
            }
        }
        Block::BlockQuote(blocks) => {
            for block in blocks {
                collect_inlines_from_block(block, predicate, results);
            }
        }
        Block::List(list) => {
            for item in &list.items {
                for block in &item.blocks {
                    collect_inlines_from_block(block, predicate, results);
                }
            }
        }
        Block::Table(table) => {
            for row in &table.rows {
                for cell in row {
                    for inline in cell {
                        collect_inlines_from_inline(inline, predicate, results);
                    }
                }
            }
        }
        Block::FootnoteDefinition(footnote) => {
            for block in &footnote.blocks {
                collect_inlines_from_block(block, predicate, results);
            }
        }
        Block::GitHubAlert(alert) => {
            for block in &alert.blocks {
                collect_inlines_from_block(block, predicate, results);
            }
        }
        Block::Definition(def) => {
            for inline in &def.label {
                collect_inlines_from_inline(inline, predicate, results);
            }
        }
        _ => {} // Terminal blocks
    }
}

fn collect_inlines_from_inline<'a, F>(
    inline: &'a Inline,
    predicate: &F,
    results: &mut Vec<&'a Inline>,
) where
    F: Fn(&Inline) -> bool,
{
    if predicate(inline) {
        results.push(inline);
    }

    match inline {
        Inline::Emphasis(inlines) | Inline::Strong(inlines) | Inline::Strikethrough(inlines) => {
            for inline in inlines {
                collect_inlines_from_inline(inline, predicate, results);
            }
        }
        Inline::Link(link) => {
            for inline in &link.children {
                collect_inlines_from_inline(inline, predicate, results);
            }
        }
        Inline::LinkReference(link_ref) => {
            for inline in &link_ref.label {
                collect_inlines_from_inline(inline, predicate, results);
            }
            for inline in &link_ref.text {
                collect_inlines_from_inline(inline, predicate, results);
            }
        }
        _ => {} // Terminal inlines
    }
}

fn collect_blocks_from_block<'a, F>(block: &'a Block, predicate: &F, results: &mut Vec<&'a Block>)
where
    F: Fn(&Block) -> bool,
{
    if predicate(block) {
        results.push(block);
    }

    match block {
        Block::BlockQuote(blocks) => {
            for block in blocks {
                collect_blocks_from_block(block, predicate, results);
            }
        }
        Block::List(list) => {
            for item in &list.items {
                for block in &item.blocks {
                    collect_blocks_from_block(block, predicate, results);
                }
            }
        }
        Block::FootnoteDefinition(footnote) => {
            for block in &footnote.blocks {
                collect_blocks_from_block(block, predicate, results);
            }
        }
        Block::GitHubAlert(alert) => {
            for block in &alert.blocks {
                collect_blocks_from_block(block, predicate, results);
            }
        }
        _ => {} // Terminal or inline-containing blocks
    }
}

fn find_first_inline_in_block<'a, F>(block: &'a Block, predicate: &F) -> Option<&'a Inline>
where
    F: Fn(&Inline) -> bool,
{
    match block {
        Block::Paragraph(inlines) => {
            for inline in inlines {
                if let Some(found) = find_first_inline_in_inline(inline, predicate) {
                    return Some(found);
                }
            }
        }
        Block::Heading(heading) => {
            for inline in &heading.content {
                if let Some(found) = find_first_inline_in_inline(inline, predicate) {
                    return Some(found);
                }
            }
        }
        Block::BlockQuote(blocks) => {
            for block in blocks {
                if let Some(found) = find_first_inline_in_block(block, predicate) {
                    return Some(found);
                }
            }
        }
        Block::List(list) => {
            for item in &list.items {
                for block in &item.blocks {
                    if let Some(found) = find_first_inline_in_block(block, predicate) {
                        return Some(found);
                    }
                }
            }
        }
        Block::Table(table) => {
            for row in &table.rows {
                for cell in row {
                    for inline in cell {
                        if let Some(found) = find_first_inline_in_inline(inline, predicate) {
                            return Some(found);
                        }
                    }
                }
            }
        }
        Block::FootnoteDefinition(footnote) => {
            for block in &footnote.blocks {
                if let Some(found) = find_first_inline_in_block(block, predicate) {
                    return Some(found);
                }
            }
        }
        Block::GitHubAlert(alert) => {
            for block in &alert.blocks {
                if let Some(found) = find_first_inline_in_block(block, predicate) {
                    return Some(found);
                }
            }
        }
        Block::Definition(def) => {
            for inline in &def.label {
                if let Some(found) = find_first_inline_in_inline(inline, predicate) {
                    return Some(found);
                }
            }
        }
        _ => {} // Terminal blocks
    }
    None
}

fn find_first_inline_in_inline<'a, F>(inline: &'a Inline, predicate: &F) -> Option<&'a Inline>
where
    F: Fn(&Inline) -> bool,
{
    if predicate(inline) {
        return Some(inline);
    }

    match inline {
        Inline::Emphasis(inlines) | Inline::Strong(inlines) | Inline::Strikethrough(inlines) => {
            for inline in inlines {
                if let Some(found) = find_first_inline_in_inline(inline, predicate) {
                    return Some(found);
                }
            }
        }
        Inline::Link(link) => {
            for inline in &link.children {
                if let Some(found) = find_first_inline_in_inline(inline, predicate) {
                    return Some(found);
                }
            }
        }
        Inline::LinkReference(link_ref) => {
            for inline in &link_ref.label {
                if let Some(found) = find_first_inline_in_inline(inline, predicate) {
                    return Some(found);
                }
            }
            for inline in &link_ref.text {
                if let Some(found) = find_first_inline_in_inline(inline, predicate) {
                    return Some(found);
                }
            }
        }
        _ => {} // Terminal inlines
    }
    None
}

fn find_first_block_in_block<'a, F>(block: &'a Block, predicate: &F) -> Option<&'a Block>
where
    F: Fn(&Block) -> bool,
{
    if predicate(block) {
        return Some(block);
    }

    match block {
        Block::BlockQuote(blocks) => {
            for block in blocks {
                if let Some(found) = find_first_block_in_block(block, predicate) {
                    return Some(found);
                }
            }
        }
        Block::List(list) => {
            for item in &list.items {
                for block in &item.blocks {
                    if let Some(found) = find_first_block_in_block(block, predicate) {
                        return Some(found);
                    }
                }
            }
        }
        Block::FootnoteDefinition(footnote) => {
            for block in &footnote.blocks {
                if let Some(found) = find_first_block_in_block(block, predicate) {
                    return Some(found);
                }
            }
        }
        Block::GitHubAlert(alert) => {
            for block in &alert.blocks {
                if let Some(found) = find_first_block_in_block(block, predicate) {
                    return Some(found);
                }
            }
        }
        _ => {} // Terminal or inline-containing blocks
    }
    None
}
