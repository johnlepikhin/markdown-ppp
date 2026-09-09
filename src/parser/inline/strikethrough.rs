use crate::ast::Inline;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char},
    combinator::{not, peek, recognize, value},
    multi::many1,
    sequence::{preceded, terminated},
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn strikethrough<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Inline> {
    move |input: &'a str| {
        let (input, _) = terminated(tag("~~"), peek(not(char('~')))).parse(input)?;
        let not_a_closing_tag = (tag("~~"), char('~'));
        let closing_tag = preceded(peek(not(not_a_closing_tag)), tag("~~"));
        let content_parser = recognize(many1(preceded(
            peek(not(closing_tag)),
            alt((value('~', tag("\\~")), anychar)),
        )));
        let (input, content) = recognize(content_parser).parse(input)?;
        let (input, _) = tag("~~").parse(input)?;

        let nested_state = Rc::new(state.deeper());
        let (_, inline) = crate::parser::inline::inline_many1(nested_state).parse(content)?;

        Ok((input, Inline::Strikethrough(inline)))
    }
}
