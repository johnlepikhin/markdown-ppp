use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};
use nom::combinator::value;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn custom_parser1() {
    use nom::Parser;
    let config = crate::parser::config::MarkdownParserConfig::default().with_custom_block_parser(
        Rc::new(RefCell::new(Box::new(|input: &str| {
            value(vec![Block::ThematicBreak], nom::bytes::complete::tag("///")).parse(input)
        }))),
    );
    let doc = parse_markdown(MarkdownParserState::with_config(config), "///\ntext\n===").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![
                Block::ThematicBreak,
                Block::Heading(Heading {
                    kind: HeadingKind::Setext(SetextHeading::Level1),
                    content: vec![Inline::Text("text".to_owned())]
                })
            ]
        }
    );
}
