use crate::ast::Block;
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    character::complete::char,
    combinator::opt,
    multi::{many0, many1, many_m_n},
    sequence::preceded,
    IResult, Parser,
};
use std::rc::Rc;

/// Block quote marker: 0-3 leading spaces followed by `>`.
///
/// Used as a cheap lookahead (e.g. to decide whether a line interrupts a paragraph)
/// without parsing the quote content.
pub(crate) fn blockquote_start(input: &str) -> IResult<&str, char> {
    preceded(many_m_n(0, 3, char(' ')), char('>')).parse(input)
}

pub(crate) fn blockquote<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Block>> {
    move |input: &'a str| {
        // Block quote marker: 0-3 leading spaces, '>', optional space
        // Per CommonMark spec, the space after '>' is part of the marker and should be stripped
        let prefix = (blockquote_start, opt(char(' ')));

        let (input, lines) =
            many1(preceded(prefix, line_terminated(not_eof_or_eol0))).parse(input)?;
        let inner = lines.join("\n");

        // `many0`: a bare `>` line is an empty block quote, not a paragraph.
        let nested_state = Rc::new(state.nested());
        let (_, inner) = many0(crate::parser::blocks::block(nested_state))
            .parse(&inner)
            .map_err(|err| err.map_input(|_| input))?;

        let inner = inner.into_iter().flatten().collect();

        Ok((input, inner))
    }
}
