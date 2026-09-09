//! Inline elements that start with `[label]`: inline links, footnote references and
//! reference links.
//!
//! All of them share the same label syntax. Parsing them as independent alternatives
//! meant that the label (and, recursively, its inline content) was parsed up to four
//! times per bracket, which is exponential in the bracket nesting depth. Here the label
//! is scanned once and its content is parsed lazily at most once, after which the
//! variants are tried in the historical order with their respective `ElementBehavior`.

use crate::ast::{Inline, Link, LinkReference};
use crate::parser::link_util::{
    link_destination, link_label_content, link_label_raw, link_title, MAX_LINK_LABEL_DEPTH,
};
use crate::parser::util::conditional_inline;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0},
    combinator::opt,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn bracketed_element<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Inline>> {
    move |input: &'a str| {
        // Past the label nesting limit only a footnote reference can match; skip the
        // balanced-bracket scan, which would otherwise be repeated for every `[`.
        if state.link_label_depth >= MAX_LINK_LABEL_DEPTH {
            let result = conditional_inline(
                state.config.inline_footnote_reference_behavior.clone(),
                footnote_reference_cheap,
            )
            .parse(input);
            return result;
        }

        let (rest, raw) = link_label_raw(&state, input)?;

        // Inline content of the label, parsed on first use and shared by the variants.
        let label_cache: RefCell<Option<Vec<Inline>>> = RefCell::new(None);
        let label = || -> Result<Vec<Inline>, nom::Err<nom::error::Error<&'a str>>> {
            if let Some(label) = label_cache.borrow().as_ref() {
                return Ok(label.clone());
            }
            let label = link_label_content(state.clone(), &raw, input)?;
            *label_cache.borrow_mut() = Some(label.clone());
            Ok(label)
        };

        // [text](destination "title")
        let inline_link = |_: &'a str| {
            let children = label()?;
            let (rest, (destination, title)) = delimited(
                char('('),
                (
                    preceded(multispace0, link_destination(state.clone())),
                    opt(preceded(multispace0, link_title(state.clone()))),
                ),
                preceded(multispace0, char(')')),
            )
            .parse(rest)?;
            Ok((
                rest,
                Inline::Link(Link {
                    destination,
                    title,
                    children,
                }),
            ))
        };

        // [^label]
        let footnote_reference = |_: &'a str| {
            let name = raw.strip_prefix('^').ok_or_else(|| error(input))?;
            if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric()) {
                return Err(error(input));
            }
            Ok((rest, Inline::FootnoteReference(name.to_owned())))
        };

        // [text][label]
        let reference_full = |_: &'a str| {
            let text = label()?;
            let (rest, label_raw) = link_label_raw(&state, rest)?;
            let label = link_label_content(state.clone(), &label_raw, input)?;
            Ok((rest, Inline::LinkReference(LinkReference { label, text })))
        };

        // [label][]
        let reference_collapsed = |_: &'a str| {
            let text = label()?;
            let (rest, _) = tag("[]").parse(rest)?;
            Ok((
                rest,
                Inline::LinkReference(LinkReference {
                    label: text.clone(),
                    text,
                }),
            ))
        };

        // [label]
        let reference_shortcut = |_: &'a str| {
            let text = label()?;
            Ok((
                rest,
                Inline::LinkReference(LinkReference {
                    label: text.clone(),
                    text,
                }),
            ))
        };

        let result = alt((
            conditional_inline(state.config.inline_link_behavior.clone(), inline_link),
            conditional_inline(
                state.config.inline_footnote_reference_behavior.clone(),
                footnote_reference,
            ),
            conditional_inline(
                state.config.inline_reference_link_behavior.clone(),
                alt((reference_full, reference_collapsed, reference_shortcut)),
            ),
        ))
        .parse(input);
        result
    }
}

/// `[^name]` matched directly on the input, without a balanced-bracket scan.
fn footnote_reference_cheap(input: &str) -> IResult<&str, Inline> {
    let body = input.strip_prefix("[^").ok_or_else(|| error(input))?;
    let name_len = body
        .find(|c: char| !c.is_ascii_alphanumeric())
        .unwrap_or(body.len());
    if name_len == 0 || !body[name_len..].starts_with(']') {
        return Err(error(input));
    }
    Ok((
        &body[name_len + 1..],
        Inline::FootnoteReference(body[..name_len].to_owned()),
    ))
}

fn error(input: &str) -> nom::Err<nom::error::Error<&str>> {
    nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag))
}
