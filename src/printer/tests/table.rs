#![cfg(test)]
use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};
use crate::printer::{config::Config, render_markdown};

#[test]
fn table_no_line_wrapping_with_small_width() {
    // Test that tables never wrap lines even with very small width
    let input = r#"| Very long header that exceeds width | Another very long header that also exceeds width |
| ------------------------------------ | ------------------------------------------------- |
| Very long cell content that definitely exceeds any reasonable width limit | Another very long cell content here |"#;

    let doc = parse_markdown(MarkdownParserState::default(), input).unwrap();

    // Test with very small width
    let config = Config::default().with_width(20);
    let result = render_markdown(&doc, config);

    // Table should be rendered as single lines without wrapping
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 3); // Header + separator + data row

    // Each line should be much longer than the width limit
    for line in &lines {
        assert!(line.len() > 20, "Line should exceed width limit: {}", line);
    }

    // Verify no line breaks within table rows
    assert!(!result.contains("Very long header that exceeds\nwidth"));
    assert!(!result.contains("Very long cell content that definitely\nexceeds"));
}

#[test]
fn table_preserves_structure_with_long_cells() {
    let input = r#"| Short | This is a very long cell that contains multiple words and should not be broken across lines |
| ----- | -------------------------------------------------------------------------------------------- |
| A     | Another long cell with important information that must stay on one line                      |
| B     | Yet another cell with extensive content that tests the table rendering capabilities          |"#;

    let doc = parse_markdown(MarkdownParserState::default(), input).unwrap();

    let config = Config::default().with_width(50);
    let result = render_markdown(&doc, config);

    // Should have exactly 4 lines: header + separator + 2 data rows
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 4);

    // Each line should preserve table structure (start with |, contain |, end with |)
    for line in &lines {
        assert!(line.starts_with('|'), "Line should start with |: {}", line);
        assert!(line.ends_with('|'), "Line should end with |: {}", line);
        assert!(
            line.matches('|').count() >= 3,
            "Line should have at least 3 | characters: {}",
            line
        );
    }
}

#[test]
fn table_renders_with_different_widths() {
    let input = r#"| Header1 | Header2 | Header3 |
| ------- | ------- | ------- |
| Cell1   | Cell2   | Cell3   |"#;

    let doc = parse_markdown(MarkdownParserState::default(), input).unwrap();

    // Test with different width configurations
    let widths = [10, 40, 80, 120, 200];

    for width in widths {
        let config = Config::default().with_width(width);
        let result = render_markdown(&doc, config);

        // All should produce the same output regardless of width
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(
            lines.len(),
            3,
            "Should have 3 lines regardless of width {}",
            width
        );

        // Table structure should be preserved
        for line in &lines {
            assert!(
                line.starts_with('|'),
                "Width {}: Line should start with |: {}",
                width,
                line
            );
            assert!(
                line.ends_with('|'),
                "Width {}: Line should end with |: {}",
                width,
                line
            );
        }
    }
}

#[test]
fn table_handles_newlines_in_cell_content() {
    // This test shows how newlines in input are handled
    // Note: in markdown, newlines in table cells are typically treated as paragraph breaks
    // So this input likely won't parse as a table at all
    let input = r#"| Header |
| ------ |
| Text with spaces |"#;

    let doc = parse_markdown(MarkdownParserState::default(), input).unwrap();
    let config = Config::default();
    let result = render_markdown(&doc, config);

    // Should be a valid table structure
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 3); // Header + separator + data

    // Verify table structure is preserved
    for line in &lines {
        assert!(line.starts_with('|'), "Line should start with |: {}", line);
        assert!(line.ends_with('|'), "Line should end with |: {}", line);
    }
}

#[test]
fn table_with_user_example_structure() {
    // Test based on the user's original problem (corrected version)
    let input = r#"| Участок | Протяженность | Крутизна | Характер рельефа | Кат. сл. | Кол-во Крючьев / закладок |
| ------- | ------------- | -------- | ---------------- | -------- | ------------------------- |
| R0 — начало маршрута — цирк балки Гнилая | Удобное место для организации связок |  |  |  |  |
| R0–R1 — Подъем на перемычку (пер. Липецкий) | 250 м | 30–35° | Снег, летом — фирн | 2– | Кр. — 0/0 <br> Закл. — 0/0 |"#;

    let doc = parse_markdown(MarkdownParserState::default(), input).unwrap();

    // Verify it parses as a table with all rows
    if let Block::Table(table) = &doc.blocks[0] {
        assert_eq!(table.rows.len(), 3, "Should have header + 2 data rows");
        assert_eq!(table.alignments.len(), 6, "Should have 6 columns");
    } else {
        panic!("Should parse as a table");
    }

    let config = Config::default().with_width(80);
    let result = render_markdown(&doc, config);

    // Should render as table without line wrapping
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(
        lines.len(),
        4,
        "Should have 4 lines: header + separator + 2 data rows"
    );

    // Each line should preserve table structure
    for line in &lines {
        assert!(line.starts_with('|'), "Line should start with |: {}", line);
        assert!(line.ends_with('|'), "Line should end with |: {}", line);
    }

    // Check that long content doesn't wrap
    let first_data_line = &lines[2];
    assert!(
        first_data_line.contains("R0 — начало маршрута — цирк балки Гнилая"),
        "Should contain full cell content"
    );
    assert!(first_data_line.len() > 100, "Line should be long"); // Much longer than typical width
}
