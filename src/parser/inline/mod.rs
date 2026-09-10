mod autolink;
mod bracketed;
mod code_span;
mod emphasis;
mod environment_variable;
mod hard_newline;
mod html_entity;
mod image;
pub(crate) mod index;
mod strikethrough;
mod text;

#[cfg(test)]
mod tests;

use crate::ast::Inline;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    combinator::{fail, map},
    IResult, Parser,
};
use std::rc::Rc;

use super::util::{conditional_inline, Pieces};

/// Merges consecutive Text elements into a single Text element, in place.
fn merge_consecutive_text_elements(mut inlines: Vec<Inline>) -> Vec<Inline> {
    // Two-pointer compaction: `write` is the end of the merged prefix.
    let mut write = 0;
    for read in 0..inlines.len() {
        let mergeable = write > 0
            && matches!(inlines[read], Inline::Text(_))
            && matches!(inlines[write - 1], Inline::Text(_));
        if mergeable {
            let Inline::Text(text) = std::mem::replace(&mut inlines[read], Inline::Empty) else {
                unreachable!()
            };
            let Inline::Text(acc) = &mut inlines[write - 1] else {
                unreachable!()
            };
            acc.push_str(&text);
        } else {
            inlines.swap(write, read);
            write += 1;
        }
    }
    inlines.truncate(write);
    inlines
}

/// `many0`/`many1` of [`inline`], collecting the pieces straight into one vector
/// instead of a vector of vectors that is flattened afterwards.
fn inline_many<'a>(
    state: &Rc<MarkdownParserState>,
    input: &'a str,
    at_least_one: bool,
) -> IResult<&'a str, Vec<Inline>> {
    let mut parser = inline(state.clone());
    let mut out = Vec::new();
    let mut rest = input;
    loop {
        match parser(rest) {
            Ok((after, pieces)) => {
                if after.len() == rest.len() {
                    // A parser that consumed nothing would loop forever.
                    return Err(nom::Err::Error(nom::error::Error::new(
                        rest,
                        nom::error::ErrorKind::Many0,
                    )));
                }
                pieces.extend_into(&mut out);
                rest = after;
            }
            Err(nom::Err::Error(_)) => {
                if at_least_one && rest.len() == input.len() {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Many1,
                    )));
                }
                break;
            }
            Err(err) => return Err(err),
        }
    }
    Ok((rest, out))
}

/// Fast path for a slice that is plain text from end to end: no byte that could
/// start an inline element, no hard line break and no leading backslash escape.
///
/// Every alternative of [`inline`] other than `text` fails on such input, and `text`
/// consumes all of it, so the result is a single [`Inline::Text`] and neither the
/// delimiter index nor the alternatives need to be built. Not applicable when a
/// custom inline parser is installed (it may accept anything) or when text is
/// handled by a non-default behavior.
fn plain_text_only<'a>(
    state: &MarkdownParserState,
    input: &'a str,
) -> Option<IResult<&'a str, Vec<Inline>>> {
    if state.config.custom_inline_parser.is_some()
        || !matches!(
            state.config.inline_text_behavior,
            crate::parser::config::ElementBehavior::Parse
        )
        || input.starts_with('\\')
    {
        return None;
    }
    if let Err(err) = state.check_depth(input) {
        return Some(Err(err));
    }
    match text::plain_run(input) {
        Ok((rest, run)) if rest.is_empty() => Some(Ok((rest, vec![Inline::Text(run.to_string())]))),
        _ => None,
    }
}

pub(crate) fn inline_many0<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Inline>> {
    move |input: &'a str| {
        if let Some(result) = plain_text_only(&state, input) {
            return result;
        }
        let (input, inlines) =
            with_index(&state, input, |input| inline_many(&state, input, false))?;
        Ok((input, merge_consecutive_text_elements(inlines)))
    }
}

pub(crate) fn inline_many1<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Inline>> {
    move |input: &'a str| {
        if let Some(result) = plain_text_only(&state, input) {
            return result;
        }
        let (input, inlines) = with_index(&state, input, |input| inline_many(&state, input, true))?;
        Ok((input, merge_consecutive_text_elements(inlines)))
    }
}

/// Run `f` with the delimiter index of `input` installed in `state`, restoring the
/// previous index afterwards. Nested inline content is parsed with a deeper state,
/// so an index is never clobbered while in use.
fn with_index<'a, T>(
    state: &MarkdownParserState,
    input: &'a str,
    f: impl FnOnce(&'a str) -> T,
) -> T {
    let index = Rc::new(index::InlineIndex::build(input));
    let previous = state.inline_index.replace(Some(index));
    let result = f(input);
    state.inline_index.replace(previous);
    result
}

pub(crate) fn inline<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Pieces<Inline>> {
    // Built once per `inline_many` slice rather than at every input position: the
    // alternatives capture clones of the state and of the configured behaviors.
    let mut parser = alt((
        conditional_inline(
            state.config.inline_autolink_behavior.clone(),
            map(crate::parser::inline::autolink::autolink, Inline::Autolink),
        ),
        // Inline links, footnote references and reference links share `[label]`;
        // behaviors of the three are applied inside.
        crate::parser::inline::bracketed::bracketed_element(state.clone()),
        conditional_inline(
            state.config.inline_hard_newline_behavior.clone(),
            crate::parser::inline::hard_newline::hard_newline,
        ),
        conditional_inline(
            state.config.inline_image_behavior.clone(),
            crate::parser::inline::image::image(state.clone()),
        ),
        conditional_inline(
            state.config.inline_code_span_behavior.clone(),
            map(crate::parser::inline::code_span::code_span, Inline::Code),
        ),
        map(
            crate::parser::inline::environment_variable::environment_variable,
            Pieces::One,
        ),
        conditional_inline(
            state.config.inline_emphasis_behavior.clone(),
            crate::parser::inline::emphasis::emphasis(state.clone()),
        ),
        conditional_inline(
            state.config.inline_strikethrough_behavior.clone(),
            crate::parser::inline::strikethrough::strikethrough(state.clone()),
        ),
        custom_parser(state.clone()),
        conditional_inline(
            state.config.inline_text_behavior.clone(),
            crate::parser::inline::text::text(state.clone()),
        ),
        // A character that may start an element but did not: literal text.
        conditional_inline(
            state.config.inline_text_behavior.clone(),
            crate::parser::inline::text::literal_char,
        ),
    ));
    move |input: &'a str| {
        state.check_depth(input)?;
        parser.parse(input)
    }
}

fn custom_parser(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&str) -> IResult<&str, Pieces<Inline>> {
    move |input: &str| {
        if let Some(custom_parser) = state.config.custom_inline_parser.as_ref() {
            let mut p = (**custom_parser).borrow_mut();
            (p.as_mut())(input).map(|(rest, inlines)| (rest, Pieces::Many(inlines)))
        } else {
            fail().parse(input)
        }
    }
}
