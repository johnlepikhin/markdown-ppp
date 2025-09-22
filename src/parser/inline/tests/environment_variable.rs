use super::super::environment_variable::*;
use crate::ast::*;

#[test]
fn test_basic_env_var() {
    let result = environment_variable("PKG_CONFIG_PATH").unwrap();
    assert_eq!(result.0, "");
    assert_eq!(result.1, Inline::Text("PKG_CONFIG_PATH".to_string()));
}

#[test]
fn test_mixed_case_env_var() {
    let result = environment_variable("CMAKE_Build_Type").unwrap();
    assert_eq!(result.0, "");
    assert_eq!(result.1, Inline::Text("CMAKE_Build_Type".to_string()));
}

#[test]
fn test_lowercase_env_var() {
    let result = environment_variable("my_custom_var").unwrap();
    assert_eq!(result.0, "");
    assert_eq!(result.1, Inline::Text("my_custom_var".to_string()));
}

#[test]
fn test_env_var_with_numbers() {
    let result = environment_variable("VAR_123_test").unwrap();
    assert_eq!(result.0, "");
    assert_eq!(result.1, Inline::Text("VAR_123_test".to_string()));
}

#[test]
fn test_reject_no_underscore() {
    assert!(environment_variable("NOUNDERCORE").is_err());
}

#[test]
fn test_reject_starts_with_underscore() {
    assert!(environment_variable("_INVALID").is_err());
}

#[test]
fn test_reject_ends_with_underscore() {
    assert!(environment_variable("INVALID_").is_err());
}

#[test]
fn test_reject_consecutive_underscores() {
    assert!(environment_variable("INVALID__VAR").is_err());
}

#[test]
fn test_reject_too_short() {
    assert!(environment_variable("A_").is_err());
    assert!(environment_variable("_A").is_err());
}

#[test]
fn test_integration_with_full_parser() {
    use crate::parser::{parse_markdown, MarkdownParserState};

    let result = parse_markdown(MarkdownParserState::default(), "PKG_CONFIG_PATH").unwrap();
    println!("Integration test result: {result:?}");

    // Проверяем, что PKG_CONFIG_PATH парсится как один Text элемент
    if let crate::ast::Block::Paragraph(inlines) = &result.blocks[0] {
        assert_eq!(inlines.len(), 1);
        if let crate::ast::Inline::Text(text) = &inlines[0] {
            assert_eq!(text, "PKG_CONFIG_PATH");
        } else {
            panic!("Expected Text, got {:?}", inlines[0]);
        }
    }
}
