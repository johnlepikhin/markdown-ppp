use crate::ast::FootnoteDefinition;
use crate::parser::util::{line_terminated, not_eof_or_eol1};
use crate::parser::MarkdownParserState;
use nom::character::complete::{char, none_of};
use nom::{
    bytes::complete::tag,
    combinator::{recognize, verify},
    multi::{many0, many1, many_m_n},
    sequence::preceded,
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn footnote_definition<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, FootnoteDefinition> {
    move |input: &'a str| {
        let (input, _) = many_m_n(0, 3, char(' ')).parse(input)?;
        let (input, _) = tag("[^").parse(input)?;
        let (input, label) = recognize(many1(verify(none_of("]"), |c| *c != ']'))).parse(input)?;
        let (input, _) = tag("]:").parse(input)?;
        let (input, _) = many_m_n(0, 3, char(' ')).parse(input)?;
        let (input, first_line) = line_terminated(not_eof_or_eol1).parse(input)?;
        let (input, rest_lines) = many0(preceded(
            many_m_n(3, 3, char(' ')),
            line_terminated(not_eof_or_eol1),
        ))
        .parse(input)?;

        let total_size = first_line.len() + rest_lines.len();
        let mut footnote_content = String::with_capacity(total_size);
        if !first_line.is_empty() {
            footnote_content.push_str(first_line)
        }
        for line in rest_lines {
            footnote_content.push('\n');
            footnote_content.push_str(line)
        }

        let (_, blocks) = many0(crate::parser::blocks::block(state.clone()))
            .parse(&footnote_content)
            .map_err(|err| err.map_input(|_| input))?;

        let blocks = blocks.into_iter().flatten().collect();

        let v = FootnoteDefinition {
            label: label.to_owned(),
            blocks,
        };

        Ok((input, v))
    }
}
