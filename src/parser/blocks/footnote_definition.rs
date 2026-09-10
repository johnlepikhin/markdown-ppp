use crate::ast::FootnoteDefinition;
use crate::parser::util::char_m_n;
use crate::parser::util::{line_terminated, not_eof_or_eol1};
use crate::parser::MarkdownParserState;
use nom::character::complete::none_of;
use nom::{
    bytes::complete::tag,
    combinator::{recognize, verify},
    multi::{many0, many1},
    sequence::preceded,
    IResult, Parser,
};
use std::rc::Rc;

/// Footnote definition marker and its first line: `[^label]: text`.
///
/// Returns the label and the first content line. Used as a cheap lookahead that does
/// not parse the footnote body.
pub(crate) fn footnote_definition_start(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = char_m_n(0, 3, ' ').parse(input)?;
    let (input, _) = tag("[^").parse(input)?;
    let (input, label) = recognize(many1(verify(none_of("]"), |c| *c != ']'))).parse(input)?;
    let (input, _) = tag("]:").parse(input)?;
    let (input, _) = char_m_n(0, 3, ' ').parse(input)?;
    let (input, first_line) = line_terminated(not_eof_or_eol1).parse(input)?;
    Ok((input, (label, first_line)))
}

pub(crate) fn footnote_definition<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, FootnoteDefinition> {
    move |input: &'a str| {
        let (input, (label, first_line)) = footnote_definition_start(input)?;
        let (input, rest_lines) = many0(preceded(
            char_m_n(3, 3, ' '),
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

        let nested_state = Rc::new(state.nested());
        let (_, blocks) = crate::parser::blocks::blocks_many0(nested_state, &footnote_content)
            .map_err(|err| err.map_input(|_| input))?;

        let v = FootnoteDefinition {
            label: label.to_owned(),
            blocks,
        };

        Ok((input, v))
    }
}
