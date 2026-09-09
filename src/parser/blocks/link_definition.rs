use super::eof_or_eol;
use crate::ast::LinkDefinition;
use crate::parser::link_util::{link_destination, link_label, link_title};
use crate::parser::MarkdownParserState;
use nom::character::complete::{char, line_ending, space0, space1};
use nom::{
    branch::alt,
    combinator::{opt, recognize, verify},
    multi::{many1, many_m_n},
    sequence::preceded,
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn link_definition<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, LinkDefinition> {
    move |input: &'a str| {
        let mut one_line_whitespace0 = (space0, opt(line_ending), space0);
        let one_line_whitespace1 = verify(
            recognize(many1(alt((line_ending, space1)))),
            |chars: &str| {
                let mut newlines = 0;
                for ch in chars.chars() {
                    if ch == '\n' {
                        newlines += 1;
                    }
                }
                newlines <= 1
            },
        );

        let (input, label) =
            preceded(many_m_n(0, 3, char(' ')), link_label(state.clone())).parse(input)?;
        let (input, _) = char(':').parse(input)?;
        let (input, _) = one_line_whitespace0.parse(input)?;
        let (input, destination) = link_destination(state.clone()).parse(input)?;
        let (input, title) =
            opt(preceded(one_line_whitespace1, link_title(state.clone()))).parse(input)?;
        let (input, _) = eof_or_eol.parse(input)?;

        let v = LinkDefinition {
            label,
            destination,
            title,
        };

        Ok((input, v))
    }
}
