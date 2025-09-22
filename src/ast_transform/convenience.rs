//! Convenience methods for common AST transformations
//!
//! This module provides high-level convenience methods for common transformation
//! tasks. These methods are implemented as extension traits on AST types, making
//! them easy to use in a fluent style.
//!
//! # Example
//!
//! ```rust
//! use markdown_ppp::ast::*;
//! use markdown_ppp::ast_transform::{Transform, FilterTransform};
//!
//! let doc = Document {
//!     blocks: vec![Block::Paragraph(vec![Inline::Text("  hello  ".to_string())])],
//! };
//!
//! let result = doc
//!     .transform_text(|text| text.trim().to_string())
//!     .normalize_whitespace()
//!     .remove_empty_text();
//! ```

use super::transformer::Transformer;
use crate::ast::*;

/// High-level transformation methods for common use cases
pub trait Transform {
    /// Transform all text elements with a function
    ///
    /// # Example
    ///
    /// ```rust
    /// use markdown_ppp::ast::*;
    /// use markdown_ppp::ast_transform::Transform;
    ///
    /// let doc = Document {
    ///     blocks: vec![Block::Paragraph(vec![Inline::Text("hello".to_string())])],
    /// };
    /// let result = doc.transform_text(|text| text.to_uppercase());
    /// ```
    fn transform_text<F>(self, f: F) -> Self
    where
        F: Fn(String) -> String;

    /// Transform all image URLs with a function
    ///
    /// # Example
    ///
    /// ```rust
    /// use markdown_ppp::ast::*;
    /// use markdown_ppp::ast_transform::Transform;
    ///
    /// let doc = Document {
    ///     blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
    ///         destination: "/image.jpg".to_string(),
    ///         title: None,
    ///         alt: "test".to_string(),
    ///     })])],
    /// };
    /// let result = doc.transform_image_urls(|url| {
    ///     format!("https://cdn.example.com{}", url)
    /// });
    /// ```
    fn transform_image_urls<F>(self, f: F) -> Self
    where
        F: Fn(String) -> String;

    /// Transform all link URLs with a function
    ///
    /// # Example
    ///
    /// ```rust
    /// use markdown_ppp::ast::*;
    /// use markdown_ppp::ast_transform::Transform;
    ///
    /// let doc = Document {
    ///     blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
    ///         destination: "http://example.com".to_string(),
    ///         title: None,
    ///         children: vec![Inline::Text("link".to_string())],
    ///     })])],
    /// };
    /// let result = doc.transform_link_urls(|url| {
    ///     url.replace("http://", "https://")
    /// });
    /// ```
    fn transform_link_urls<F>(self, f: F) -> Self
    where
        F: Fn(String) -> String;

    /// Transform all autolink URLs with a function
    fn transform_autolink_urls<F>(self, f: F) -> Self
    where
        F: Fn(String) -> String;

    /// Transform all code spans with a function
    fn transform_code<F>(self, f: F) -> Self
    where
        F: Fn(String) -> String;

    /// Transform all HTML content with a function
    fn transform_html<F>(self, f: F) -> Self
    where
        F: Fn(String) -> String;

    /// Apply a custom transformer
    fn transform_with<T: Transformer>(self, transformer: T) -> Self;

    /// Transform conditionally based on a predicate
    fn transform_if<F>(self, condition: bool, transform: F) -> Self
    where
        F: FnOnce(Self) -> Self,
        Self: Sized;
}

impl Transform for Document {
    fn transform_text<F>(self, f: F) -> Self
    where
        F: Fn(String) -> String,
    {
        let mut transformer = TextTransformer::new(f);
        transformer.transform_document(self)
    }

    fn transform_image_urls<F>(self, f: F) -> Self
    where
        F: Fn(String) -> String,
    {
        let mut transformer = ImageUrlTransformer::new(f);
        transformer.transform_document(self)
    }

    fn transform_link_urls<F>(self, f: F) -> Self
    where
        F: Fn(String) -> String,
    {
        let mut transformer = LinkUrlTransformer::new(f);
        transformer.transform_document(self)
    }

    fn transform_autolink_urls<F>(self, f: F) -> Self
    where
        F: Fn(String) -> String,
    {
        let mut transformer = AutolinkTransformer::new(f);
        transformer.transform_document(self)
    }

    fn transform_code<F>(self, f: F) -> Self
    where
        F: Fn(String) -> String,
    {
        let mut transformer = CodeTransformer::new(f);
        transformer.transform_document(self)
    }

    fn transform_html<F>(self, f: F) -> Self
    where
        F: Fn(String) -> String,
    {
        let mut transformer = HtmlTransformer::new(f);
        transformer.transform_document(self)
    }

    fn transform_with<T: Transformer>(self, mut transformer: T) -> Self {
        transformer.transform_document(self)
    }

    fn transform_if<F>(self, condition: bool, transform: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        if condition {
            transform(self)
        } else {
            self
        }
    }
}

// Internal transformer implementations

struct TextTransformer<F> {
    func: F,
}

impl<F> TextTransformer<F>
where
    F: Fn(String) -> String,
{
    fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F> Transformer for TextTransformer<F>
where
    F: Fn(String) -> String,
{
    fn transform_inline(&mut self, inline: Inline) -> Inline {
        match inline {
            Inline::Text(text) => Inline::Text((self.func)(text)),
            other => self.walk_transform_inline(other),
        }
    }
}

struct ImageUrlTransformer<F> {
    func: F,
}

impl<F> ImageUrlTransformer<F>
where
    F: Fn(String) -> String,
{
    fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F> Transformer for ImageUrlTransformer<F>
where
    F: Fn(String) -> String,
{
    fn transform_inline(&mut self, inline: Inline) -> Inline {
        match inline {
            Inline::Image(mut image) => {
                image.destination = (self.func)(image.destination);
                Inline::Image(image)
            }
            other => self.walk_transform_inline(other),
        }
    }
}

struct LinkUrlTransformer<F> {
    func: F,
}

impl<F> LinkUrlTransformer<F>
where
    F: Fn(String) -> String,
{
    fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F> Transformer for LinkUrlTransformer<F>
where
    F: Fn(String) -> String,
{
    fn transform_inline(&mut self, inline: Inline) -> Inline {
        match inline {
            Inline::Link(mut link) => {
                link.destination = (self.func)(link.destination);
                link.children = link
                    .children
                    .into_iter()
                    .map(|child| self.transform_inline(child))
                    .collect();
                Inline::Link(link)
            }
            other => self.walk_transform_inline(other),
        }
    }
}

struct AutolinkTransformer<F> {
    func: F,
}

impl<F> AutolinkTransformer<F>
where
    F: Fn(String) -> String,
{
    fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F> Transformer for AutolinkTransformer<F>
where
    F: Fn(String) -> String,
{
    fn transform_inline(&mut self, inline: Inline) -> Inline {
        match inline {
            Inline::Autolink(url) => Inline::Autolink((self.func)(url)),
            other => self.walk_transform_inline(other),
        }
    }
}

struct CodeTransformer<F> {
    func: F,
}

impl<F> CodeTransformer<F>
where
    F: Fn(String) -> String,
{
    fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F> Transformer for CodeTransformer<F>
where
    F: Fn(String) -> String,
{
    fn transform_inline(&mut self, inline: Inline) -> Inline {
        match inline {
            Inline::Code(code) => Inline::Code((self.func)(code)),
            other => self.walk_transform_inline(other),
        }
    }
}

struct HtmlTransformer<F> {
    func: F,
}

impl<F> HtmlTransformer<F>
where
    F: Fn(String) -> String,
{
    fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F> Transformer for HtmlTransformer<F>
where
    F: Fn(String) -> String,
{
    fn transform_inline(&mut self, inline: Inline) -> Inline {
        match inline {
            Inline::Html(html) => Inline::Html((self.func)(html)),
            other => self.walk_transform_inline(other),
        }
    }

    fn transform_block(&mut self, block: Block) -> Block {
        match block {
            Block::HtmlBlock(html) => Block::HtmlBlock((self.func)(html)),
            other => self.walk_transform_block(other),
        }
    }
}

/// Additional utility methods for filtering and common operations
pub trait FilterTransform {
    /// Remove empty paragraphs
    fn remove_empty_paragraphs(self) -> Self;

    /// Remove empty text elements
    fn remove_empty_text(self) -> Self;

    /// Normalize whitespace in text elements
    fn normalize_whitespace(self) -> Self;

    /// Remove specific block types
    fn filter_blocks<F>(self, predicate: F) -> Self
    where
        F: Fn(&Block) -> bool;
}

impl FilterTransform for Document {
    fn remove_empty_paragraphs(mut self) -> Self {
        self.blocks
            .retain(|block| !matches!(block, Block::Paragraph(inlines) if inlines.is_empty()));
        self
    }

    fn remove_empty_text(self) -> Self {
        let mut transformer = EmptyTextRemover;
        transformer.transform_document(self)
    }

    fn normalize_whitespace(self) -> Self {
        self.transform_text(|text| text.split_whitespace().collect::<Vec<_>>().join(" "))
    }

    fn filter_blocks<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&Block) -> bool,
    {
        self.blocks.retain(|block| predicate(block));
        self
    }
}

struct EmptyTextRemover;

impl Transformer for EmptyTextRemover {
    fn transform_inline(&mut self, inline: Inline) -> Inline {
        match inline {
            Inline::Text(text) if text.trim().is_empty() => Inline::Empty,
            other => self.walk_transform_inline(other),
        }
    }
}
