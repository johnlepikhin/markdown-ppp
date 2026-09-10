use crate::parser::util::char_m_n;
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    character::complete::{char, space0},
    combinator::map,
    multi::many,
    sequence::{preceded, terminated},
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn thematic_break<'a>(
    _state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, ()> {
    move |input: &str| {
        map(
            line_terminated(preceded(
                char_m_n(0, 3, ' '),
                terminated(
                    alt((
                        many(3.., char('-')),
                        many(3.., char('_')),
                        many(3.., char('*')),
                    )),
                    space0,
                ),
            )),
            |_: Vec<_>| (),
        )
        .parse(input)
    }
}
