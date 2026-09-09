//! Markdown parser for CommonMark + GitHub Flavored Markdown (GFM)
//!
//! This module provides a comprehensive parser for Markdown documents following the
//! CommonMark specification with GitHub Flavored Markdown extensions. The parser
//! converts raw Markdown text into a fully-typed Abstract Syntax Tree (AST).
//!
//! # Features
//!
//! - **CommonMark compliance**: Full support for CommonMark 1.0 specification
//! - **GitHub extensions**: Tables, task lists, strikethrough, autolinks, footnotes, alerts
//! - **Configurable parsing**: Control which elements to parse, skip, or transform
//! - **Custom parsers**: Register custom block and inline element parsers
//! - **Error handling**: Comprehensive error reporting with nom-based parsing
//!
//! # Basic Usage
//!
//! ```rust
//! use markdown_ppp::parser::{parse_markdown, MarkdownParserState};
//!
//! let state = MarkdownParserState::new();
//! let input = "# Hello World\n\nThis is **bold** text.";
//!
//! match parse_markdown(state, input) {
//!     Ok(document) => {
//!         println!("Parsed {} blocks", document.blocks.len());
//!     }
//!     Err(err) => {
//!         eprintln!("Parse error: {:?}", err);
//!     }
//! }
//! ```
//!
//! # Configuration
//!
//! The parser behavior can be extensively customized using configuration:
//!
//! ```rust
//! use markdown_ppp::parser::{MarkdownParserState, config::*};
//!
//! let config = MarkdownParserConfig::default()
//!     .with_block_thematic_break_behavior(ElementBehavior::Skip)
//!     .with_inline_emphasis_behavior(ElementBehavior::Parse);
//!
//! let state = MarkdownParserState::with_config(config);
//! ```

mod blocks;

/// Configuration options for Markdown parsing behavior.
pub mod config;
mod inline;
mod link_util;
mod util;

#[cfg(test)]
mod tests;

use crate::ast::Document;
use crate::parser::config::MarkdownParserConfig;
use nom::{
    branch::alt,
    character::complete::{line_ending, space1},
    combinator::eof,
    multi::many0,
    sequence::terminated,
    Parser,
};
use std::rc::Rc;

/// Parser state containing configuration and shared context
///
/// This structure holds the parser configuration and provides shared state
/// during the parsing process. It's designed to be cloned cheaply using
/// reference counting for the configuration.
///
/// # Examples
///
/// ```rust
/// use markdown_ppp::parser::{MarkdownParserState, config::MarkdownParserConfig};
///
/// // Create with default configuration
/// let state = MarkdownParserState::new();
///
/// // Create with custom configuration
/// let config = MarkdownParserConfig::default();
/// let state = MarkdownParserState::with_config(config);
/// ```
/// Note: This struct is marked `#[non_exhaustive]` to allow adding new fields
/// in future versions without breaking existing code.
#[non_exhaustive]
pub struct MarkdownParserState {
    /// The parser configuration (reference-counted for efficient cloning)
    pub config: Rc<MarkdownParserConfig>,
    /// Whether we are parsing content extracted from a container block (list item, blockquote, etc.)
    /// When true, fenced code blocks should not strip additional indentation from their content.
    /// This field is for internal use only.
    pub(crate) is_nested_block_context: bool,
    /// Current nesting depth (container blocks + inline elements with nested content).
    /// Checked against `config.max_nesting_depth` at every `block`/`inline` entry.
    pub(crate) depth: usize,
    /// Nesting depth of link labels (`[a [b [c]]]`). Tracked separately because a
    /// shortcut/collapsed `LinkReference` stores the label content twice, so the AST
    /// doubles at every level; see `link_util::MAX_LINK_LABEL_DEPTH`.
    pub(crate) link_label_depth: usize,
    /// Delimiter index of the slice currently parsed by `inline_many0`/`inline_many1`
    /// with this state. Nested inline content is parsed with a deeper state and gets
    /// its own index.
    pub(crate) inline_index: std::cell::RefCell<Option<Rc<inline::index::InlineIndex>>>,
}

impl MarkdownParserState {
    /// Create a new parser state with default configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use markdown_ppp::parser::MarkdownParserState;
    ///
    /// let state = MarkdownParserState::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new parser state with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The parser configuration to use
    ///
    /// # Examples
    ///
    /// ```rust
    /// use markdown_ppp::parser::{MarkdownParserState, config::MarkdownParserConfig};
    ///
    /// let config = MarkdownParserConfig::default();
    /// let state = MarkdownParserState::with_config(config);
    /// ```
    pub fn with_config(config: MarkdownParserConfig) -> Self {
        Self {
            config: Rc::new(config),
            is_nested_block_context: false,
            depth: 0,
            link_label_depth: 0,
            inline_index: Default::default(),
        }
    }

    /// Create a nested parser state for parsing content extracted from container blocks
    ///
    /// This method creates a new state that shares the same configuration, marks
    /// the parsing context as nested and increments the nesting depth. The flag prevents
    /// double-stripping of indentation when parsing fenced code blocks inside list items,
    /// blockquotes, etc.
    pub(crate) fn nested(&self) -> Self {
        Self {
            config: self.config.clone(),
            is_nested_block_context: true,
            depth: self.depth + 1,
            link_label_depth: self.link_label_depth,
            inline_index: Default::default(),
        }
    }

    /// Create a state one nesting level deeper, for parsing the content of inline
    /// elements (emphasis, strikethrough, ...). Does not touch the block context flag.
    pub(crate) fn deeper(&self) -> Self {
        Self {
            config: self.config.clone(),
            is_nested_block_context: self.is_nested_block_context,
            depth: self.depth + 1,
            link_label_depth: self.link_label_depth,
            inline_index: Default::default(),
        }
    }

    /// Like [`Self::deeper`], additionally counting one level of link label nesting.
    pub(crate) fn deeper_link_label(&self) -> Self {
        Self {
            link_label_depth: self.link_label_depth + 1,
            ..self.deeper()
        }
    }

    /// Fail with an unrecoverable `TooLarge` error when the nesting depth exceeds
    /// `config.max_nesting_depth`. `Failure` (not `Error`) is used so that `alt`,
    /// `many*`, `not` and `opt` propagate it instead of trying alternatives.
    pub(crate) fn check_depth<'a>(&self, input: &'a str) -> nom::IResult<&'a str, ()> {
        if self.depth > self.config.max_nesting_depth {
            Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::TooLarge,
            )))
        } else {
            Ok((input, ()))
        }
    }
}

impl Default for MarkdownParserState {
    fn default() -> Self {
        Self::with_config(MarkdownParserConfig::default())
    }
}

/// Parse a Markdown string into an Abstract Syntax Tree (AST)
///
/// This is the main entry point for parsing Markdown text. It processes the input
/// according to the CommonMark specification with GitHub Flavored Markdown extensions,
/// returning a fully-typed AST that can be manipulated, analyzed, or rendered.
///
/// # Arguments
///
/// * `state` - Parser state containing configuration options
/// * `input` - The Markdown text to parse
///
/// # Returns
///
/// Returns a `Result` containing either:
/// - `Ok(Document)` - Successfully parsed AST document
/// - `Err(nom::Err)` - Parse error with position and context information
///
/// # Examples
///
/// Basic parsing:
/// ```rust
/// use markdown_ppp::parser::{parse_markdown, MarkdownParserState};
///
/// let state = MarkdownParserState::new();
/// let result = parse_markdown(state, "# Hello\n\nWorld!");
///
/// match result {
///     Ok(doc) => println!("Parsed {} blocks", doc.blocks.len()),
///     Err(e) => eprintln!("Parse error: {:?}", e),
/// }
/// ```
///
/// With custom configuration:
/// ```rust
/// use markdown_ppp::parser::{parse_markdown, MarkdownParserState};
/// use markdown_ppp::parser::config::*;
///
/// let config = MarkdownParserConfig::default()
///     .with_block_thematic_break_behavior(ElementBehavior::Skip);
/// let state = MarkdownParserState::with_config(config);
///
/// let doc = parse_markdown(state, "---\n\nContent").unwrap();
/// ```
///
/// # Errors
///
/// Returns a parse error if the input contains invalid Markdown syntax
/// that cannot be recovered from. Most malformed Markdown is handled
/// gracefully according to CommonMark's error handling rules.
///
/// Returns `nom::Err::Failure` with [`nom::error::ErrorKind::TooLarge`] when the
/// nesting depth of the document exceeds
/// [`MarkdownParserConfig::with_max_nesting_depth`]. This protects against
/// adversarial input such as thousands of nested `>` markers.
pub fn parse_markdown(
    state: MarkdownParserState,
    input: &str,
) -> Result<Document, nom::Err<nom::error::Error<&str>>> {
    let empty_lines = many0(alt((space1, line_ending)));
    let mut parser = terminated(
        many0(crate::parser::blocks::block(Rc::new(state))),
        (empty_lines, eof),
    );
    let (_, blocks) = parser.parse(input)?;

    let blocks = blocks.into_iter().flatten().collect();

    Ok(Document { blocks })
}
