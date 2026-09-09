use crate::ast::Inline;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value, verify},
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn emphasis(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&str) -> IResult<&str, Inline> {
    move |input: &str| {
        alt((
            map(
                alt((
                    delimited_by(state.clone(), "***"),
                    delimited_by(state.clone(), "___"),
                )),
                |inner| Inline::Strong(vec![Inline::Emphasis(inner)]),
            ),
            map(
                alt((
                    delimited_by(state.clone(), "**"),
                    delimited_by(state.clone(), "__"),
                )),
                Inline::Strong,
            ),
            map(
                alt((
                    delimited_by(state.clone(), "*"),
                    delimited_by(state.clone(), "_"),
                )),
                Inline::Emphasis,
            ),
        ))
        .parse(input)
    }
}

/// `<tag>content<tag>` where the content is parsed as inline elements.
fn delimited_by<'a>(
    state: Rc<MarkdownParserState>,
    tag_value: &'static str,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Inline>> {
    move |input: &'a str| {
        let (after_open, _) = open_tag(tag_value).parse(input)?;

        // Locate the closing tag *before* recursing into the content. Without this
        // check an unclosed opener would trigger a full recursive parse of the
        // remaining input at every position, which is exponential.
        let indexed = state
            .inline_index
            .borrow()
            .as_ref()
            .and_then(|index| index.emphasis_content_len(after_open, tag_value));
        let content_len = indexed
            .unwrap_or_else(|| find_close_tag(after_open, tag_value))
            .ok_or_else(|| {
                nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag))
            })?;
        let content = &after_open[..content_len];
        let rest = &after_open[content_len + tag_value.len()..];

        let nested_state = Rc::new(state.deeper());
        let (_, inner) = crate::parser::inline::inline_many1(nested_state)
            .parse(content)
            .map_err(|err| err.map_input(|_| input))?;

        Ok((rest, inner))
    }
}

/// Byte offset of the first closing `tag_value` in `input` that is allowed to close
/// emphasis (see [`can_close`]), or `None`. A closer at offset 0 means empty content,
/// which is not an emphasis.
///
/// A `*` immediately preceded by a backslash is an escaped star and never closes
/// (`_` is not treated this way, matching the historical behaviour).
///
/// Linear scan; used only when the input is not covered by an `InlineIndex`.
fn find_close_tag(input: &str, tag_value: &str) -> Option<usize> {
    let marker = tag_value.chars().next().unwrap();
    let mut from = 0;
    while let Some(found) = input[from..].find(tag_value) {
        let idx = from + found;
        let escaped = marker == '*' && idx > 0 && input.as_bytes()[idx - 1] == b'\\';
        if !escaped {
            let next = input[idx + tag_value.len()..].chars().next();
            if can_close(marker, next) {
                return if idx > 0 { Some(idx) } else { None };
            }
        }
        from = idx + marker.len_utf8();
    }
    None
}

fn open_tag(tag_value: &'static str) -> impl FnMut(&str) -> IResult<&str, ()> {
    move |input: &str| {
        value(
            (),
            verify(tag(tag_value), |v: &str| {
                can_open(v.chars().next().unwrap(), input.chars().nth(v.len()))
            }),
        )
        .parse(input)
    }
}

fn can_open(marker: char, next: Option<char>) -> bool {
    let left_flanking = next.is_some_and(|c| !c.is_whitespace())
        && (next.is_some_and(|c| !is_punctuation(c)) || (next.is_some_and(is_punctuation)));
    if !left_flanking {
        return false;
    }
    if marker == '_' {
        let right_flanking = next.is_none_or(|c| c.is_whitespace() || is_punctuation(c));
        return !right_flanking;
    }
    true
}

pub(crate) fn can_close(marker: char, next: Option<char>) -> bool {
    let right_flanking = next.is_none_or(|c| c.is_whitespace() || is_punctuation(c));
    if !right_flanking {
        return false;
    }

    if marker == '_' {
        let left_flanking = next.is_some_and(|c| !c.is_whitespace())
            && (next.is_some_and(|c| !is_punctuation(c)))
            || (next.is_some_and(is_punctuation));
        return !left_flanking || next.is_some_and(is_punctuation);
    }
    true
}

fn is_punctuation(c: char) -> bool {
    use unicode_categories::UnicodeCategories;
    c.is_ascii_punctuation() || c.is_punctuation()
}
