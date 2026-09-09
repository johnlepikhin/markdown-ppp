use crate::parser::link_util::link_title;
use crate::parser::MarkdownParserState;
use crate::{
    ast::{Image, Inline},
    parser::link_util::link_destination,
};
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{char, multispace0},
    combinator::opt,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use std::rc::Rc;

// ![alt text](/url "title")
pub(crate) fn image<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Inline> {
    move |input: &'a str| {
        let (input, _) = tag("![").parse(input)?;

        // The alt text runs up to the first `]`. Look it up in the delimiter index
        // when there is one, so that a run of `![` is not scanned once per image.
        let indexed = state
            .inline_index
            .borrow()
            .as_ref()
            .and_then(|index| index.covers(input).then(|| index.next_close_bracket(input)));
        let (input, alt) = match indexed {
            Some(Some(close)) => (&input[close + 1..], &input[..close]),
            Some(None) => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Char,
                )))
            }
            None => (take_while(|c| c != ']'), char(']'))
                .parse(input)
                .map(|(i, (alt, _))| (i, alt))?,
        };

        let (input, (destination, title)) = delimited(
            char('('),
            (
                preceded(multispace0, link_destination(state.clone())),
                opt(preceded(multispace0, link_title(state.clone()))),
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
