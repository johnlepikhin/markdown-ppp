use crate::parser::link_util::link_title;
use crate::parser::MarkdownParserState;
use crate::{
    ast::{Image, Inline},
    parser::link_util::link_destination,
};
use nom::{
    bytes::complete::take_while,
    character::complete::{char, multispace0},
    combinator::opt,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use std::rc::Rc;

// ![alt text](/url "title")
pub(crate) fn image<'a>(
    _state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Inline> {
    move |input: &'a str| {
        let (input, alt) = preceded(
            char('!'),
            delimited(char('['), take_while(|c| c != ']'), char(']')),
        )
        .parse(input)?;

        let (input, (destination, title)) = delimited(
            char('('),
            (
                preceded(multispace0, link_destination),
                opt(preceded(multispace0, link_title)),
            ),
            preceded(multispace0, char(')')),
        )
        .parse(input)?;

        Ok((
            input,
            Inline::Image(Image {
                destination,
                title,
                alt: alt.to_owned(),
            }),
        ))
    }
}
