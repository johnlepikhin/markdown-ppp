//! Tests for line wrapping issues that can break markdown syntax
//!
//! When text is wrapped to fit within a width limit, certain characters
//! at the beginning of a new line can be interpreted as markdown syntax,
//! changing the document's meaning.

use crate::ast::*;
use crate::parser::parse_markdown;
use crate::parser::MarkdownParserState;
use crate::printer::{config::Config, render_markdown};

/// Test that text with dashes doesn't create accidental list items when wrapped
#[test]
fn test_dash_wrapping_creates_list() {
    // This text contains many dashes that could become list markers if wrapped incorrectly
    let text = "a - b - c - d - e - f - g - h - i - j - k - l - m - n - o - p - q - r - s - t";

    // Parse the original text
    let original_doc = parse_markdown(MarkdownParserState::default(), text).unwrap();

    // Should be a single paragraph
    assert_eq!(original_doc.blocks.len(), 1);
    assert!(matches!(original_doc.blocks[0], Block::Paragraph(_)));

    // Render with very narrow width to force wrapping
    let config = Config::default().with_width(10);
    let rendered = render_markdown(&original_doc, config);

    // Parse the rendered result to check if it's still the same structure
    let rendered_doc = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    // Should still be a single paragraph, not a list
    assert_eq!(
        rendered_doc.blocks.len(),
        1,
        "Rendered document should still be a single paragraph, but got: {:?}",
        rendered_doc.blocks
    );
    assert!(
        matches!(rendered_doc.blocks[0], Block::Paragraph(_)),
        "First block should be a paragraph, but got: {:?}",
        rendered_doc.blocks[0]
    );
}

/// Test that text with asterisks doesn't create accidental list items when wrapped
#[test]
fn test_asterisk_wrapping_creates_list() {
    let text = "a * b * c * d * e * f * g * h * i * j * k * l * m * n * o * p * q * r * s * t";

    let original_doc = parse_markdown(MarkdownParserState::default(), text).unwrap();
    assert_eq!(original_doc.blocks.len(), 1);
    assert!(matches!(original_doc.blocks[0], Block::Paragraph(_)));

    let config = Config::default().with_width(10);
    let rendered = render_markdown(&original_doc, config);
    let rendered_doc = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    assert_eq!(
        rendered_doc.blocks.len(),
        1,
        "Rendered document should still be a single paragraph, but got: {:?}",
        rendered_doc.blocks
    );
    assert!(matches!(rendered_doc.blocks[0], Block::Paragraph(_)));
}

/// Test that text with plus signs doesn't create accidental list items when wrapped
#[test]
fn test_plus_wrapping_creates_list() {
    let text = "a + b + c + d + e + f + g + h + i + j + k + l + m + n + o + p + q + r + s + t";

    let original_doc = parse_markdown(MarkdownParserState::default(), text).unwrap();
    assert_eq!(original_doc.blocks.len(), 1);
    assert!(matches!(original_doc.blocks[0], Block::Paragraph(_)));

    let config = Config::default().with_width(10);
    let rendered = render_markdown(&original_doc, config);
    let rendered_doc = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    assert_eq!(rendered_doc.blocks.len(), 1);
    assert!(matches!(rendered_doc.blocks[0], Block::Paragraph(_)));
}

/// Test that text with hash symbols doesn't create accidental headings when wrapped
#[test]
fn test_hash_wrapping_creates_heading() {
    let text = "This is a long paragraph with # hash symbols # scattered throughout # the text # that should # not become # headings when # wrapped";

    let original_doc = parse_markdown(MarkdownParserState::default(), text).unwrap();
    assert_eq!(original_doc.blocks.len(), 1);
    assert!(matches!(original_doc.blocks[0], Block::Paragraph(_)));

    let config = Config::default().with_width(15);
    let rendered = render_markdown(&original_doc, config);
    let rendered_doc = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    // Should still be one paragraph, not multiple headings
    assert_eq!(
        rendered_doc.blocks.len(),
        1,
        "Should be one paragraph but got {} blocks: {:?}",
        rendered_doc.blocks.len(),
        rendered_doc.blocks
    );
    assert!(matches!(rendered_doc.blocks[0], Block::Paragraph(_)));
}

/// Test that text with pipe symbols doesn't create accidental tables when wrapped
#[test]
fn test_pipe_wrapping_creates_table() {
    let text = "Data looks like this: value1 | value2 | value3 | value4 | value5 | value6 | value7 | value8";

    let original_doc = parse_markdown(MarkdownParserState::default(), text).unwrap();
    assert_eq!(original_doc.blocks.len(), 1);
    assert!(matches!(original_doc.blocks[0], Block::Paragraph(_)));

    let config = Config::default().with_width(15);
    let rendered = render_markdown(&original_doc, config);
    let rendered_doc = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    // Should still be a paragraph, not a table
    assert_eq!(rendered_doc.blocks.len(), 1);
    assert!(matches!(rendered_doc.blocks[0], Block::Paragraph(_)));
}

/// Test that text with greater-than symbols doesn't create accidental blockquotes when wrapped
#[test]
fn test_gt_wrapping_creates_blockquote() {
    let text = "In programming, we use operators like > greater than > and < less than > frequently > in comparisons > and logical expressions";

    let original_doc = parse_markdown(MarkdownParserState::default(), text).unwrap();
    assert_eq!(original_doc.blocks.len(), 1);
    assert!(matches!(original_doc.blocks[0], Block::Paragraph(_)));

    let config = Config::default().with_width(20);
    let rendered = render_markdown(&original_doc, config);
    let rendered_doc = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    // Should still be a paragraph, not a blockquote
    assert_eq!(rendered_doc.blocks.len(), 1);
    assert!(matches!(rendered_doc.blocks[0], Block::Paragraph(_)));
}

/// Test that text with numbered list patterns doesn't create accidental ordered lists when wrapped
#[test]
fn test_number_dot_wrapping_creates_ordered_list() {
    let text = "Version numbers: 1. First release 2. Second patch 3. Major update 4. Hotfix 5. Feature release 6. Bug fix 7. Final version";

    let original_doc = parse_markdown(MarkdownParserState::default(), text).unwrap();
    assert_eq!(original_doc.blocks.len(), 1);
    assert!(matches!(original_doc.blocks[0], Block::Paragraph(_)));

    let config = Config::default().with_width(25);
    let rendered = render_markdown(&original_doc, config);
    let rendered_doc = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    // Should still be a paragraph, not an ordered list
    assert_eq!(
        rendered_doc.blocks.len(),
        1,
        "Should be one paragraph but got: {:?}",
        rendered_doc.blocks
    );
    assert!(matches!(rendered_doc.blocks[0], Block::Paragraph(_)));
}

/// Test that text with horizontal rule patterns doesn't create accidental thematic breaks when wrapped
#[test]
fn test_horizontal_rule_wrapping_creates_thematic_break() {
    let text = "Mathematical expressions: --- means subtraction --- and addition --- or sometimes --- division --- depending on context";

    let original_doc = parse_markdown(MarkdownParserState::default(), text).unwrap();
    assert_eq!(original_doc.blocks.len(), 1);
    assert!(matches!(original_doc.blocks[0], Block::Paragraph(_)));

    let config = Config::default().with_width(20);
    let rendered = render_markdown(&original_doc, config);
    let rendered_doc = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    // Should still be a paragraph, not multiple blocks separated by thematic breaks
    assert_eq!(rendered_doc.blocks.len(), 1);
    assert!(matches!(rendered_doc.blocks[0], Block::Paragraph(_)));
}

/// Test that text with backticks doesn't create accidental code blocks when wrapped
#[test]
fn test_code_fence_wrapping_creates_code_block() {
    let text = "Code samples: ``` inline code ``` and ``` another sample ``` with ``` multiple examples ``` throughout the text";

    let original_doc = parse_markdown(MarkdownParserState::default(), text).unwrap();
    assert_eq!(original_doc.blocks.len(), 1);
    assert!(matches!(original_doc.blocks[0], Block::Paragraph(_)));

    let config = Config::default().with_width(20);
    let rendered = render_markdown(&original_doc, config);
    let rendered_doc = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    // Should still be a paragraph, not code blocks
    assert_eq!(rendered_doc.blocks.len(), 1);
    assert!(matches!(rendered_doc.blocks[0], Block::Paragraph(_)));
}

/// Test multiple potential syntax conflicts in one text
#[test]
fn test_multiple_syntax_conflicts() {
    let text = "Complex text with # headings * lists - items + more > quotes | tables 1. numbers ``` code --- rules";

    let original_doc = parse_markdown(MarkdownParserState::default(), text).unwrap();
    assert_eq!(original_doc.blocks.len(), 1);
    assert!(matches!(original_doc.blocks[0], Block::Paragraph(_)));

    let config = Config::default().with_width(12);
    let rendered = render_markdown(&original_doc, config);
    let rendered_doc = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    // Should still be a paragraph despite all the potential syntax conflicts
    assert_eq!(
        rendered_doc.blocks.len(),
        1,
        "Should be one paragraph but parsing the rendered output gave: {:?}",
        rendered_doc.blocks
    );
    assert!(matches!(rendered_doc.blocks[0], Block::Paragraph(_)));
}

/// Test edge case with indented content that could become code blocks
#[test]
fn test_indentation_wrapping_creates_code_block() {
    let text = "    This line starts with spaces    and has more spaces    throughout the text    which could    be problematic";

    let original_doc = parse_markdown(MarkdownParserState::default(), text).unwrap();

    let config = Config::default().with_width(15);
    let rendered = render_markdown(&original_doc, config);
    let rendered_doc = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    // The structure should remain consistent
    assert_eq!(rendered_doc.blocks.len(), original_doc.blocks.len());
    assert_eq!(
        std::mem::discriminant(&rendered_doc.blocks[0]),
        std::mem::discriminant(&original_doc.blocks[0])
    );
}

/// Helper function to create test cases for specific problematic patterns
fn test_problematic_pattern(pattern: &str, description: &str) {
    // Create text that doesn't already contain markdown syntax
    // Use spaces around pattern to avoid creating unintended markdown
    let text = if pattern.contains('*') {
        // For patterns with asterisks, use a different approach to avoid Strong parsing
        format!("text {pattern} more {pattern} words {pattern} here {pattern} and {pattern} there {pattern} end")
    } else {
        format!("a {pattern} b {pattern} c {pattern} d {pattern} e {pattern} f {pattern} g {pattern} h {pattern}")
    };

    let original_doc = parse_markdown(MarkdownParserState::default(), &text).unwrap();

    // Skip patterns that already create complex structures in the original
    if original_doc.blocks.len() != 1 {
        // Original text already has complex structure, skip this test
        return;
    }

    let config = Config::default().with_width(8); // Very narrow to force wrapping
    let rendered = render_markdown(&original_doc, config);
    let rendered_doc = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

    assert_eq!(
        rendered_doc.blocks.len(),
        original_doc.blocks.len(),
        "Block count changed for pattern '{}' ({}). Original: {}, Rendered: {}.\nOriginal text: {:?}\nRendered text: {:?}",
        pattern, description, original_doc.blocks.len(), rendered_doc.blocks.len(), text, rendered
    );

    if !original_doc.blocks.is_empty() && !rendered_doc.blocks.is_empty() {
        assert_eq!(
            std::mem::discriminant(&rendered_doc.blocks[0]),
            std::mem::discriminant(&original_doc.blocks[0]),
            "Block type changed for pattern '{pattern}' ({description})"
        );
    }
}

/// Test various problematic patterns systematically
#[test]
fn test_systematic_problematic_patterns() {
    let patterns = vec![
        ("-", "dash/hyphen (unordered list marker)"),
        ("*", "asterisk (unordered list marker or emphasis)"),
        ("+", "plus (unordered list marker)"),
        ("#", "hash (heading marker)"),
        ("##", "double hash (heading marker)"),
        ("###", "triple hash (heading marker)"),
        (">", "greater than (blockquote marker)"),
        ("|", "pipe (table marker)"),
        ("1.", "number dot (ordered list marker)"),
        ("2.", "number dot (ordered list marker)"),
        ("10.", "double digit number dot"),
        ("```", "triple backtick (code fence)"),
        ("---", "triple dash (horizontal rule)"),
        ("___", "triple underscore (horizontal rule)"),
        ("    ", "four spaces (code block)"),
        ("\t", "tab (code block)"),
    ];

    for (pattern, description) in patterns {
        test_problematic_pattern(pattern, description);
    }
}

/// Test that the round-trip property holds even with problematic wrapping
#[test]
fn test_round_trip_with_wrapping_issues() {
    let problematic_texts = vec![
        "a - b - c - d - e - f - g - h - i - j - k - l",
        "Version 1. was released 2. years ago 3. months after 4. the initial design",
        "Use operators > for comparison > and < for less than > in code",
        "Data format: value1 | value2 | value3 | value4 | value5",
        "Headers like # Main # Sub # Detail # levels are common",
        "Code with ``` examples ``` and ``` samples ``` throughout",
    ];

    for text in problematic_texts {
        let original_doc = parse_markdown(MarkdownParserState::default(), text).unwrap();

        // Test with various narrow widths
        for width in [10, 15, 20, 25] {
            let config = Config::default().with_width(width);
            let rendered = render_markdown(&original_doc, config);
            let rendered_doc = parse_markdown(MarkdownParserState::default(), &rendered).unwrap();

            assert_eq!(
                rendered_doc.blocks.len(),
                original_doc.blocks.len(),
                "Round-trip failed for text '{}' at width {}: block count changed from {} to {}.\nRendered text: {:?}",
                text, width, original_doc.blocks.len(), rendered_doc.blocks.len(), rendered
            );
        }
    }
}
