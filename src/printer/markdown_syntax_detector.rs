//! Markdown syntax detector for safe line wrapping
//!
//! This module provides functionality to detect when a word at the beginning
//! of a line might be interpreted as markdown syntax, which would break the
//! document structure when text is wrapped.

use regex::Regex;
use std::sync::OnceLock;

/// Characters that can start markdown block syntax when at the beginning of a line
const MARKDOWN_BLOCK_STARTERS: &[char] = &['-', '*', '+', '#', '>', '|'];

/// Get compiled regex for ordered list detection
fn ordered_list_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"^\d+\.$").expect("Invalid regex"))
}

/// Get compiled regex for horizontal rule detection
fn horizontal_rule_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"^(-{3,}|\*{3,}|_{3,})$").expect("Invalid regex"))
}

/// Get compiled regex for code fence detection
fn code_fence_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"^(`{3,}|~{4,})").expect("Invalid regex"))
}

/// Check if a word would be interpreted as markdown syntax at the start of a line
///
/// This function detects various markdown block-level syntax elements that
/// would break document structure if they appeared at the beginning of a line
/// due to text wrapping.
///
/// # Arguments
///
/// * `word` - The word that would appear at the start of a line
///
/// # Returns
///
/// `true` if the word could be interpreted as markdown syntax, `false` otherwise
///
pub fn is_markdown_syntax_at_line_start(word: &str) -> bool {
    if word.is_empty() {
        return false;
    }

    // Check for simple single-character block starters
    if word.len() == 1 {
        let first_char = word.chars().next().unwrap();
        if word == "\t" {
            return true; // Special case for tab
        }
        return MARKDOWN_BLOCK_STARTERS.contains(&first_char);
    }

    // Check for heading markers (# ## ### etc.)
    if word.starts_with('#') && word.chars().all(|c| c == '#') {
        return true;
    }

    // Check for ordered list markers (1. 2. 42. etc.)
    if ordered_list_regex().is_match(word) {
        return true;
    }

    // Check for horizontal rules (--- *** ___ etc.)
    if horizontal_rule_regex().is_match(word) {
        return true;
    }

    // Check for code fences (``` ~~~ ```` etc.)
    if code_fence_regex().is_match(word) {
        return true;
    }

    // Check for indented code blocks (4+ spaces or single tab at start)
    if word.starts_with("    ") || word.starts_with('\t') {
        return true;
    }

    false
}

/// Determine if a line break before a given word would be safe
///
/// This function considers the word that would appear at the start of the new line
/// and determines if placing it there would create unwanted markdown syntax.
///
/// # Arguments
///
/// * `next_word` - The word that would start the new line after a break
/// * `context_words` - Additional context words that might be relevant
///
/// # Returns
///
/// `true` if the line break is safe, `false` if it would create syntax conflicts
pub fn is_safe_line_break_before(next_word: &str, _context_words: &[&str]) -> bool {
    !is_markdown_syntax_at_line_start(next_word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unordered_list_markers() {
        assert!(is_markdown_syntax_at_line_start("-"));
        assert!(is_markdown_syntax_at_line_start("*"));
        assert!(is_markdown_syntax_at_line_start("+"));
    }

    #[test]
    fn test_ordered_list_markers() {
        assert!(is_markdown_syntax_at_line_start("1."));
        assert!(is_markdown_syntax_at_line_start("42."));
        assert!(is_markdown_syntax_at_line_start("999."));

        assert!(!is_markdown_syntax_at_line_start("1")); // No dot
        assert!(!is_markdown_syntax_at_line_start("1.5")); // Decimal
        assert!(!is_markdown_syntax_at_line_start("a.")); // Not a number
    }

    #[test]
    fn test_heading_markers() {
        assert!(is_markdown_syntax_at_line_start("#"));
        assert!(is_markdown_syntax_at_line_start("##"));
        assert!(is_markdown_syntax_at_line_start("###"));
        assert!(is_markdown_syntax_at_line_start("####"));
        assert!(is_markdown_syntax_at_line_start("#####"));
        assert!(is_markdown_syntax_at_line_start("######"));

        assert!(!is_markdown_syntax_at_line_start("#text")); // Has other chars
        assert!(!is_markdown_syntax_at_line_start("##text")); // Has other chars
    }

    #[test]
    fn test_blockquote_markers() {
        assert!(is_markdown_syntax_at_line_start(">"));
    }

    #[test]
    fn test_table_markers() {
        assert!(is_markdown_syntax_at_line_start("|"));
    }

    #[test]
    fn test_horizontal_rule_markers() {
        assert!(is_markdown_syntax_at_line_start("---"));
        assert!(is_markdown_syntax_at_line_start("----"));
        assert!(is_markdown_syntax_at_line_start("***"));
        assert!(is_markdown_syntax_at_line_start("****"));
        assert!(is_markdown_syntax_at_line_start("___"));
        assert!(is_markdown_syntax_at_line_start("____"));

        assert!(!is_markdown_syntax_at_line_start("--")); // Too short
        assert!(!is_markdown_syntax_at_line_start("**")); // Too short
        assert!(!is_markdown_syntax_at_line_start("__")); // Too short
    }

    #[test]
    fn test_code_fence_markers() {
        assert!(is_markdown_syntax_at_line_start("```"));
        assert!(is_markdown_syntax_at_line_start("````"));
        assert!(is_markdown_syntax_at_line_start("~~~~"));
        assert!(is_markdown_syntax_at_line_start("~~~~~"));

        assert!(!is_markdown_syntax_at_line_start("``")); // Too short
        assert!(!is_markdown_syntax_at_line_start("~~~")); // Too short
    }

    #[test]
    fn test_indented_code_markers() {
        assert!(is_markdown_syntax_at_line_start("    ")); // 4 spaces
        assert!(is_markdown_syntax_at_line_start("    code")); // 4 spaces + content

        assert!(is_markdown_syntax_at_line_start("\t")); // Tab
        assert!(is_markdown_syntax_at_line_start("\tcode")); // Tab + content

        assert!(!is_markdown_syntax_at_line_start("   ")); // Only 3 spaces
        assert!(!is_markdown_syntax_at_line_start("  code")); // Only 2 spaces
    }

    #[test]
    fn test_regular_words() {
        assert!(!is_markdown_syntax_at_line_start("word"));
        assert!(!is_markdown_syntax_at_line_start("hello"));
        assert!(!is_markdown_syntax_at_line_start("123"));
        assert!(!is_markdown_syntax_at_line_start("test-word"));
        assert!(!is_markdown_syntax_at_line_start("markdown"));
    }

    #[test]
    fn test_safe_line_break() {
        assert!(is_safe_line_break_before("word", &[]));
        assert!(!is_safe_line_break_before("-", &[]));
        assert!(!is_safe_line_break_before("*", &[]));
        assert!(!is_safe_line_break_before("#", &[]));
        assert!(!is_safe_line_break_before("1.", &[]));
    }
}
