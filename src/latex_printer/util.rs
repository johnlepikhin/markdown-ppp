//! Utility functions for LaTeX rendering
//!
//! This module provides helper functions for LaTeX generation including
//! character escaping and LaTeX environment/command generation.

use pretty::{Arena, DocAllocator, DocBuilder};

/// Escape LaTeX special characters in text
///
/// This function converts plain text to LaTeX-safe text by escaping all
/// special characters that have meaning in LaTeX. This is essential for
/// preventing LaTeX compilation errors and ensuring text displays correctly.
///
/// # LaTeX Special Characters
///
/// The following characters are escaped:
/// - `\` → `\textbackslash{}`
/// - `{` → `\{`
/// - `}` → `\}`
/// - `$` → `\$`
/// - `&` → `\&`
/// - `%` → `\%`
/// - `#` → `\#`
/// - `^` → `\textasciicircum{}`
/// - `_` → `\_`
/// - `~` → `\textasciitilde{}`
///
/// # Examples
///
/// ```rust
/// # use markdown_ppp::latex_printer::util::escape_latex;
/// assert_eq!(escape_latex("Hello & goodbye"), "Hello \\& goodbye");
/// assert_eq!(escape_latex("Price: $100"), "Price: \\$100");
/// assert_eq!(escape_latex("50% off"), "50\\% off");
/// ```
pub fn escape_latex(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '\\' => r"\textbackslash{}".to_string(),
            '{' => r"\{".to_string(),
            '}' => r"\}".to_string(),
            '$' => r"\$".to_string(),
            '&' => r"\&".to_string(),
            '%' => r"\%".to_string(),
            '#' => r"\#".to_string(),
            '^' => r"\textasciicircum{}".to_string(),
            '_' => r"\_".to_string(),
            '~' => r"\textasciitilde{}".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

/// Create a LaTeX environment with begin/end blocks
///
/// This function generates a complete LaTeX environment with proper
/// `\begin{env}` and `\end{env}` markers, optional parameters, and content.
///
/// # Arguments
///
/// * `arena` - The pretty-printer arena for document generation
/// * `name` - The environment name (e.g., "itemize", "verbatim")
/// * `options` - Optional environment parameters (e.g., "\[htbp\]")
/// * `content` - The content to place inside the environment
///
/// # Examples
///
/// ```rust
/// # use pretty::{Arena, DocAllocator};
/// # use markdown_ppp::latex_printer::util::environment;
/// let arena = Arena::new();
/// let content = arena.text("Hello world");
/// let env = environment(&arena, "quote", None, content);
/// // Generates: \begin{quote}\nHello world\n\end{quote}
/// ```
///
/// With options:
/// ```rust
/// # use pretty::{Arena, DocAllocator};
/// # use markdown_ppp::latex_printer::util::environment;
/// let arena = Arena::new();
/// let content = arena.text("Column 1 & Column 2");
/// let env = environment(&arena, "tabular", Some("lc"), content);
/// // Generates: \begin{tabular}[lc]\nColumn 1 & Column 2\n\end{tabular}
/// ```
pub fn environment<'a>(
    arena: &'a Arena<'a>,
    name: &str,
    options: Option<&str>,
    content: DocBuilder<'a, Arena<'a>, ()>,
) -> DocBuilder<'a, Arena<'a>, ()> {
    let begin = if let Some(opts) = options {
        arena.text(format!(r"\begin{{{name}}}[{opts}]"))
    } else {
        arena.text(format!(r"\begin{{{name}}}"))
    };

    let end = arena.text(format!(r"\end{{{name}}}"));

    begin
        .append(arena.hardline())
        .append(content)
        .append(arena.hardline())
        .append(end)
}

/// Create a LaTeX command with optional arguments and content
///
/// This function generates a LaTeX command with optional square-bracket
/// arguments and curly-brace content.
///
/// # Arguments
///
/// * `arena` - The pretty-printer arena for document generation
/// * `name` - The command name (without backslash)
/// * `args` - Optional square-bracket arguments
/// * `content` - The content to place in curly braces
///
/// # Examples
///
/// Basic command:
/// ```rust
/// # use pretty::{Arena, DocAllocator};
/// # use markdown_ppp::latex_printer::util::command;
/// let arena = Arena::new();
/// let content = arena.text("bold text");
/// let cmd = command(&arena, "textbf", &[], content);
/// // Generates: \textbf{bold text}
/// ```
///
/// Command with arguments:
/// ```rust
/// # use pretty::{Arena, DocAllocator};
/// # use markdown_ppp::latex_printer::util::command;
/// let arena = Arena::new();
/// let content = arena.text("https://example.com");
/// let cmd = command(&arena, "href", &["target=_blank"], content);
/// // Generates: \href[target=_blank]{https://example.com}
/// ```
pub fn command<'a>(
    arena: &'a Arena<'a>,
    name: &str,
    args: &[&str],
    content: DocBuilder<'a, Arena<'a>, ()>,
) -> DocBuilder<'a, Arena<'a>, ()> {
    let mut cmd = arena.text(format!(r"\{name}"));

    for arg in args {
        cmd = cmd.append(arena.text(format!("[{arg}]")));
    }

    cmd.append(arena.text("{"))
        .append(content)
        .append(arena.text("}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_latex() {
        assert_eq!(escape_latex("hello"), "hello");
        assert_eq!(escape_latex("$100"), r"\$100");
        assert_eq!(escape_latex("A & B"), r"A \& B");
        assert_eq!(escape_latex("50%"), r"50\%");
        assert_eq!(escape_latex("#hashtag"), r"\#hashtag");
        assert_eq!(escape_latex("x^2"), r"x\textasciicircum{}2");
        assert_eq!(escape_latex("file_name"), r"file\_name");
        assert_eq!(escape_latex("~home"), r"\textasciitilde{}home");
        assert_eq!(escape_latex("{code}"), r"\{code\}");
        assert_eq!(escape_latex(r"\command"), r"\textbackslash{}command");
    }
}
