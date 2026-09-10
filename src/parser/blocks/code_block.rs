use crate::ast::{CodeBlock, CodeBlockKind};
use crate::parser::util::char_m_n;
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{not, opt, peek, recognize, value},
    multi::{many0, many1},
    sequence::preceded,
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn code_block<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, CodeBlock> {
    move |input: &'a str| {
        alt((
            code_block_indented(state.clone()),
            code_block_fenced(state.clone()),
        ))
        .parse(input)
    }
}

pub(crate) fn code_block_indented<'a>(
    _state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, CodeBlock> {
    move |input: &'a str| {
        let line_parser = preceded(
            alt((value((), char_m_n(4, 4, ' ')), value((), char('\t')))),
            line_terminated(not_eof_or_eol0),
        );

        let (input, lines) = many1(line_parser).parse(input)?;
        let literal = lines.join("\n");

        let code_block = CodeBlock {
            kind: CodeBlockKind::Indented,
            literal,
        };

        Ok((input, code_block))
    }
}

pub(crate) fn code_block_fenced<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, CodeBlock> {
    move |input: &'a str| {
        let (input, space_prefix) = char_m_n(0, 3, ' ').parse(input)?;
        // Only strip prefix indentation when NOT in nested context.
        // In nested context (inside list items, blockquotes, etc.), the container
        // parser already stripped the container's indentation from the content.
        let prefix_length = if state.is_nested_block_context {
            0
        } else {
            space_prefix.len()
        };

        let (input, (fence, info)) = line_terminated((
            recognize(alt((
                char_m_n(3, usize::MAX, '`'),
                char_m_n(3, usize::MAX, '~'),
            ))),
            opt(recognize(not_eof_or_eol1)),
        ))
        .parse(input)?;
        let ending_fence = || {
            line_terminated((
                char_m_n(0, 3, ' '),
                tag(fence),
                many0(char(fence.chars().next().unwrap())),
            ))
        };

        let (input, lines) = many0(preceded(
            peek(not(ending_fence())),
            preceded(
                char_m_n(0, prefix_length, ' '),
                line_terminated(not_eof_or_eol0),
            ),
        ))
        .parse(input)?;
        let (input, _) = ending_fence().parse(input)?;

        let literal = lines.join("\n");
        let code_block = CodeBlock {
            kind: CodeBlockKind::Fenced {
                info: info.map(|v| v.to_owned()),
            },
            literal,
        };

        Ok((input, code_block))
    }
}
