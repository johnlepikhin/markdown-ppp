use nom::character::complete::{anychar, char, none_of, one_of, satisfy};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, not, peek, recognize, value, verify},
    multi::{fold_many0, many0, many1},
    sequence::{delimited, preceded},
    IResult, Parser,
};
use std::rc::Rc;

use super::MarkdownParserState;

pub(crate) fn link_label<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<crate::ast::Inline>> {
    move |input: &'a str| {
        delimited(tag("["), link_label_inner(state.clone()), tag("]")).parse(input)
    }
}

fn link_label_inner<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<crate::ast::Inline>> {
    move |input: &'a str| {
        let (input, label_chars) = verify(
            many1(preceded(
                peek(not(char(']'))),
                alt((value(']', tag("\\]")), anychar)),
            )),
            |chars: &[char]| chars.iter().any(|&c| c != ' ' && c != '\n') && chars.len() < 1000,
        )
        .parse(input)?;

        let label = label_chars.iter().collect::<String>();

        let (_, label) = crate::parser::inline::inline_many1(state.clone())
            .parse(label.as_str())
            .map_err(|err| err.map_input(|_| input))?;

        Ok((input, label))
    }
}

pub(crate) fn link_title(input: &str) -> IResult<&str, String> {
    alt((
        link_title_double_quoted,
        link_title_single_quoted,
        link_title_parenthesized,
    ))
    .parse(input)
}

fn link_title_parenthesized(input: &str) -> IResult<&str, String> {
    delimited(char('('), link_title_inner(')'), char(')')).parse(input)
}

fn link_title_single_quoted(input: &str) -> IResult<&str, String> {
    delimited(char('\''), link_title_inner('\''), char('\'')).parse(input)
}

fn link_title_double_quoted(input: &str) -> IResult<&str, String> {
    delimited(tag("\""), link_title_inner('"'), tag("\"")).parse(input)
}

fn link_title_inner(end_delim: char) -> impl FnMut(&str) -> IResult<&str, String> {
    move |input: &str| {
        fold_many0(
            alt((
                map(escaped_char, |c| c.to_string()),
                map(none_of(&[end_delim, '\\'][..]), |c| c.to_string()),
            )),
            String::new,
            |mut acc, s| {
                acc.push_str(&s);
                acc
            },
        )
        .parse(input)
    }
}

fn escaped_char(input: &str) -> IResult<&str, char> {
    preceded(tag("\\"), anychar).parse(input)
}

pub(crate) fn link_destination(input: &str) -> IResult<&str, String> {
    alt((link_destination1, link_destination2)).parse(input)
}

fn link_destination1(input: &str) -> IResult<&str, String> {
    let (input, _) = char('<').parse(input)?;

    let (input, chars) = many0(alt((
        preceded(char('\\'), one_of("<>")),
        preceded(peek(not(one_of("\n<>"))), anychar),
    )))
    .parse(input)?;
    let (input, _) = char('>').parse(input)?;

    let v: String = chars.iter().collect();

    Ok((input, v))
}

fn link_destination2(input: &str) -> IResult<&str, String> {
    let (input, _) = peek(satisfy(|c| is_valid_char(c) && c != '<')).parse(input)?;

    map(
        recognize(many1(alt((
            value((), escaped_char),
            value((), balanced_parens),
            value((), satisfy(|c| is_valid_char(c) && c != '(' && c != ')')),
        )))),
        |s: &str| s.to_string(),
    )
    .parse(input)
}

fn balanced_parens(input: &str) -> IResult<&str, String> {
    delimited(
        tag("("),
        map(
            fold_many0(
                alt((
                    map(escaped_char, |c| c.to_string()),
                    map(balanced_parens, |s| format!("({s})")),
                    map(satisfy(|c| is_valid_char(c) && c != '(' && c != ')'), |c| {
                        c.to_string()
                    }),
                )),
                String::new,
                |mut acc, item| {
                    acc.push_str(&item);
                    acc
                },
            ),
            |s| s,
        ),
        tag(")"),
    )
    .parse(input)
}

fn is_valid_char(c: char) -> bool {
    !c.is_ascii_control() && c != ' ' && c != '<'
}
