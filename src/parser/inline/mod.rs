mod autolink;
mod code_span;
mod emphasis;
mod environment_variable;
mod footnote_reference;
mod hard_newline;
mod html_entity;
mod image;
mod inline_link;
mod reference_link;
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
        let (input, list_of_lists) = many0(inline(state.clone())).parse(input)?;
        let r: Vec<_> = list_of_lists.into_iter().flatten().collect();
        let merged = merge_consecutive_text_elements(r);
        Ok((input, merged))
    }
}

pub(crate) fn inline_many1<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Inline>> {
    move |input: &'a str| {
        let (input, list_of_lists) = many1(inline(state.clone())).parse(input)?;
        let r: Vec<_> = list_of_lists.into_iter().flatten().collect();
        let merged = merge_consecutive_text_elements(r);
        Ok((input, merged))
    }
}

pub(crate) fn inline<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Inline>> {
    move |input: &'a str| {
        alt((
            conditional_inline(
                state.config.inline_autolink_behavior.clone(),
                map(crate::parser::inline::autolink::autolink, Inline::Autolink),
            ),
            conditional_inline(
                state.config.inline_link_behavior.clone(),
                map(
                    crate::parser::inline::inline_link::inline_link(state.clone()),
                    Inline::Link,
                ),
            ),
            conditional_inline(
                state.config.inline_footnote_reference_behavior.clone(),
                crate::parser::inline::footnote_reference::footnote_reference,
            ),
            conditional_inline(
                state.config.inline_reference_link_behavior.clone(),
                crate::parser::inline::reference_link::reference_link(state.clone()),
            ),
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
