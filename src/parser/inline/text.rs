use crate::parser::MarkdownParserState;
use crate::{ast::Inline, parser::util::conditional_inline_unit};
use nom::{
    branch::alt,
    character::complete::{anychar, char, one_of},
    combinator::{map, not, peek, recognize, value},
    multi::many1,
    sequence::preceded,
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn text<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Inline> {
    move |input: &'a str| {
        map(
            many1(alt((
                map(escaped_char, |c| c.to_string()),
                map(
                    crate::parser::inline::html_entity::html_entity(state.clone()),
                    |c| c.to_string(),
                ),
                map(
                    recognize(many1(preceded(peek(is_text(state.clone())), anychar))),
                    |c| c.to_string(),
                ),
            ))),
            |vec| Inline::Text(vec.join("")),
        )
        .parse(input)
    }
}

fn is_text<'a>(state: Rc<MarkdownParserState>) -> impl FnMut(&'a str) -> IResult<&'a str, ()> {
    move |input: &'a str| not(not_a_text(state.clone())).parse(input)
}

fn not_a_text<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<()>> {
    move |input: &'a str| {
        alt((
            conditional_inline_unit(
                state.config.inline_autolink_behavior.clone(),
                value((), crate::parser::inline::autolink::autolink),
            ),
            conditional_inline_unit(
                state.config.inline_reference_link_behavior.clone(),
                value(
                    (),
                    crate::parser::inline::reference_link::reference_link(state.clone()),
                ),
            ),
            conditional_inline_unit(
                state.config.inline_hard_newline_behavior.clone(),
                value((), crate::parser::inline::hard_newline::hard_newline),
            ),
            conditional_inline_unit(
                state.config.inline_text_behavior.clone(),
                value(
                    (),
                    crate::parser::inline::html_entity::html_entity(state.clone()),
                ),
            ),
            conditional_inline_unit(
                state.config.inline_image_behavior.clone(),
                value((), crate::parser::inline::image::image(state.clone())),
            ),
            conditional_inline_unit(
                state.config.inline_link_behavior.clone(),
                value(
                    (),
                    crate::parser::inline::inline_link::inline_link(state.clone()),
                ),
            ),
            conditional_inline_unit(
                state.config.inline_code_span_behavior.clone(),
                value((), crate::parser::inline::code_span::code_span),
            ),
            conditional_inline_unit(
                state.config.inline_emphasis_behavior.clone(),
                value((), crate::parser::inline::emphasis::emphasis(state.clone())),
            ),
            conditional_inline_unit(
                state.config.inline_footnote_reference_behavior.clone(),
                value(
                    (),
                    crate::parser::inline::footnote_reference::footnote_reference,
                ),
            ),
            conditional_inline_unit(
                state.config.inline_strikethrough_behavior.clone(),
                value(
                    (),
                    crate::parser::inline::strikethrough::strikethrough(state.clone()),
                ),
            ),
        ))
        .parse(input)
    }
}

fn escaped_char(input: &str) -> IResult<&str, char> {
    preceded(char('\\'), one_of("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~")).parse(input)
}
