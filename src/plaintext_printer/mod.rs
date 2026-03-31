//! Plaintext renderer for converting Markdown AST to plain text
//!
//! This module strips all Markdown formatting and produces clean plaintext output.
//!
//! # Basic Usage
//!
//! ```rust
//! use markdown_ppp::ast::*;
//! use markdown_ppp::plaintext_printer::{render_plaintext, config::Config};
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
//! let text = render_plaintext(&doc, config);
//! assert_eq!(text, "Hello World\n\nThis is bold text.");
//! ```

mod block;

/// Configuration options for plaintext rendering.
pub mod config;
mod inline;

#[cfg(test)]
mod tests;

use crate::ast::*;
use config::Config;
use pretty::{Arena, DocBuilder};
use std::collections::HashMap;

/// Internal rendering state for plaintext generation
pub(crate) struct State<'a> {
    arena: Arena<'a>,
    config: Config,
    footnote_index: HashMap<String, usize>,
}

impl State<'_> {
    pub fn new(config: Config, ast: &Document) -> Self {
        let footnote_index = crate::ast::index::get_footnote_indices(ast);
        let arena = Arena::new();
        Self {
            arena,
            config,
            footnote_index,
        }
    }

    pub fn get_footnote_index(&self, label: &str) -> Option<&usize> {
        self.footnote_index.get(label)
    }
}

/// Render a Markdown AST to plain text, stripping all formatting
///
/// # Arguments
///
/// * `ast` - The Markdown document AST to render
/// * `config` - Configuration options controlling the output
///
/// # Returns
///
/// A `String` containing the plain text
///
/// # Examples
///
/// ```rust
/// use markdown_ppp::ast::*;
/// use markdown_ppp::plaintext_printer::{render_plaintext, config::Config};
///
/// let doc = Document {
///     blocks: vec![Block::Paragraph(vec![
///         Inline::Text("Hello ".to_string()),
///         Inline::Strong(vec![Inline::Text("world".to_string())]),
///     ])],
/// };
///
/// let text = render_plaintext(&doc, Config::default());
/// assert_eq!(text, "Hello world");
/// ```
pub fn render_plaintext(ast: &Document, config: Config) -> String {
    let state = State::new(config, ast);
    let doc = ast.to_doc(&state);

    let mut buf = Vec::new();
    doc.render(state.config.width, &mut buf)
        .expect("Vec<u8> write is infallible");
    String::from_utf8(buf).expect("pretty crate always produces valid UTF-8")
}

trait ToDoc<'a> {
    fn to_doc(&self, state: &'a State<'a>) -> DocBuilder<'a, Arena<'a>, ()>;
}

impl<'a> ToDoc<'a> for Document {
    fn to_doc(&self, state: &'a State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        self.blocks.to_doc(state)
    }
}

