//! Pipeline builder for composing transformations
//!
//! This module provides the TransformPipeline builder for chaining multiple
//! transformations together. Pipelines support conditional logic, custom
//! transformations, and functional composition patterns.
//!
//! # Example
//!
//! ```rust
//! use markdown_ppp::ast::*;
//! use markdown_ppp::ast_transform::TransformPipeline;
//!
//! let doc = Document {
//!     blocks: vec![Block::Paragraph(vec![Inline::Text("  hello  ".to_string())])],
//! };
//!
//! let result = TransformPipeline::new()
//!     .transform_text(|s| s.trim().to_string())
//!     .normalize_whitespace()
//!     .when(true, |pipeline| {
//!         pipeline.transform_text(|s| s.to_uppercase())
//!     })
//!     .apply(doc);
//! ```

use super::transformer::Transformer;
use crate::ast::*;

/// Builder for creating transformation pipelines
///
/// Allows chaining multiple transformations together with conditional logic.
///
/// # Example
///
/// ```rust
/// use markdown_ppp::ast::*;
/// use markdown_ppp::ast_transform::TransformPipeline;
///
/// let doc = Document {
///     blocks: vec![Block::Paragraph(vec![Inline::Text("  hello  ".to_string())])],
/// };
///
/// let result = TransformPipeline::new()
///     .transform_text(|s| s.trim().to_string())
///     .transform_image_urls(|url| format!("https://cdn.example.com{}", url))
///     .apply(doc);
/// ```
pub struct TransformPipeline {
    steps: Vec<Box<dyn FnOnce(Document) -> Document>>,
}

impl TransformPipeline {
    /// Create a new empty pipeline
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    /// Transform all text elements
    pub fn transform_text<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> String + 'static,
    {
        use crate::ast_transform::Transform;
        self.steps.push(Box::new(move |doc| doc.transform_text(f)));
        self
    }

    /// Transform all image URLs
    pub fn transform_image_urls<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> String + 'static,
    {
        use crate::ast_transform::Transform;
        self.steps
            .push(Box::new(move |doc| doc.transform_image_urls(f)));
        self
    }

    /// Transform all link URLs
    pub fn transform_link_urls<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> String + 'static,
    {
        use crate::ast_transform::Transform;
        self.steps
            .push(Box::new(move |doc| doc.transform_link_urls(f)));
        self
    }

    /// Transform all autolink URLs
    pub fn transform_autolink_urls<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> String + 'static,
    {
        use crate::ast_transform::Transform;
        self.steps
            .push(Box::new(move |doc| doc.transform_autolink_urls(f)));
        self
    }

    /// Transform all code spans
    pub fn transform_code<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> String + 'static,
    {
        use crate::ast_transform::Transform;
        self.steps.push(Box::new(move |doc| doc.transform_code(f)));
        self
    }

    /// Transform all HTML content
    pub fn transform_html<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> String + 'static,
    {
        use crate::ast_transform::Transform;
        self.steps.push(Box::new(move |doc| doc.transform_html(f)));
        self
    }

    /// Apply a custom transformer
    pub fn transform_with<T: Transformer + 'static>(mut self, transformer: T) -> Self {
        use crate::ast_transform::Transform;
        self.steps
            .push(Box::new(move |doc| doc.transform_with(transformer)));
        self
    }

    /// Add a custom transformation function
    pub fn custom<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Document) -> Document + 'static,
    {
        self.steps.push(Box::new(f));
        self
    }

    /// Conditionally apply a sub-pipeline
    pub fn when<F>(mut self, condition: bool, builder: F) -> Self
    where
        F: FnOnce(TransformPipeline) -> TransformPipeline,
    {
        if condition {
            let sub_pipeline = builder(TransformPipeline::new());
            self.steps
                .push(Box::new(move |doc| sub_pipeline.apply(doc)));
        }
        self
    }

    /// Apply transformations only if the document matches a predicate
    pub fn when_doc<P, F>(mut self, predicate: P, builder: F) -> Self
    where
        P: Fn(&Document) -> bool + 'static,
        F: FnOnce(TransformPipeline) -> TransformPipeline + 'static,
    {
        self.steps.push(Box::new(move |doc| {
            if predicate(&doc) {
                let sub_pipeline = builder(TransformPipeline::new());
                sub_pipeline.apply(doc)
            } else {
                doc
            }
        }));
        self
    }

    /// Remove empty paragraphs
    pub fn remove_empty_paragraphs(mut self) -> Self {
        use crate::ast_transform::FilterTransform;
        self.steps
            .push(Box::new(|doc| doc.remove_empty_paragraphs()));
        self
    }

    /// Remove empty text elements
    pub fn remove_empty_text(mut self) -> Self {
        use crate::ast_transform::FilterTransform;
        self.steps.push(Box::new(|doc| doc.remove_empty_text()));
        self
    }

    /// Normalize whitespace
    pub fn normalize_whitespace(mut self) -> Self {
        use crate::ast_transform::FilterTransform;
        self.steps.push(Box::new(|doc| doc.normalize_whitespace()));
        self
    }

    /// Filter blocks by predicate
    pub fn filter_blocks<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&Block) -> bool + 'static,
    {
        use crate::ast_transform::FilterTransform;
        self.steps
            .push(Box::new(move |doc| doc.filter_blocks(predicate)));
        self
    }

    /// Apply all transformations in the pipeline
    pub fn apply(self, mut doc: Document) -> Document {
        for step in self.steps {
            doc = step(doc);
        }
        doc
    }
}

impl Default for TransformPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Functional composition helpers
pub trait PipeExt {
    /// Apply a function to self (functional pipe operator)
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
        Self: Sized,
    {
        f(self)
    }
}

impl PipeExt for Document {}
