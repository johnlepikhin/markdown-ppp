use crate::ast::Block;
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    character::complete::char,
    multi::{many1, many_m_n},
    sequence::preceded,
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn blockquote<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Block>> {
    move |input: &'a str| {
        let prefix = preceded(many_m_n(0, 3, char(' ')), char('>'));

        let (input, lines) =
            many1(preceded(prefix, line_terminated(not_eof_or_eol0))).parse(input)?;
        let inner = lines.join("\n");

        let (_, inner) = many1(crate::parser::blocks::block(state.clone()))
            .parse(&inner)
            .map_err(|err| err.map_input(|_| input))?;

        let inner = inner.into_iter().flatten().collect();

        Ok((input, inner))
    }
}
