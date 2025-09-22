//! Markdown pretty-printer for formatting AST back to Markdown
//!
//! This module provides functionality to render a Markdown Abstract Syntax Tree (AST)
//! back to formatted Markdown text. The printer supports configurable formatting
//! options and produces clean, readable Markdown output.
//!
//! # Features
//!
//! - **Full AST support**: All CommonMark + GFM elements are supported
//! - **Configurable formatting**: Control line width, indentation, and spacing
//! - **Pretty-printing**: Intelligent line wrapping and formatting
//! - **Round-trip capability**: Parse → Render → Parse produces equivalent AST
//! - **GitHub extensions**: Tables, task lists, alerts, footnotes, strikethrough
//!
//! # Basic Usage
//!
//! ```rust
//! use markdown_ppp::ast::*;
//! use markdown_ppp::printer::{render_markdown, config::Config};
//!
//! let doc = Document {
//!     blocks: vec![
//!         Block::Heading(Heading {
//!             kind: HeadingKind::Atx(1),
//!             content: vec![Inline::Text("Hello World".to_string())],
//!         }),
//!         Block::Paragraph(vec![
//!             Inline::Text("This is ".to_string()),
//!             Inline::Strong(vec![Inline::Text("formatted".to_string())]),
//!             Inline::Text(" text.".to_string()),
//!         ]),
//!     ],
//! };
//!
//! let config = Config::default();
//! let markdown = render_markdown(&doc, config);
//! println!("{}", markdown);
//! ```
//!
//! # Configuration
//!
//! Customize the output format using configuration:
//!
//! ```rust
//! use markdown_ppp::printer::{render_markdown, config::Config};
//! use markdown_ppp::ast::Document;
//!
//! let config = Config::default().with_width(120);
//! let markdown = render_markdown(&Document::default(), config);
//! ```

mod block;
mod blockquote;

/// Configuration options for Markdown pretty-printing.
pub mod config;
mod github_alert;
mod heading;
mod inline;
mod list;
mod table;
mod tests;

use crate::ast::*;
use pretty::{Arena, DocBuilder};
use std::rc::Rc;

/// Render a Markdown AST back to formatted Markdown text
///
/// This function takes a parsed Markdown document (AST) and renders it back
/// to clean, well-formatted Markdown text. The output follows consistent
/// formatting rules and can be customized via configuration options.
///
/// # Arguments
///
/// * `ast` - The Markdown document AST to render
/// * `config` - Configuration options controlling the output format
///
/// # Returns
///
/// A `String` containing the formatted Markdown text
///
/// # Examples
///
/// Basic rendering:
/// ```rust
/// use markdown_ppp::ast::*;
/// use markdown_ppp::printer::{render_markdown, config::Config};
///
/// let doc = Document {
///     blocks: vec![
///         Block::Paragraph(vec![
///             Inline::Text("Hello ".to_string()),
///             Inline::Strong(vec![Inline::Text("world".to_string())]),
///         ]),
///     ],
/// };
///
/// let config = Config::default();
/// let markdown = render_markdown(&doc, config);
/// assert!(markdown.contains("**world**"));
/// ```
///
/// With custom width:
/// ```rust
/// use markdown_ppp::ast::Document;
/// use markdown_ppp::printer::{render_markdown, config::Config};
///
/// let doc = Document { blocks: vec![] };
/// let config = Config::default().with_width(60);
/// let markdown = render_markdown(&doc, config);
/// ```
///
/// # Round-trip Guarantee
///
/// For most valid Markdown documents, the following property holds:
/// ```text
/// parse(render(parse(input))) ≈ parse(input)
/// ```
/// Where ≈ means semantically equivalent AST structures.
pub fn render_markdown(ast: &Document, config: crate::printer::config::Config) -> String {
    let config = Rc::new(config);
    let arena = Arena::new();
    let doc = ast.to_doc(config.clone(), &arena);

    let mut buf = Vec::new();
    doc.render(config.width, &mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

trait ToDoc<'a> {
    fn to_doc(
        &self,
        config: Rc<crate::printer::config::Config>,
        arena: &'a Arena<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()>;
}

impl<'a> ToDoc<'a> for Document {
    fn to_doc(
        &self,
        config: Rc<crate::printer::config::Config>,
        arena: &'a Arena<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        self.blocks.to_doc(config, arena)
    }
}
