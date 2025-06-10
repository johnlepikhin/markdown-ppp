use crate::ast::Inline;
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    character::complete::{char, line_ending, space0},
    combinator::{not, peek, value},
    multi::{many_m_n, separated_list0},
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
                preceded(many_m_n(0, 3, char(' ')), not_eof_or_eol1).parse(input)?;
            lines.push(first_line);
            input
        };

        let paragraph_parser = separated_list0(
            line_ending,
            preceded(
                is_paragraph_line_start(state.clone()),
                preceded(many_m_n(0, 3, char(' ')), not_eof_or_eol1),
            ),
        );
        let (input, rest_lines) = line_terminated(paragraph_parser).parse(input)?;
        lines.extend(rest_lines);

        let content = lines.join("\n");

        let (_, content) = crate::parser::inline::inline_many1(state.clone())
            .parse(content.as_str())
            .map_err(|err| err.map_input(|_| input))?;

        Ok((input, content))
    }
}

pub(crate) fn is_paragraph_line_start<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, ()> {
    move |input: &'a str| {
        peek(not(alt((
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
            conditional_block_unit(
                state.config.block_blockquote_behavior.clone(),
                value(
                    (),
                    crate::parser::blocks::blockquote::blockquote(state.clone()),
                ),
            ),
            conditional_block_unit(
                state.config.block_list_behavior.clone(),
                value((), crate::parser::blocks::list::list_item(state.clone())),
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
                    crate::parser::blocks::footnote_definition::footnote_definition(state.clone()),
                ),
            ),
            conditional_block_unit(
                state.config.block_table_behavior.clone(),
                value((), crate::parser::blocks::table::table(state.clone())),
            ),
            value(
                vec![()],
                crate::parser::blocks::custom_parser(state.clone()),
            ),
            value(vec![()], line_terminated(space0)),
        ))))
        .parse(input)
    }
}
