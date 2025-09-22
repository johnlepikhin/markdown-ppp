//! AST transformation and manipulation utilities
//!
//! This module provides a comprehensive set of tools for transforming and querying Markdown AST:
//! - Visitor pattern for read-only traversal
//! - Transformer pattern for AST modifications
//! - Query API for finding elements by conditions
//! - Convenience methods for common transformations
//! - Pipeline builder for composing complex transformations
//!
//! # Examples
//!
//! ```rust
//! use markdown_ppp::ast::*;
//! use markdown_ppp::ast_transform::*;
//!
//! // Create a simple document
//! let doc = Document {
//!     blocks: vec![
//!         Block::Paragraph(vec![
//!             Inline::Text("hello world".to_string()),
//!         ]),
//!     ],
//! };
//!
//! // Transform all text to uppercase
//! let doc = doc.transform_text(|text| text.to_uppercase());
//!
//! // Find all autolinks (in a document that has them)
//! let autolinks = doc.find_all_inlines(|inline| {
//!     matches!(inline, Inline::Autolink(_))
//! });
//!
//! // Complex pipeline
//! let result = TransformPipeline::new()
//!     .transform_text(|s| s.trim().to_string())
//!     .transform_image_urls(|url| format!("https://cdn.example.com{}", url))
//!     .apply(doc);
//! ```

pub mod convenience;
pub mod pipeline;
pub mod query;
pub mod transformer;
pub mod visitor;

#[cfg(test)]
mod tests;

pub use convenience::*;
pub use pipeline::*;
pub use query::*;
pub use transformer::*;
pub use visitor::*;
