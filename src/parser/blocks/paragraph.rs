use crate::ast::Inline;
use crate::parser::util::char_m_n;
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    character::complete::{line_ending, space0},
    combinator::{not, peek, value},
    multi::separated_list0,
    sequence::preceded,
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn paragraph<'a>(
    state: Rc<MarkdownParserState>,
    check_first_line: bool,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Inline>> {
    move |input: &'a str| {
        let mut lines = Vec::new();
        let input = if check_first_line {
            input
        } else {
            // Skip checks for the first line, just make it a paragraph
            let (input, first_line) =
                preceded(char_m_n(0, 3, ' '), not_eof_or_eol1).parse(input)?;
            lines.push(first_line);
            input
        };

        let paragraph_parser = separated_list0(
            line_ending,
            preceded(
                is_paragraph_line_start(state.clone()),
                preceded(char_m_n(0, 3, ' '), not_eof_or_eol1),
            ),
        );
        let (input, rest_lines) = line_terminated(paragraph_parser).parse(input)?;
        lines.extend(rest_lines);

        // A single line is parsed in place; only a multi-line paragraph is joined.
        let joined;
        let content: &str = match lines.as_slice() {
            [line] => line,
            _ => {
                joined = lines.join("\n");
                &joined
            }
        };

        let (_, content) = crate::parser::inline::inline_many1(state.clone())
            .parse(content)
            .map_err(|err| err.map_input(|_| input))?;

        Ok((input, content))
    }
}

pub(crate) fn is_paragraph_line_start<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, ()> {
    let has_custom_parser = state.config.custom_block_parser.is_some();
    let mut parser = peek(not(alt((
        conditional_block_unit(
            state.config.block_heading_v1_behavior.clone(),
            value(
                (),
                crate::parser::blocks::heading::heading_v1(state.clone()),
            ),
        ),
        conditional_block_unit(
            state.config.block_heading_v2_behavior.clone(),
            value(
                (),
                crate::parser::blocks::heading::heading_v2_level(state.clone()),
            ),
        ),
        conditional_block_unit(
            state.config.block_thematic_break_behavior.clone(),
            crate::parser::blocks::thematic_break::thematic_break(state.clone()),
        ),
        // Lookaheads below must only match the *marker* of a container block, never
        // parse its content: a full parse here is discarded and repeated by `block`,
        // which makes the parse time exponential in the nesting depth.
        conditional_block_unit(
            state.config.block_blockquote_behavior.clone(),
            value((), crate::parser::blocks::blockquote::blockquote_start),
        ),
        conditional_block_unit(
            state.config.block_list_behavior.clone(),
            value((), crate::parser::blocks::list::list_marker_with_span_size),
        ),
        conditional_block_unit(
            state.config.block_code_block_behavior.clone(),
            value(
                (),
                crate::parser::blocks::code_block::code_block_fenced(state.clone()),
            ),
        ),
        conditional_block_unit(
            state.config.block_html_block_behavior.clone(),
            value(
                (),
                crate::parser::blocks::html_block::html_block(state.clone()),
            ),
        ),
        conditional_block_unit(
            state.config.block_link_definition_behavior.clone(),
            value(
                (),
                crate::parser::blocks::link_definition::link_definition(state.clone()),
            ),
        ),
        conditional_block_unit(
            state.config.block_footnote_definition_behavior.clone(),
            value(
                (),
                crate::parser::blocks::footnote_definition::footnote_definition_start,
            ),
        ),
        conditional_block_unit(
            state.config.block_table_behavior.clone(),
            value((), crate::parser::blocks::table::table(state.clone())),
        ),
        value(
            Pieces::One(()),
            crate::parser::blocks::custom_parser(state.clone()),
        ),
        value(Pieces::One(()), line_terminated(space0)),
    ))));
    move |input: &'a str| {
        // Every lookahead below needs one of a few marker bytes right after at most
        // three spaces (`#`, `=`, `-`, `_`, `*`, `+`, `>`, `` ` ``, `~`, `<`, `[`,
        // `|`, a digit, or a line ending for the blank line). A line starting with
        // anything else can only be a paragraph line, unless a custom block parser
        // is installed, which may accept anything.
        if !has_custom_parser && !may_start_block(input) {
            return Ok((input, ()));
        }
        parser.parse(input)
    }
}

/// Whether the first byte after up to three spaces could start one of the blocks
/// that end a paragraph. Conservative: `true` for every doubtful case.
fn may_start_block(input: &str) -> bool {
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < 3 && i < bytes.len() && bytes[i] == b' ' {
        i += 1;
    }
    match bytes.get(i) {
        None => true,
        Some(b) => {
            b.is_ascii_digit()
                || b.is_ascii_whitespace()
                || matches!(
                    b,
                    b'#' | b'='
                        | b'-'
                        | b'_'
                        | b'*'
                        | b'+'
                        | b'>'
                        | b'`'
                        | b'~'
                        | b'<'
                        | b'['
                        | b'|'
                )
        }
    }
}
