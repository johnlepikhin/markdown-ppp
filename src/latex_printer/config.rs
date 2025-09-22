//! Configuration for LaTeX rendering
//!
//! This module provides configuration options to customize the LaTeX output
//! style and format. The main configuration struct [`Config`] allows you to
//! control table styles, code block formatting, and output width.

/// Table rendering style
///
/// Controls how Markdown tables are converted to LaTeX table environments.
/// Each style has different capabilities and LaTeX package requirements.
#[derive(Debug, Clone, PartialEq)]
pub enum TableStyle {
    /// Basic `tabular` environment
    ///
    /// Uses the standard LaTeX `tabular` environment with `\hline` for borders.
    /// This is the most compatible option and works without additional packages.
    ///
    /// **Requirements:** None (built into LaTeX)
    ///
    /// **Example output:**
    /// ```latex
    /// \begin{tabular}[lc]
    /// Header 1 & Header 2 \\
    /// \hline
    /// Cell 1 & Cell 2 \\
    /// \end{tabular}
    /// ```
    Tabular,

    /// Long tables with page breaks using `longtabu`
    ///
    /// Uses the `longtabu` environment which can automatically break across pages.
    /// Useful for very long tables that exceed page length.
    ///
    /// **Requirements:** `longtabu` package
    ///
    /// **Example output:**
    /// ```latex
    /// \begin{longtabu}[X[l] to \textwidth {lc}]
    /// Header 1 & Header 2 \\
    /// \\ \hline
    /// Cell 1 & Cell 2 \\
    /// \end{longtabu}
    /// ```
    Longtabu,

    /// Beautiful tables using `booktabs` package
    ///
    /// Uses professional typography rules from the `booktabs` package.
    /// Produces the most aesthetically pleasing tables with proper spacing
    /// and rule weights.
    ///
    /// **Requirements:** `booktabs` package
    ///
    /// **Example output:**
    /// ```latex
    /// \begin{tabular}[lc]
    /// \toprule
    /// Header 1 & Header 2 \\
    /// \midrule
    /// Cell 1 & Cell 2 \\
    /// \bottomrule
    /// \end{tabular}
    /// ```
    Booktabs,
}

/// Code block rendering style
///
/// Controls how Markdown fenced and indented code blocks are converted
/// to LaTeX environments. Each style offers different features for
/// syntax highlighting and formatting.
#[derive(Debug, Clone, PartialEq)]
pub enum CodeBlockStyle {
    /// Basic `verbatim` environment
    ///
    /// Uses the standard LaTeX `verbatim` environment. No syntax highlighting
    /// but maximum compatibility. Language information is ignored.
    ///
    /// **Requirements:** None (built into LaTeX)
    ///
    /// **Example output:**
    /// ```latex
    /// \begin{verbatim}
    /// fn main() {
    ///     println!("Hello");
    /// }
    /// \end{verbatim}
    /// ```
    Verbatim,

    /// Syntax highlighting with `listings` package
    ///
    /// Uses the `listings` package for syntax highlighting. Supports many
    /// programming languages and offers extensive customization options.
    /// Language is taken from the fenced code block info string.
    ///
    /// **Requirements:** `listings` package
    ///
    /// **Example output:**
    /// ```latex
    /// \begin{lstlisting}[language=rust]
    /// fn main() {
    ///     println!("Hello");
    /// }
    /// \end{lstlisting}
    /// ```
    Listings,

    /// Advanced syntax highlighting with `minted` package
    ///
    /// Uses the `minted` package which leverages Pygments for superior
    /// syntax highlighting. Requires Python and Pygments to be installed.
    /// Supports more languages and produces better highlighting than `listings`.
    ///
    /// **Requirements:** `minted` package, Python, Pygments
    ///
    /// **Example output:**
    /// ```latex
    /// \begin{minted}{rust}
    /// fn main() {
    ///     println!("Hello");
    /// }
    /// \end{minted}
    /// ```
    Minted,
}

/// Configuration for LaTeX rendering
///
/// This struct controls various aspects of how the Markdown AST is converted
/// to LaTeX. Use the builder methods to customize the output style.
///
/// # Examples
///
/// ```rust
/// use markdown_ppp::latex_printer::config::*;
///
/// // Default configuration
/// let config = Config::default();
///
/// // Custom configuration
/// let config = Config::default()
///     .with_width(120)
///     .with_table_style(TableStyle::Booktabs)
///     .with_code_block_style(CodeBlockStyle::Minted);
/// ```
pub struct Config {
    pub(crate) width: usize,
    pub(crate) table_style: TableStyle,
    pub(crate) code_block_style: CodeBlockStyle,
}

impl Default for Config {
    /// Create a default configuration
    ///
    /// Default settings:
    /// - Width: 80 characters
    /// - Table style: [`TableStyle::Tabular`]
    /// - Code block style: [`CodeBlockStyle::Verbatim`]
    fn default() -> Self {
        Self {
            width: 80,
            table_style: TableStyle::Tabular,
            code_block_style: CodeBlockStyle::Verbatim,
        }
    }
}

impl Config {
    /// Set the line width for pretty-printing
    ///
    /// Controls how the pretty-printer wraps long lines. This affects the
    /// formatting of the generated LaTeX, not the content itself.
    ///
    /// # Arguments
    ///
    /// * `width` - Maximum line width in characters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use markdown_ppp::latex_printer::config::Config;
    ///
    /// let config = Config::default().with_width(120);
    /// ```
    pub fn with_width(self, width: usize) -> Self {
        Self { width, ..self }
    }

    /// Set the table rendering style
    ///
    /// Controls which LaTeX environment is used for tables and what
    /// styling rules are applied.
    ///
    /// # Arguments
    ///
    /// * `table_style` - The desired table style
    ///
    /// # Examples
    ///
    /// ```rust
    /// use markdown_ppp::latex_printer::config::*;
    ///
    /// let config = Config::default()
    ///     .with_table_style(TableStyle::Booktabs);
    /// ```
    pub fn with_table_style(self, table_style: TableStyle) -> Self {
        Self {
            table_style,
            ..self
        }
    }

    /// Set the code block rendering style
    ///
    /// Controls which LaTeX environment is used for code blocks and
    /// whether syntax highlighting is applied.
    ///
    /// # Arguments
    ///
    /// * `code_block_style` - The desired code block style
    ///
    /// # Examples
    ///
    /// ```rust
    /// use markdown_ppp::latex_printer::config::*;
    ///
    /// let config = Config::default()
    ///     .with_code_block_style(CodeBlockStyle::Minted);
    /// ```
    pub fn with_code_block_style(self, code_block_style: CodeBlockStyle) -> Self {
        Self {
            code_block_style,
            ..self
        }
    }
}
