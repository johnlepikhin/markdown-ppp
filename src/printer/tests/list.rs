#![cfg(test)]
use rstest::rstest;

#[rstest(
    input,
    case(
        r#"- item1
- item2"#
    ),
    case(
        r#"11. item1
12. item2"#
    ),
    case(
        r#"9. item1
10. item2"#
    ),
    case(
        r#"- item1
- item2
  
  - item2 1
  - item2 2"#
    )
)]
fn symmetric_round_trip_list_with_spaces_before_list_item(input: &str) {
    let config = crate::printer::config::Config::default().with_spaces_before_list_item(0);
    let doc = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();
    println!("{input:?} => {doc:#?}");
    let result = crate::printer::render_markdown(&doc, config);
    assert_eq!(input, result);
}

#[rstest(
    input,
    case(
        r#"# head1
 - item1
 - item2

# head2
 - item3
 - item4

# head3
 - item5
 - item6"#
    ),
    case(
        r#" - item1
 - item2
    - item2 1
    - item2 2"#
    ),
    case(
        r#" - item1
 - item2
    - item2 1
    - item2 2
       - item2 2 1
       - item2 2 2"#
    )
)]
fn symmetric_round_trip_list_without_empty_line_before_list(input: &str) {
    let config = crate::printer::config::Config::default().with_empty_line_before_list(false);
    let doc = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();
    println!("{input:?} => {doc:#?}");
    let result = crate::printer::render_markdown(&doc, config);
    assert_eq!(input, result);
}

// Regression test: fenced code blocks inside list items should preserve internal indentation
// and formatting should be idempotent (format(format(x)) == format(x))
// Fix for bug: fenced code blocks inside lists were losing indentation on each render pass
// because literal '\n' characters prevented nest() from applying indentation properly.
#[rstest(
    input,
    // Basic code block in ordered list item
    case(
        r#" 1. **Example:**

    ```rust
    fn test() {
        println!("hello");
    }
    ```"#
    ),
    // Code block with multiple indentation levels
    case(
        r#" 1. Item with code:

    ```python
    def foo():
        if True:
            return bar()
    ```"#
    ),
    // Nested list with code block (2 levels)
    case(
        r#" - Outer item
    - Inner item with code:

      ```js
      function test() {
          console.log("nested");
      }
      ```"#
    ),
    // Empty code block in list item
    case(
        r#" - Empty code block:

    ```rust
    ```"#
    ),
    // Code block with blank lines inside
    case(
        r#" - Code with blank lines:

    ```python
    def foo():
        pass

    def bar():
        pass
    ```"#
    ),
    // Deeply nested list with code block (3 levels)
    case(
        r#" - Level 1
    - Level 2
       - Level 3 with code:

         ```rust
         fn deep() {
             nested();
         }
         ```"#
    ),
    // Unordered list with asterisk marker
    case(
        r#" * Item with asterisk:

    ```rust
    fn asterisk() {}
    ```"#
    ),
    // Unordered list with plus marker
    case(
        r#" + Item with plus:

    ```rust
    fn plus() {}
    ```"#
    ),
    // Multiple code blocks in one list item
    case(
        r#" - Multiple blocks:

    First:

    ```rust
    fn first() {}
    ```

    Second:

    ```rust
    fn second() {}
    ```"#
    ),
    // Code block with unusual info string
    case(
        r#" - Item:

    ```rust,no_run,edition=2021
    fn info_string() {}
    ```"#
    ),
)]
fn fenced_code_block_in_list_idempotent(input: &str) {
    // First pass
    let doc1 = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();
    let pass1 = crate::printer::render_markdown(&doc1, crate::printer::config::Config::default());

    // Second pass - should be identical to first pass (idempotent)
    let doc2 = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), &pass1)
        .unwrap();
    let pass2 = crate::printer::render_markdown(&doc2, crate::printer::config::Config::default());

    assert_eq!(
        pass1, pass2,
        "Formatting should be idempotent.\nInput:\n{input}\n\nFirst pass:\n{pass1}\n\nSecond pass:\n{pass2}"
    );
}

// Test that code blocks in blockquotes are also idempotent
#[rstest(
    input,
    // Code block in blockquote
    case(
        r#"> Quote with code:
>
> ```rust
> fn quoted() {
>     println!("in quote");
> }
> ```"#
    ),
    // Code block in GitHub alert
    case(
        r#"> [!NOTE]
> Alert with code:
>
> ```python
> def alert():
>     pass
> ```"#
    ),
)]
fn fenced_code_block_in_blockquote_idempotent(input: &str) {
    let doc1 = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();
    let pass1 = crate::printer::render_markdown(&doc1, crate::printer::config::Config::default());

    let doc2 = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), &pass1)
        .unwrap();
    let pass2 = crate::printer::render_markdown(&doc2, crate::printer::config::Config::default());

    assert_eq!(
        pass1, pass2,
        "Formatting should be idempotent.\nInput:\n{input}\n\nFirst pass:\n{pass1}\n\nSecond pass:\n{pass2}"
    );
}
