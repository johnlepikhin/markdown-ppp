use crate::ast::{Block, GitHubAlert, GitHubAlertType};
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    character::complete::char,
    combinator::opt,
    multi::{many1, many_m_n},
    sequence::preceded,
    IResult, Parser,
};
use std::rc::Rc;

/// Parse alert type from marker text (e.g., "[!NOTE]" -> Some(Note))
fn parse_alert_marker(marker: &str) -> Option<GitHubAlertType> {
    match marker.trim().to_uppercase().as_str() {
        "[!NOTE]" => Some(GitHubAlertType::Note),
        "[!TIP]" => Some(GitHubAlertType::Tip),
        "[!IMPORTANT]" => Some(GitHubAlertType::Important),
        "[!WARNING]" => Some(GitHubAlertType::Warning),
        "[!CAUTION]" => Some(GitHubAlertType::Caution),
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
