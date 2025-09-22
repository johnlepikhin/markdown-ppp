//! Fully‑typed Abstract Syntax Tree (AST) for CommonMark + GitHub Flavored Markdown (GFM)
//! ------------------------------------------------------------------------------------
//! This module models every construct described in the **CommonMark 1.0 specification**
//! together with the widely‑used **GFM extensions**: tables, strikethrough, autolinks,
//! task‑list items and footnotes.
//!
//! The design separates **block‑level** and **inline‑level** nodes because parsers and
//! renderers typically operate on these tiers independently.
//!
//! ```text
//! Document ─┐
//!           └─ Block ─┐
//!                     ├─ Inline
//!                     └─ ...
//! ```

mod github_alerts;
pub use github_alerts::{GitHubAlert, GitHubAlertType};

// ——————————————————————————————————————————————————————————————————————————
// Document root
// ——————————————————————————————————————————————————————————————————————————

/// Root of a Markdown document
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Document {
    /// Top‑level block sequence **in document order**.
    pub blocks: Vec<Block>,
}

// ——————————————————————————————————————————————————————————————————————————
// Block‑level nodes
// ——————————————————————————————————————————————————————————————————————————

/// Block‑level constructs in the order they appear in the CommonMark spec.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Block {
    /// Ordinary paragraph
    Paragraph(Vec<Inline>),

    /// ATX (`# Heading`) or Setext (`===`) heading
    Heading(Heading),

    /// Thematic break (horizontal rule)
    ThematicBreak,

    /// Block quote
    BlockQuote(Vec<Block>),

    /// List (bullet or ordered)
    List(List),

    /// Fenced or indented code block
    CodeBlock(CodeBlock),

    /// Raw HTML block
    HtmlBlock(String),

    /// Link reference definition.  Preserved for round‑tripping.
    Definition(LinkDefinition),

    /// Tables
    Table(Table),

    /// Footnote definition
    FootnoteDefinition(FootnoteDefinition),

    /// GitHub alert block (NOTE, TIP, IMPORTANT, WARNING, CAUTION)
    GitHubAlert(GitHubAlert),

    /// Empty block. This is used to represent skipped blocks in the AST.
    Empty,
}

/// Heading with level 1–6 and inline content.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Heading {
    /// Kind of heading (ATX or Setext) together with the level.
    pub kind: HeadingKind,

    /// Inlines that form the heading text (before trimming).
    pub content: Vec<Inline>,
}

/// Heading with level 1–6 and inline content.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HeadingKind {
    /// ATX heading (`# Heading`)
    Atx(u8),

    /// Setext heading (`===` or `---`)
    Setext(SetextHeading),
}

/// Setext heading with level and underline type.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SetextHeading {
    /// Setext heading with `=` underline
    Level1,

    /// Setext heading with `-` underline
    Level2,
}

// ——————————————————————————————————————————————————————————————————————————
// Lists
// ——————————————————————————————————————————————————————————————————————————

/// A list container — bullet or ordered.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct List {
    /// Kind of list together with additional semantic data (start index or
    /// bullet marker).
    pub kind: ListKind,

    /// List items in source order.
    pub items: Vec<ListItem>,
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

/// Specifies *what kind* of list we have.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListOrderedKindOptions {
    /// Start index (1, 2, …) for ordered lists.
    pub start: u64,
}

/// Concrete bullet character used for a bullet list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ListBulletKind {
    /// `-` U+002D
    Dash,

    /// `*` U+002A
    Star,

    /// `+` U+002B
    Plus,
}

/// Item within a list.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListItem {
    /// Task‑list checkbox state (GFM task‑lists). `None` ⇒ not a task list.
    pub task: Option<TaskState>,

    /// Nested blocks inside the list item.
    pub blocks: Vec<Block>,
}

/// State of a task‑list checkbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TaskState {
    /// Unchecked (GFM task‑list item)
    Incomplete,

    /// Checked (GFM task‑list item)
    Complete,
}

// ——————————————————————————————————————————————————————————————————————————
// Code blocks
// ——————————————————————————————————————————————————————————————————————————

/// Fenced or indented code block.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeBlock {
    /// Distinguishes indented vs fenced code and stores the *info string*.
    pub kind: CodeBlockKind,

    /// Literal text inside the code block **without** final newline trimming.
    pub literal: String,
}

/// The concrete kind of a code block.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CodeBlockKind {
    /// Indented block (≥ 4 spaces or 1 tab per line).
    Indented,

    /// Fenced block with *optional* info string (language, etc.).
    Fenced { info: Option<String> },
}

// ——————————————————————————————————————————————————————————————————————————
// Link reference definitions
// ——————————————————————————————————————————————————————————————————————————

/// Link reference definition (GFM) with a label, destination and optional title.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinkDefinition {
    /// Link label (acts as the *identifier*).
    pub label: Vec<Inline>,

    /// Link URL (absolute or relative) or email address.
    pub destination: String,

    /// Optional title (for links and images).
    pub title: Option<String>,
}

// ——————————————————————————————————————————————————————————————————————————
// Tables
// ——————————————————————————————————————————————————————————————————————————

/// A table is a collection of rows and columns with optional alignment.
/// The first row is the header row.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Table {
    /// Each row is a vector of *cells*; header row is **row 0**.
    pub rows: Vec<TableRow>,

    /// Column alignment; `alignments.len() == column_count`.
    pub alignments: Vec<Alignment>,
}

/// A table row is a vector of cells (columns).
pub type TableRow = Vec<TableCell>;

/// A table cell is a vector of inlines (text, links, etc.).
pub type TableCell = Vec<Inline>;

/// Specifies the alignment of a table cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Alignment {
    /// No alignment specified
    None,

    /// Left aligned
    #[default]
    Left,

    /// Right aligned
    Center,

    /// Right aligned
    Right,
}

// ——————————————————————————————————————————————————————————————————————————
// Footnotes
// ——————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, PartialEq)]
/// Footnote definition block (e.g., `[^label]: content`).
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FootnoteDefinition {
    /// Normalized label (without leading `^`).
    pub label: String,

    /// Footnote content (blocks).
    pub blocks: Vec<Block>,
}

// ——————————————————————————————————————————————————————————————————————————
// Inline‑level nodes
// ——————————————————————————————————————————————————————————————————————————

/// Inline-level elements within paragraphs, headings, and other blocks.
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Inline {
    /// Plain text (decoded entity references, preserved backslash escapes).
    Text(String),

    /// Hard line break
    LineBreak,

    /// Inline code span
    Code(String),

    /// Raw HTML fragment
    Html(String),

    /// Link to a destination with optional title.
    Link(Link),

    /// Reference link
    LinkReference(LinkReference),

    /// Image with optional title.
    Image(Image),

    /// Emphasis (`*` / `_`)
    Emphasis(Vec<Inline>),
    /// Strong emphasis (`**` / `__`)
    Strong(Vec<Inline>),
    /// Strikethrough (`~~`)
    Strikethrough(Vec<Inline>),

    /// Autolink (`<https://>` or `<mailto:…>`)
    Autolink(String),

    /// Footnote reference (`[^label]`)
    FootnoteReference(String),

    /// Empty element. This is used to represent skipped elements in the AST.
    Empty,
}

/// Re‑usable structure for links and images (destination + children).
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Link {
    /// Destination URL (absolute or relative) or email address.
    pub destination: String,

    /// Optional title (for links and images).
    pub title: Option<String>,

    /// Inline content (text, code, etc.) inside the link or image.
    pub children: Vec<Inline>,
}

/// Re‑usable structure for links and images (destination + children).
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Image {
    /// Image URL (absolute or relative).
    pub destination: String,

    /// Optional title.
    pub title: Option<String>,

    /// Alternative text.
    pub alt: String,
}

/// Reference-style link (e.g., `[text][label]` or `[label][]`).
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinkReference {
    /// Link label (acts as the *identifier*).
    pub label: Vec<Inline>,

    /// Link text
    pub text: Vec<Inline>,
}
