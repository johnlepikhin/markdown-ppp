//! Tests for verifying that parser correctly merges consecutive text elements
//!
//! These tests check cases where the parser might create multiple consecutive
//! Text elements that should be merged into a single element.

use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

/// Checks that there are no consecutive Text elements in the vec
fn assert_no_consecutive_text_elements(inlines: &[Inline]) {
    for window in inlines.windows(2) {
        if let [Inline::Text(_), Inline::Text(_)] = window {
            panic!(
                "Found consecutive Text elements in {:?}, which should be merged",
                inlines
            );
        }
    }

    // Recursively check content of other elements
    for inline in inlines {
        match inline {
            Inline::Emphasis(content)
            | Inline::Strong(content)
            | Inline::Strikethrough(content) => {
                assert_no_consecutive_text_elements(content);
            }
            Inline::Link(link) => {
                assert_no_consecutive_text_elements(&link.children);
            }
            Inline::LinkReference(link_ref) => {
                assert_no_consecutive_text_elements(&link_ref.label);
                assert_no_consecutive_text_elements(&link_ref.text);
            }
            _ => {}
        }
    }
}

/// Checks entire document for absence of consecutive Text elements
fn assert_no_consecutive_text_in_document(doc: &Document) {
    for block in &doc.blocks {
        match block {
            Block::Paragraph(inlines) => {
                assert_no_consecutive_text_elements(inlines);
            }
            Block::Heading(heading) => {
                assert_no_consecutive_text_elements(&heading.content);
            }
            Block::BlockQuote(blocks) => {
                assert_no_consecutive_text_in_document(&Document {
                    blocks: blocks.clone(),
                });
            }
            Block::List(list) => {
                for item in &list.items {
                    assert_no_consecutive_text_in_document(&Document {
                        blocks: item.blocks.clone(),
                    });
                }
            }
            Block::Table(table) => {
                for row in &table.rows {
                    for cell in row {
                        assert_no_consecutive_text_elements(cell);
                    }
                }
            }
            Block::FootnoteDefinition(footnote) => {
                assert_no_consecutive_text_in_document(&Document {
                    blocks: footnote.blocks.clone(),
                });
            }
            Block::GitHubAlert(alert) => {
                assert_no_consecutive_text_in_document(&Document {
                    blocks: alert.blocks.clone(),
                });
            }
            // Other blocks don't contain inline elements
            _ => {}
        }
    }
}

#[test]
fn test_environment_variables_with_text() {
    // Environment variables between regular text
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "Set PKG_CONFIG_PATH and CMAKE_BUILD_TYPE to debug for testing",
    )
    .unwrap();

    assert_no_consecutive_text_in_document(&doc);
}

#[test]
fn test_emphasis_with_surrounding_text() {
    // Emphasis with surrounding text
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "This is *emphasized* text with more content",
    )
    .unwrap();

    assert_no_consecutive_text_in_document(&doc);
}

#[test]
fn test_multiple_environment_variables() {
    // Multiple environment variables
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "Use PATH_TO_FILE and CMAKE_BUILD_TYPE and PKG_CONFIG_PATH variables",
    )
    .unwrap();

    assert_no_consecutive_text_in_document(&doc);
}

#[test]
fn test_mixed_inline_elements() {
    // Mix of different inline elements
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "Text with ENV_VAR and *emphasis* and **strong** and `code` formatting",
    )
    .unwrap();

    assert_no_consecutive_text_in_document(&doc);
}

#[test]
fn test_escaped_characters_with_text() {
    // Escaped characters with text
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "Text with \\* escaped asterisk \\_ underscore and more text",
    )
    .unwrap();

    assert_no_consecutive_text_in_document(&doc);
}

#[test]
fn test_autolinks_with_text() {
    // Autolinks with text
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "Visit <https://example.com> for more information about ENV_VAR usage",
    )
    .unwrap();

    assert_no_consecutive_text_in_document(&doc);
}

#[test]
fn test_html_entities_with_text() {
    // HTML entities with text
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "Text &amp; more text &lt; even more text",
    )
    .unwrap();

    assert_no_consecutive_text_in_document(&doc);
}

#[test]
fn test_complex_paragraph() {
    // Complex paragraph with multiple elements
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "Configure CMAKE_BUILD_TYPE to *debug* mode using PKG_CONFIG_PATH variable and check <https://example.com> for &amp; documentation"
    ).unwrap();

    assert_no_consecutive_text_in_document(&doc);
}

#[test]
fn test_nested_emphasis_with_env_vars() {
    // Nested emphasis with environment variables
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "Use **strong with *nested ENV_VAR emphasis* and more** formatting",
    )
    .unwrap();

    assert_no_consecutive_text_in_document(&doc);
}

#[test]
fn test_list_items_with_mixed_content() {
    // List items with mixed content
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "- Set ENV_VAR to *value* for testing\n- Use CMAKE_BUILD_TYPE in **production** mode\n- Check &amp; validate settings"
    ).unwrap();

    assert_no_consecutive_text_in_document(&doc);
}

#[test]
fn test_table_cells_with_mixed_content() {
    // Table cells with mixed content
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "| Variable | Value | Description |\n|----------|-------|-------------|\n| ENV_VAR | *debug* | Test &amp; development |\n| PKG_CONFIG_PATH | `/usr/lib` | System **path** |"
    ).unwrap();

    assert_no_consecutive_text_in_document(&doc);
}

#[test]
fn test_blockquote_with_mixed_content() {
    // Blockquote with mixed content
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "> Set ENV_VAR for *testing* and check PKG_CONFIG_PATH configuration",
    )
    .unwrap();

    assert_no_consecutive_text_in_document(&doc);
}

#[test]
fn test_heading_with_mixed_content() {
    // Heading with mixed content
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "# Configuration of ENV_VAR and *Other* Settings",
    )
    .unwrap();

    assert_no_consecutive_text_in_document(&doc);
}

#[test]
fn test_multiline_paragraph_with_mixed_content() {
    // Multiline paragraph with mixed content
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "First line with ENV_VAR variable\nSecond line with *emphasis* formatting\nThird line with PKG_CONFIG_PATH and more content"
    ).unwrap();

    assert_no_consecutive_text_in_document(&doc);
}

#[test]
fn test_footnote_reference_with_text() {
    // Footnote references with text
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "Text with footnote[^1] and ENV_VAR variable\n\n[^1]: Footnote with PKG_CONFIG_PATH reference"
    ).unwrap();

    assert_no_consecutive_text_in_document(&doc);
}
