use crate::parser::link_util::{link_title, unescape_punctuation};
use crate::parser::MarkdownParserState;
use crate::{
    ast::{Image, Inline},
    parser::link_util::link_destination,
};
use nom::{
    bytes::complete::tag,
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
            None => match unescaped_close_bracket(input) {
                Some(close) => (&input[close + 1..], &input[..close]),
                None => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Char,
                    )))
                }
            },
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
                alt: unescape_punctuation(alt),
            }),
        ))
    }
}

/// Byte offset of the first `]` in `input` that is not preceded by a backslash escape,
/// treating `\x` as an opaque pair. Mirrors what [`InlineIndex`] records, so both paths
/// of the alt scan agree on `![a\](x)](u.jpg)`.
///
/// [`InlineIndex`]: crate::parser::inline::index::InlineIndex
pub(crate) fn unescaped_close_bracket(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => i += 1 + input[i + 1..].chars().next().map_or(0, char::len_utf8),
            b']' => return Some(i),
            _ => i += 1,
        }
    }
    None
}
