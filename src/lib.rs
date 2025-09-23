//! # markdown-ppp
//!
//! Feature-rich Markdown Parsing and Pretty-Printing library.
//!
//! This crate provides comprehensive support for parsing CommonMark + GitHub Flavored Markdown (GFM)
//! and converting it to various output formats including Markdown, HTML, and LaTeX.

/// Fully-typed Abstract Syntax Tree (AST) for CommonMark + GitHub Flavored Markdown.
///
/// The AST module provides a generic AST structure. See [`ast::generic`] for more details.
pub mod ast;

/// Specialized AST types for element identification.
///
/// This module provides pre-defined specialized versions of the generic AST
/// for element identification scenarios.
///
/// # Available modules
///
/// - `element_id` - Element ID support and related functionality
/// - `type_aliases` - Convenient type aliases for specialized AST types
/// - `utilities` - Helper functions and utilities
#[cfg(feature = "ast-specialized")]
pub mod ast_specialized;

/// Markdown parser for CommonMark + GFM.
///
/// Parse Markdown text into an AST using [`parse_markdown`](parser::parse_markdown).
#[cfg(feature = "parser")]
pub mod parser;

/// Markdown pretty-printer for formatting AST back to Markdown.
///
/// Render AST to Markdown using [`render_markdown`](printer::render_markdown).
#[cfg(feature = "printer")]
pub mod printer;

/// HTML renderer for converting Markdown AST to HTML.
///
/// Render AST to HTML using [`render_html`](html_printer::render_html).
#[cfg(feature = "html-printer")]
pub mod html_printer;

/// LaTeX renderer for converting Markdown AST to LaTeX.
///
/// Render AST to LaTeX using [`render_latex`](latex_printer::render_latex).
#[cfg(feature = "latex-printer")]
pub mod latex_printer;

/// AST transformation utilities for manipulating parsed Markdown.
#[cfg(feature = "ast-transform")]
pub mod ast_transform;
