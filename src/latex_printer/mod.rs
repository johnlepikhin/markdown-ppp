//! LaTeX printer for Markdown AST
//!
//! This module provides functionality to render a Markdown Abstract Syntax Tree (AST)
//! into LaTeX format. The printer supports full CommonMark + GitHub Flavored Markdown
//! features and offers configurable output styles.
//!
//! # Features
//!
//! - **Full AST coverage**: All block and inline elements from CommonMark + GFM
//! - **Configurable table styles**: `tabular`, `longtabu`, `booktabs`
//! - **Configurable code styles**: `verbatim`, `listings`, `minted`
//! - **Proper LaTeX escaping**: All special characters are properly escaped
//! - **GitHub extensions**: Alerts, task lists, footnotes, strikethrough
//! - **Width control**: Configurable line width for pretty-printing
//!
//! # Basic Usage
//!
//! ```rust
//! use markdown_ppp::ast::*;
//! use markdown_ppp::latex_printer::{render_latex, config::Config};
//!
//! let doc = Document {
//!     blocks: vec![
//!         Block::Heading(Heading {
//!             kind: HeadingKind::Atx(1),
//!             content: vec![Inline::Text("Hello LaTeX".to_string())],
//!         }),
//!         Block::Paragraph(vec![
//!             Inline::Text("This is ".to_string()),
//!             Inline::Strong(vec![Inline::Text("bold".to_string())]),
//!             Inline::Text(" and ".to_string()),
//!             Inline::Emphasis(vec![Inline::Text("italic".to_string())]),
//!             Inline::Text(" text with special chars: $100 & 50%.".to_string()),
//!         ]),
//!     ],
//! };
//!
//! let latex = render_latex(&doc, Config::default());
//! // Produces:
//! // \section{Hello LaTeX}
//! //
//! // This is \textbf{bold} and \textit{italic} text with special chars: \$100 \& 50\%.
//! ```
//!
//! # Advanced Configuration
//!
//! ```rust
//! # use markdown_ppp::ast::*;
//! # use markdown_ppp::latex_printer::{render_latex, config::*};
//! let config = Config::default()
//!     .with_width(120)
//!     .with_table_style(TableStyle::Booktabs)
//!     .with_code_block_style(CodeBlockStyle::Minted);
//!
//! # let doc = Document { blocks: vec![] };
//! let latex = render_latex(&doc, config);
//! ```
//!
//! # LaTeX Element Mappings
//!
//! | Markdown          | LaTeX                                |
//! |-------------------|--------------------------------------|
//! | `# Heading`       | `\section{Heading}`                 |
//! | `**bold**`        | `\textbf{bold}`                     |
//! | `*italic*`        | `\textit{italic}`                   |
//! | `~~strike~~`      | `\sout{strike}`                     |
//! | `` `code` ``      | `\texttt{code}`                     |
//! | `> quote`         | `\begin{quote}...\end{quote}`       |
//! | `- list`          | `\begin{itemize}...\end{itemize}`   |
//! | `1. ordered`      | `\begin{enumerate}...\end{enumerate}` |
//! | `[link](url)`     | `\href{url}{link}`                  |
//! | `![img](url)`     | `\includegraphics{url}`             |
//! | Tables            | `\begin{tabular}...` (configurable) |
//! | Code blocks       | `\begin{verbatim}...` (configurable) |

mod block;
pub mod config;
mod inline;
mod table;
pub mod util;

#[cfg(test)]
mod tests;

use crate::ast::*;
use pretty::{Arena, DocBuilder};
use std::{collections::HashMap, rc::Rc};

/// Internal state for LaTeX rendering
///
/// This structure holds the rendering context including the pretty-printer arena,
/// configuration, and pre-processed indices for footnotes and link definitions.
pub(crate) struct State<'a> {
    arena: Arena<'a>,
    config: crate::latex_printer::config::Config,
    /// Mapping of footnote labels to their indices in the footnote list.
    footnote_index: HashMap<String, usize>,
    /// Mapping of link labels to their definitions.
    link_definitions: HashMap<Vec<Inline>, LinkDefinition>,
}

impl State<'_> {
    /// Create a new rendering state
    ///
    /// This processes the AST to build indices for footnotes and link definitions,
    /// which are needed for proper cross-referencing during rendering.
    pub fn new(config: crate::latex_printer::config::Config, ast: &Document) -> Self {
        let (footnote_index, link_definitions) = get_indices(ast);
        let arena = Arena::new();
        Self {
            arena,
            config,
            footnote_index,
            link_definitions,
        }
    }

    /// Get the numeric index for a footnote label
    ///
    /// Returns `None` if the footnote is not defined in the document.
    pub fn get_footnote_index(&self, label: &str) -> Option<&usize> {
        self.footnote_index.get(label)
    }

    /// Get the link definition for a reference link
    ///
    /// Returns `None` if the link reference is not defined in the document.
    pub fn get_link_definition(&self, label: &Vec<Inline>) -> Option<&LinkDefinition> {
        self.link_definitions.get(label)
    }
}

/// Render the given Markdown AST to LaTeX
///
/// This is the main entry point for LaTeX rendering. It takes a parsed Markdown
/// document and configuration, then produces LaTeX source code.
///
/// # Arguments
///
/// * `ast` - The parsed Markdown document as an AST
/// * `config` - Configuration for rendering (table styles, code styles, width, etc.)
///
/// # Returns
///
/// LaTeX source code as a string. This will be a document fragment suitable
/// for inclusion in a larger LaTeX document, not a complete document with
/// `\documentclass` etc.
///
/// # Examples
///
/// ```rust
/// use markdown_ppp::ast::*;
/// use markdown_ppp::latex_printer::{render_latex, config::Config};
///
/// let doc = Document {
///     blocks: vec![
///         Block::Paragraph(vec![
///             Inline::Text("Visit ".to_string()),
///             Inline::Link(Link {
///                 destination: "https://example.com".to_string(),
///                 title: None,
///                 children: vec![Inline::Text("this link".to_string())],
///             }),
///             Inline::Text(" for more info.".to_string()),
///         ]),
///         Block::List(List {
///             kind: ListKind::Bullet(ListBulletKind::Star),
///             items: vec![ListItem {
///                 task: Some(TaskState::Complete),
///                 blocks: vec![Block::Paragraph(vec![
///                     Inline::Strong(vec![Inline::Text("Bold".to_string())]),
///                     Inline::Text(" item with special chars: $100 & 50%".to_string()),
///                 ])],
///             }],
///         }),
///     ],
/// };
///
/// let latex = render_latex(&doc, Config::default());
/// // Produces LaTeX with proper escaping and formatting:
/// // Visit \href{https://example.com}{this link} for more info.
/// //
/// // \begin{itemize}
/// // \item $\boxtimes$ \textbf{Bold} item with special chars: \$100 \& 50\%
/// // \end{itemize}
/// ```
///
/// # LaTeX Packages Required
///
/// The generated LaTeX may require these packages depending on features used:
///
/// - `hyperref` - for links (`\href`)
/// - `graphicx` - for images (`\includegraphics`)
/// - `ulem` - for strikethrough (`\sout`)
/// - `booktabs` - if using booktabs table style
/// - `longtabu` - if using longtabu table style
/// - `listings` - if using listings code style
/// - `minted` - if using minted code style
pub fn render_latex(ast: &Document, config: crate::latex_printer::config::Config) -> String {
    let state = Rc::new(State::new(config, ast));
    let doc = ast.to_doc(&state);

    let mut buf = Vec::new();
    doc.render(state.config.width, &mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

/// Internal trait for converting AST nodes to pretty-printer documents
///
/// This trait is implemented by all AST node types and provides the core
/// rendering logic for each element type.
trait ToDoc<'a> {
    /// Convert this AST node to a pretty-printer document
    fn to_doc(&self, state: &'a State<'a>) -> DocBuilder<'a, Arena<'a>, ()>;
}

impl<'a> ToDoc<'a> for Document {
    fn to_doc(&self, state: &'a State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        self.blocks.to_doc(state)
    }
}

/// Extract footnote and link definition indices from the document
///
/// This function performs a pre-processing pass over the AST to:
/// 1. Assign numeric indices to footnote definitions (1, 2, 3, ...)
/// 2. Collect link definitions for reference link resolution
///
/// Returns a tuple of (footnote_index, link_definitions) where:
/// - footnote_index maps footnote labels to their numeric indices
/// - link_definitions maps link labels to their full definitions
fn get_indices(ast: &Document) -> (HashMap<String, usize>, HashMap<Vec<Inline>, LinkDefinition>) {
    let mut footnote_index = HashMap::new();
    let mut link_definitions = HashMap::new();
    let mut footnote_counter = 1;

    fn process_blocks(
        blocks: &[Block],
        footnote_index: &mut HashMap<String, usize>,
        link_definitions: &mut HashMap<Vec<Inline>, LinkDefinition>,
        footnote_counter: &mut usize,
    ) {
        for block in blocks {
            match block {
                Block::FootnoteDefinition(def) => {
                    footnote_index.insert(def.label.clone(), *footnote_counter);
                    *footnote_counter += 1;
                }
                Block::Definition(def) => {
                    link_definitions.insert(def.label.clone(), def.clone());
                }
                Block::List(list) => {
                    for item in &list.items {
                        process_blocks(
                            &item.blocks,
                            footnote_index,
                            link_definitions,
                            footnote_counter,
                        );
                    }
                }
                Block::BlockQuote(blocks) => {
                    process_blocks(blocks, footnote_index, link_definitions, footnote_counter);
                }
                Block::GitHubAlert(alert) => {
                    process_blocks(
                        &alert.blocks,
                        footnote_index,
                        link_definitions,
                        footnote_counter,
                    );
                }
                _ => {}
            }
        }
    }

    process_blocks(
        &ast.blocks,
        &mut footnote_index,
        &mut link_definitions,
        &mut footnote_counter,
    );

    (footnote_index, link_definitions)
}
