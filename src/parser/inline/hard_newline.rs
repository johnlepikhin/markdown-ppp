use crate::ast::Inline;
use crate::parser::util::char_m_n;
use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::value,
    sequence::pair,
    IResult, Parser,
};

pub(crate) fn hard_newline(input: &str) -> IResult<&str, Inline> {
    value(
        Inline::LineBreak,
        alt((
            value((), pair(char('\\'), line_ending)),
            value((), pair(char_m_n(2, usize::MAX, ' '), line_ending)),
        )),
    )
    .parse(input)
}
