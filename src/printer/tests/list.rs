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
    println!("{:?} => {:#?}", input, doc);
    let result = crate::printer::render_markdown(&doc, config);
    assert_eq!(input, result);
}

#[rstest(
    input,
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
fn symmetric_round_trip_list_with_empty_line_before_list(input: &str) {
    let config = crate::printer::config::Config::default().with_empty_line_before_list(false);
    let doc = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();
    println!("{:?} => {:#?}", input, doc);
    let result = crate::printer::render_markdown(&doc, config);
    assert_eq!(input, result);
}
