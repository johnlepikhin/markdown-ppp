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
    multi::{many0, many1},
    IResult, Parser,
};
use std::rc::Rc;

use super::util::conditional_inline;

/// Merges consecutive Text elements into a single Text element
fn merge_consecutive_text_elements(inlines: Vec<Inline>) -> Vec<Inline> {
    let mut result = Vec::new();
    let mut current_text = String::new();
    let mut has_text = false;

    for inline in inlines {
        match inline {
            Inline::Text(text) => {
                current_text.push_str(&text);
                has_text = true;
            }
            other => {
                // If we have accumulated text, add it to result
                if has_text {
                    result.push(Inline::Text(current_text.clone()));
                    current_text.clear();
                    has_text = false;
                }
                // Add the non-text element
                result.push(other);
            }
        }
    }

    // Don't forget the last accumulated text
    if has_text {
        result.push(Inline::Text(current_text));
    }

    result
}

pub(crate) fn inline_many0<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Inline>> {
    move |input: &'a str| {
        let (input, list_of_lists) = with_index(&state, input, |input| {
            many0(inline(state.clone())).parse(input)
        })?;
        let r: Vec<_> = list_of_lists.into_iter().flatten().collect();
        let merged = merge_consecutive_text_elements(r);
        Ok((input, merged))
    }
}

pub(crate) fn inline_many1<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Inline>> {
    move |input: &'a str| {
        let (input, list_of_lists) = with_index(&state, input, |input| {
            many1(inline(state.clone())).parse(input)
        })?;
        let r: Vec<_> = list_of_lists.into_iter().flatten().collect();
        let merged = merge_consecutive_text_elements(r);
        Ok((input, merged))
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
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Inline>> {
    move |input: &'a str| {
        state.check_depth(input)?;
        alt((
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
                |env_var| vec![env_var],
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
        ))
        .parse(input)
    }
}

fn custom_parser(state: Rc<MarkdownParserState>) -> impl FnMut(&str) -> IResult<&str, Vec<Inline>> {
    move |input: &str| {
        if let Some(custom_parser) = state.config.custom_inline_parser.as_ref() {
            let mut p = (**custom_parser).borrow_mut();
            (p.as_mut())(input)
        } else {
            fail().parse(input)
        }
    }
}
