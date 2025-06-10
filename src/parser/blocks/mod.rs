mod blockquote;
mod code_block;
mod footnote_definition;
mod heading;
mod html_block;
mod link_definition;
mod list;
pub(crate) mod paragraph;
mod table;
mod thematic_break;

#[cfg(test)]
mod tests;

use crate::ast::Block;
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::branch::alt;
use nom::combinator::fail;
use nom::{combinator::map, sequence::preceded, IResult, Parser};
use std::rc::Rc;

pub(crate) fn block<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Block>> {
    move |input: &'a str| {
        preceded(
            many_empty_lines0,
            alt((
                conditional_block(
                    state.config.block_heading_v1_behavior.clone(),
                    map(
                        crate::parser::blocks::heading::heading_v1(state.clone()),
                        Block::Heading,
                    ),
                ),
                conditional_block(
                    state.config.block_heading_v2_behavior.clone(),
                    crate::parser::blocks::heading::heading_v2_or_paragraph(state.clone()),
                ),
                conditional_block(
                    state.config.block_thematic_break_behavior.clone(),
                    map(
                        crate::parser::blocks::thematic_break::thematic_break(state.clone()),
                        |()| Block::ThematicBreak,
                    ),
                ),
                conditional_block(
                    state.config.block_blockquote_behavior.clone(),
                    map(
                        crate::parser::blocks::blockquote::blockquote(state.clone()),
                        Block::BlockQuote,
                    ),
                ),
                conditional_block(
                    state.config.block_list_behavior.clone(),
                    map(
                        crate::parser::blocks::list::list(state.clone()),
                        Block::List,
                    ),
                ),
                conditional_block(
                    state.config.block_code_block_behavior.clone(),
                    map(
                        crate::parser::blocks::code_block::code_block(state.clone()),
                        Block::CodeBlock,
                    ),
                ),
                conditional_block(
                    state.config.block_html_block_behavior.clone(),
                    map(
                        crate::parser::blocks::html_block::html_block(state.clone()),
                        |s| Block::HtmlBlock(s.to_owned()),
                    ),
                ),
                // Alway try before link definition
                conditional_block(
                    state.config.block_footnote_definition_behavior.clone(),
                    map(
                        crate::parser::blocks::footnote_definition::footnote_definition(
                            state.clone(),
                        ),
                        Block::FootnoteDefinition,
                    ),
                ),
                conditional_block(
                    state.config.block_link_definition_behavior.clone(),
                    map(
                        crate::parser::blocks::link_definition::link_definition(state.clone()),
                        Block::Definition,
                    ),
                ),
                conditional_block(
                    state.config.block_table_behavior.clone(),
                    map(
                        crate::parser::blocks::table::table(state.clone()),
                        Block::Table,
                    ),
                ),
                custom_parser(state.clone()),
                conditional_block(
                    state.config.block_paragraph_behavior.clone(),
                    map(
                        crate::parser::blocks::paragraph::paragraph(state.clone(), false),
                        Block::Paragraph,
                    ),
                ),
            )),
        )
        .parse(input)
    }
}

pub(crate) fn custom_parser(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&str) -> IResult<&str, Vec<Block>> {
    move |input: &str| {
        if let Some(custom_parser) = state.config.custom_block_parser.as_ref() {
            let mut p = (**custom_parser).borrow_mut();
            (p.as_mut())(input)
        } else {
            fail().parse(input)
        }
    }
}
