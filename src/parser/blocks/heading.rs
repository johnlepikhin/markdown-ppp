use crate::ast::{Block, Heading, HeadingKind, SetextHeading};
use crate::parser::util::char_m_n;
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    character::complete::{char, space0, space1},
    combinator::{opt, value},
    multi::many1,
    sequence::{preceded, terminated},
    IResult, Parser,
};
use std::rc::Rc;

/// Parse headings in format:
///      ### Header text
pub(crate) fn heading_v1<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Heading> {
    move |input: &'a str| {
        let to_space_or_not_to_space = if state.config.allow_no_space_in_headings {
            space0
        } else {
            space1
        };

        let (input, (prefix, _, content)) = (
            char_m_n(1, 6, '#'),
            to_space_or_not_to_space,
            line_terminated(not_eof_or_eol1),
        )
            .parse(input)?;

        let (_, content) = crate::parser::inline::inline_many0(state.clone()).parse(content)?;

        let heading = Heading {
            kind: HeadingKind::Atx(prefix.len() as u8),
            content,
        };

        Ok((input, heading))
    }
}

/// Parse headings in format:
///      Heading text
///      ====
pub(crate) fn heading_v2_or_paragraph<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Block> {
    move |input: &'a str| {
        let (input, (content, level)) = (
            crate::parser::blocks::paragraph::paragraph(state.clone(), true),
            opt(heading_v2_level(state.clone())),
        )
            .parse(input)?;

        if let Some(level) = level {
            let heading = Heading {
                kind: HeadingKind::Setext(level),
                content,
            };
            return Ok((input, Block::Heading(heading)));
        }

        Ok((input, Block::Paragraph(content)))
    }
}

pub(crate) fn heading_v2_level<'a>(
    _state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, SetextHeading> {
    move |input: &'a str| {
        let setext_parser = alt((
            value(SetextHeading::Level1, many1(char('='))),
            value(SetextHeading::Level2, many1(char('-'))),
        ));

        let r = line_terminated(preceded(
            char_m_n(0, 3, ' '),
            terminated(setext_parser, space0),
        ))
        .parse(input)?;

        Ok(r)
    }
}
