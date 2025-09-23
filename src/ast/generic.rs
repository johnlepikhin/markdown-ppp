//! Generic Abstract Syntax Tree (AST) for CommonMark + GitHub Flavored Markdown (GFM)
//! =====================================================================================
//!
//! This module provides generic versions of all AST structures that allow attaching
//! user-defined data to any AST node. The generic parameter `T` represents the type
//! of user data that can be associated with each element.
//!
//! # Features
//!
//! - **Zero-cost abstraction**: When `T = ()`, no additional memory is used
//! - **Flexible user data**: Support for any user-defined type
//! - **Serde compatibility**: Proper serialization with optional user data fields
//! - **Type safety**: Compile-time guarantees about data presence
//!
//! # Examples
//!
//! ```rust
//! use markdown_ppp::ast::generic::*;
//!
//! // AST without user data (equivalent to regular AST)
//! type SimpleDocument = Document<()>;
//!
//! // AST with element IDs
//! #[derive(Debug, Clone, PartialEq)]
//! struct ElementId(u32);
//! type DocumentWithIds = Document<ElementId>;
//!
//! // AST with source information
//! #[derive(Debug, Clone, PartialEq)]
//! struct SourceInfo {
//!     line: u32,
//!     column: u32,
//! }
//! type DocumentWithSource = Document<SourceInfo>;
//! ```

// Re-export types from parent module that don't need generics
pub use super::{
    Alignment, CodeBlockKind, GitHubAlert, GitHubAlertType, HeadingKind, ListBulletKind,
    ListOrderedKindOptions, SetextHeading, TaskState,
};

// ——————————————————————————————————————————————————————————————————————————
// Document root
// ——————————————————————————————————————————————————————————————————————————

/// Root of a Markdown document with optional user data
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Document<T = ()> {
    /// Top‑level block sequence **in document order**.
    pub blocks: Vec<Block<T>>,

    /// User-defined data associated with this document
    #[cfg_attr(feature = "ast-serde", serde(default))]
    pub user_data: T,
}

// ——————————————————————————————————————————————————————————————————————————
// Block‑level nodes
// ——————————————————————————————————————————————————————————————————————————

/// Block‑level constructs in the order they appear in the CommonMark spec.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Block<T = ()> {
    /// Ordinary paragraph
    Paragraph {
        content: Vec<Inline<T>>,
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },

    /// ATX (`# Heading`) or Setext (`===`) heading
    Heading(Heading<T>),

    /// Thematic break (horizontal rule)
    ThematicBreak {
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },

    /// Block quote
    BlockQuote {
        blocks: Vec<Block<T>>,
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },

    /// List (bullet or ordered)
    List(List<T>),

    /// Fenced or indented code block
    CodeBlock(CodeBlock<T>),

    /// Raw HTML block
    HtmlBlock {
        content: String,
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },

    /// Link reference definition. Preserved for round‑tripping.
    Definition(LinkDefinition<T>),

    /// Tables
    Table(Table<T>),

    /// Footnote definition
    FootnoteDefinition(FootnoteDefinition<T>),

    /// GitHub alert block (NOTE, TIP, IMPORTANT, WARNING, CAUTION)
    GitHubAlert(GitHubAlertNode<T>),

    /// Empty block. This is used to represent skipped blocks in the AST.
    Empty {
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },
}

/// Heading with level 1–6 and inline content.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Heading<T = ()> {
    /// Kind of heading (ATX or Setext) together with the level.
    pub kind: HeadingKind,

    /// Inlines that form the heading text (before trimming).
    pub content: Vec<Inline<T>>,

    /// User-defined data associated with this heading
    #[cfg_attr(feature = "ast-serde", serde(default))]
    pub user_data: T,
}

// ——————————————————————————————————————————————————————————————————————————
// Lists
// ——————————————————————————————————————————————————————————————————————————

/// A list container — bullet or ordered.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct List<T = ()> {
    /// Kind of list together with additional semantic data (start index or
    /// bullet marker).
    pub kind: ListKind,

    /// List items in source order.
    pub items: Vec<ListItem<T>>,

    /// User-defined data associated with this list
    #[cfg_attr(feature = "ast-serde", serde(default))]
    pub user_data: T,
}

/// Specifies *what kind* of list we have.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ListKind {
    /// Ordered list (`1.`, `42.` …) with an *optional* explicit start number.
    Ordered(ListOrderedKindOptions),

    /// Bullet list (`-`, `*`, or `+`) together with the concrete marker.
    Bullet(ListBulletKind),
}

/// Item within a list.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListItem<T = ()> {
    /// Task‑list checkbox state (GFM task‑lists). `None` ⇒ not a task list.
    pub task: Option<TaskState>,

    /// Nested blocks inside the list item.
    pub blocks: Vec<Block<T>>,

    /// User-defined data associated with this list item
    #[cfg_attr(feature = "ast-serde", serde(default))]
    pub user_data: T,
}

// ——————————————————————————————————————————————————————————————————————————
// Code blocks
// ——————————————————————————————————————————————————————————————————————————

/// Fenced or indented code block.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeBlock<T = ()> {
    /// Distinguishes indented vs fenced code and stores the *info string*.
    pub kind: CodeBlockKind,

    /// Literal text inside the code block **without** final newline trimming.
    pub literal: String,

    /// User-defined data associated with this code block
    #[cfg_attr(feature = "ast-serde", serde(default))]
    pub user_data: T,
}

// ——————————————————————————————————————————————————————————————————————————
// Link reference definitions
// ——————————————————————————————————————————————————————————————————————————

/// Link reference definition (GFM) with a label, destination and optional title.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinkDefinition<T = ()> {
    /// Link label (acts as the *identifier*).
    pub label: Vec<Inline<T>>,

    /// Link URL (absolute or relative) or email address.
    pub destination: String,

    /// Optional title (for links and images).
    pub title: Option<String>,

    /// User-defined data associated with this link definition
    #[cfg_attr(feature = "ast-serde", serde(default))]
    pub user_data: T,
}

// ——————————————————————————————————————————————————————————————————————————
// Tables
// ——————————————————————————————————————————————————————————————————————————

/// A table is a collection of rows and columns with optional alignment.
/// The first row is the header row.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Table<T = ()> {
    /// Each row is a vector of *cells*; header row is **row 0**.
    pub rows: Vec<TableRow<T>>,

    /// Column alignment; `alignments.len() == column_count`.
    pub alignments: Vec<Alignment>,

    /// User-defined data associated with this table
    #[cfg_attr(feature = "ast-serde", serde(default))]
    pub user_data: T,
}

/// A table row is a vector of cells (columns).
pub type TableRow<T> = Vec<TableCell<T>>;

/// A table cell is a vector of inlines (text, links, etc.).
pub type TableCell<T> = Vec<Inline<T>>;

// ——————————————————————————————————————————————————————————————————————————
// Footnotes
// ——————————————————————————————————————————————————————————————————————————

/// Footnote definition block (e.g., `[^label]: content`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FootnoteDefinition<T = ()> {
    /// Normalized label (without leading `^`).
    pub label: String,

    /// Footnote content (blocks).
    pub blocks: Vec<Block<T>>,

    /// User-defined data associated with this footnote definition
    #[cfg_attr(feature = "ast-serde", serde(default))]
    pub user_data: T,
}

// ——————————————————————————————————————————————————————————————————————————
// GitHub Alerts
// ——————————————————————————————————————————————————————————————————————————

/// GitHub alert block with user data support
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GitHubAlertNode<T = ()> {
    /// Type of alert (NOTE, TIP, IMPORTANT, WARNING, CAUTION)
    pub alert_type: GitHubAlertType,

    /// Content blocks within the alert
    pub blocks: Vec<Block<T>>,

    /// User-defined data associated with this GitHub alert
    #[cfg_attr(feature = "ast-serde", serde(default))]
    pub user_data: T,
}

// ——————————————————————————————————————————————————————————————————————————
// Inline‑level nodes
// ——————————————————————————————————————————————————————————————————————————

/// Inline-level elements within paragraphs, headings, and other blocks.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Inline<T = ()> {
    /// Plain text (decoded entity references, preserved backslash escapes).
    Text {
        content: String,
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },

    /// Hard line break
    LineBreak {
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },

    /// Inline code span
    Code {
        content: String,
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },

    /// Raw HTML fragment
    Html {
        content: String,
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },

    /// Link to a destination with optional title.
    Link(Link<T>),

    /// Reference link
    LinkReference(LinkReference<T>),

    /// Image with optional title.
    Image(Image<T>),

    /// Emphasis (`*` / `_`)
    Emphasis {
        content: Vec<Inline<T>>,
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },

    /// Strong emphasis (`**` / `__`)
    Strong {
        content: Vec<Inline<T>>,
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },

    /// Strikethrough (`~~`)
    Strikethrough {
        content: Vec<Inline<T>>,
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },

    /// Autolink (`<https://>` or `<mailto:…>`)
    Autolink {
        url: String,
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },

    /// Footnote reference (`[^label]`)
    FootnoteReference {
        label: String,
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },

    /// Empty element. This is used to represent skipped elements in the AST.
    Empty {
        #[cfg_attr(feature = "ast-serde", serde(default))]
        user_data: T,
    },
}

/// Re‑usable structure for links and images (destination + children).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Link<T = ()> {
    /// Destination URL (absolute or relative) or email address.
    pub destination: String,

    /// Optional title (for links and images).
    pub title: Option<String>,

    /// Inline content (text, code, etc.) inside the link or image.
    pub children: Vec<Inline<T>>,

    /// User-defined data associated with this link
    #[cfg_attr(feature = "ast-serde", serde(default))]
    pub user_data: T,
}

/// Re‑usable structure for images.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Image<T = ()> {
    /// Image URL (absolute or relative).
    pub destination: String,

    /// Optional title.
    pub title: Option<String>,

    /// Alternative text.
    pub alt: String,

    /// User-defined data associated with this image
    #[cfg_attr(feature = "ast-serde", serde(default))]
    pub user_data: T,
}

/// Reference-style link (e.g., `[text][label]` or `[label][]`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinkReference<T = ()> {
    /// Link label (acts as the *identifier*).
    pub label: Vec<Inline<T>>,

    /// Link text
    pub text: Vec<Inline<T>>,

    /// User-defined data associated with this link reference
    #[cfg_attr(feature = "ast-serde", serde(default))]
    pub user_data: T,
}

// ——————————————————————————————————————————————————————————————————————————
// Default implementations for common cases
// ——————————————————————————————————————————————————————————————————————————

impl<T: Default> Default for Document<T> {
    fn default() -> Self {
        Self {
            blocks: Vec::new(),
            user_data: T::default(),
        }
    }
}

impl<T: Default> Default for Heading<T> {
    fn default() -> Self {
        Self {
            kind: HeadingKind::Atx(1),
            content: Vec::new(),
            user_data: T::default(),
        }
    }
}

impl<T: Default> Default for List<T> {
    fn default() -> Self {
        Self {
            kind: ListKind::Bullet(ListBulletKind::Dash),
            items: Vec::new(),
            user_data: T::default(),
        }
    }
}

impl<T: Default> Default for Table<T> {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            alignments: Vec::new(),
            user_data: T::default(),
        }
    }
}
