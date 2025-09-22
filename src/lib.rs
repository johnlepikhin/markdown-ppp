//! # markdown-ppp
//!
//! Feature-rich Markdown Parsing and Pretty-Printing library.
//!
//! This crate provides comprehensive support for parsing CommonMark + GitHub Flavored Markdown (GFM)
//! and converting it to various output formats including Markdown, HTML, and LaTeX.

/// Fully-typed Abstract Syntax Tree (AST) for CommonMark + GitHub Flavored Markdown.
pub mod ast;

/// Markdown parser for CommonMark + GFM.
#[cfg(feature = "parser")]
pub mod parser;

/// Markdown pretty-printer for formatting AST back to Markdown.
#[cfg(feature = "printer")]
pub mod printer;

/// HTML renderer for converting Markdown AST to HTML.
#[cfg(feature = "html-printer")]
pub mod html_printer;

/// LaTeX renderer for converting Markdown AST to LaTeX.
#[cfg(feature = "latex-printer")]
pub mod latex_printer;

/// AST transformation utilities for manipulating parsed Markdown.
#[cfg(feature = "ast-transform")]
pub mod ast_transform;
