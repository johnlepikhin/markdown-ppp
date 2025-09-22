//! HTML renderer for converting Markdown AST to HTML
//!
//! This module provides functionality to render a Markdown Abstract Syntax Tree (AST)
//! into clean, semantic HTML. The renderer supports all CommonMark + GitHub Flavored
//! Markdown features and produces standards-compliant HTML output.
//!
//! # Features
//!
//! - **Full AST coverage**: All CommonMark + GFM elements are supported
//! - **Semantic HTML**: Produces clean, accessible HTML with proper structure
//! - **GitHub extensions**: Tables, task lists, alerts, footnotes, strikethrough
//! - **Link resolution**: Automatic resolution of reference links and footnotes
//! - **Configurable output**: Control HTML formatting and features
//! - **Security**: Proper escaping of HTML entities and attributes
//!
//! # Basic Usage
//!
//! ```rust
//! use markdown_ppp::ast::*;
//! use markdown_ppp::html_printer::{render_html, config::Config};
//!
//! let doc = Document {
//!     blocks: vec![
//!         Block::Heading(Heading {
//!             kind: HeadingKind::Atx(1),
//!             content: vec![Inline::Text("Hello World".to_string())],
//!         }),
//!         Block::Paragraph(vec![
//!             Inline::Text("This is ".to_string()),
//!             Inline::Strong(vec![Inline::Text("bold".to_string())]),
//!             Inline::Text(" text.".to_string()),
//!         ]),
//!     ],
//! };
//!
//! let config = Config::default();
//! let html = render_html(&doc, config);
//! assert!(html.contains("<h1>Hello World</h1>"));
//! assert!(html.contains("<strong>bold</strong>"));
//! ```
//!
//! # Configuration
//!
//! Customize the HTML output using [`Config`]:
//!
//! ```rust
//! use markdown_ppp::html_printer::{render_html, config::Config};
//! use markdown_ppp::ast::Document;
//!
//! let config = Config::default();
//! let html = render_html(&Document::default(), config);
//! ```

mod block;
pub mod config;
mod github_alert;
mod index;
mod inline;
mod tests;
mod util;

use crate::ast::*;
use pretty::{Arena, DocBuilder};
use std::{collections::HashMap, rc::Rc};

/// Internal rendering state for HTML generation
///
/// This structure holds the shared state needed during HTML rendering,
/// including configuration, footnote indexing, and link definition resolution.
/// It's used internally by the rendering process and is not part of the public API.
pub(crate) struct State<'a> {
    /// Pretty-printing arena for efficient document building
    arena: Arena<'a>,
    /// HTML rendering configuration
    config: crate::html_printer::config::Config,
    /// Mapping of footnote labels to their indices in the footnote list
    footnote_index: HashMap<String, usize>,
    /// Mapping of link labels to their definitions for reference link resolution
    link_definitions: HashMap<Vec<Inline>, LinkDefinition>,
}

impl State<'_> {
    pub fn new(config: crate::html_printer::config::Config, ast: &Document) -> Self {
        let (footnote_index, link_definitions) = crate::html_printer::index::get_indicies(ast);
        let arena = Arena::new();
        Self {
            arena,
            config,
            footnote_index,
            link_definitions,
        }
    }

    pub fn get_footnote_index(&self, label: &str) -> Option<&usize> {
        self.footnote_index.get(label)
    }

    pub fn get_link_definition(&self, label: &Vec<Inline>) -> Option<&LinkDefinition> {
        self.link_definitions.get(label)
    }
}

/// Render a Markdown AST to semantic HTML
///
/// This function takes a parsed Markdown document (AST) and converts it to
/// clean, standards-compliant HTML. The output includes proper semantic
/// markup, accessibility features, and support for all CommonMark + GFM
/// elements.
///
/// # Arguments
///
/// * `ast` - The Markdown document AST to render
/// * `config` - Configuration options controlling the HTML output
///
/// # Returns
///
/// A `String` containing the generated HTML
///
/// # Examples
///
/// Basic HTML rendering:
/// ```rust
/// use markdown_ppp::ast::*;
/// use markdown_ppp::html_printer::{render_html, config::Config};
///
/// let doc = Document {
///     blocks: vec![
///         Block::Heading(Heading {
///             kind: HeadingKind::Atx(1),
///             content: vec![Inline::Text("Title".to_string())],
///         }),
///         Block::Paragraph(vec![
///             Inline::Text("Text with ".to_string()),
///             Inline::Strong(vec![Inline::Text("emphasis".to_string())]),
///         ]),
///     ],
/// };
///
/// let config = Config::default();
/// let html = render_html(&doc, config);
///
/// assert!(html.contains("<h1>Title</h1>"));
/// assert!(html.contains("emphasis"));
/// ```
///
/// # HTML Features
///
/// The renderer produces:
/// - Semantic HTML5 elements (`<article>`, `<section>`, `<aside>`, etc.)
/// - Proper heading hierarchy (`<h1>` through `<h6>`)
/// - Accessible tables with `<thead>`, `<tbody>`, and proper scoping
/// - Code syntax highlighting preparation with language classes
/// - Task list checkboxes with proper `disabled` attributes
/// - Footnote links with proper `aria-describedby` attributes
/// - GitHub Alert styling with appropriate CSS classes
///
/// # Security
///
/// All user content is properly escaped to prevent XSS attacks.
/// HTML content in the AST is preserved as-is (assumed to be trusted).
pub fn render_html(ast: &Document, config: crate::html_printer::config::Config) -> String {
    let state = Rc::new(State::new(config, ast));
    let doc = ast.to_doc(&state);

    let mut buf = Vec::new();
    doc.render(state.config.width, &mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

trait ToDoc<'a> {
    fn to_doc(&self, state: &'a State<'a>) -> DocBuilder<'a, Arena<'a>, ()>;
}

impl<'a> ToDoc<'a> for Document {
    fn to_doc(&self, state: &'a State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        self.blocks.to_doc(state)
    }
}
