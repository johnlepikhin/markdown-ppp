use crate::ast::{Block, GitHubAlert, GitHubAlertType};
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, satisfy},
    combinator::{opt, recognize},
    multi::{many0, many1, many_m_n},
    sequence::{delimited, pair, preceded},
    IResult, Parser,
};
use std::rc::Rc;

/// Parse custom alert name (must start with letter, contain only letters, digits, underscores)
fn parse_custom_alert_name(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alpha1,
        many0(satisfy(|c| c.is_ascii_alphanumeric() || c == '_')),
    ))
    .parse(input)
}

/// Parse alert type from marker text (e.g., "[!NOTE]" -> Some(Note))
fn parse_alert_marker(marker: &str) -> Option<GitHubAlertType> {
    let trimmed = marker.trim().to_uppercase();

    let mut parser = delimited(
        tag("[!"),
        alt((
            tag("NOTE").map(|_| GitHubAlertType::Note),
            tag("TIP").map(|_| GitHubAlertType::Tip),
            tag("IMPORTANT").map(|_| GitHubAlertType::Important),
            tag("WARNING").map(|_| GitHubAlertType::Warning),
            tag("CAUTION").map(|_| GitHubAlertType::Caution),
            parse_custom_alert_name.map(|name| GitHubAlertType::Custom(name.to_string())),
        )),
        tag("]"),
    );

    match parser.parse(&trimmed) {
        Ok(("", alert_type)) => Some(alert_type),
        _ => None,
    }
}

pub(crate) fn github_alert<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Block>> {
    move |input: &'a str| {
        // Try to parse as a blockquote first
        let prefix = preceded(many_m_n(0, 3, char(' ')), char('>'));

        // Peek at the first line to check if it starts with an alert marker
        let (_remaining, first_line) =
            preceded((prefix, opt(char(' '))), line_terminated(not_eof_or_eol0)).parse(input)?;

        // Check if the first line contains a GitHub alert marker
        let alert_type = if let Some(alert_type) = parse_alert_marker(first_line.trim()) {
            alert_type
        } else {
            // Not a GitHub alert, fail to let regular blockquote parser handle it
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        };

        // Now parse the rest of the blockquote lines
        let prefix = preceded(many_m_n(0, 3, char(' ')), char('>'));
        let (input, mut lines) =
            many1(preceded(prefix, line_terminated(not_eof_or_eol0))).parse(input)?;

        // Remove the first line (alert marker) and join the rest
        lines.remove(0); // Remove the alert marker line completely
        let inner = lines.join("\n");

        // Parse the inner content as blocks
        let (_, blocks) = if !inner.is_empty() {
            many1(crate::parser::blocks::block(state.clone()))
                .parse(&inner)
                .map_err(|err| err.map_input(|_| input))?
        } else {
            ("", vec![])
        };

        let blocks = blocks.into_iter().flatten().collect();

        Ok((
            input,
            vec![Block::GitHubAlert(GitHubAlert { alert_type, blocks })],
        ))
    }
}
