use crate::ast::{ListBulletKind, ListItem, ListKind, ListOrderedKindOptions, TaskState};
use crate::parser::util::char_m_n;
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::combinator::verify;
use nom::{
    branch::alt,
    character::complete::{char, one_of, space0},
    combinator::{map, not, opt, peek, recognize, value},
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};
use std::rc::Rc;

fn list_item_task_state(input: &str) -> IResult<&str, TaskState> {
    delimited(
        char('['),
        alt((
            value(TaskState::Complete, one_of("xX")),
            value(TaskState::Incomplete, char(' ')),
        )),
        char(']'),
    )
    .parse(input)
}

fn list_marker(input: &str) -> IResult<&str, ListKind> {
    alt((
        list_marker_ordered,
        list_marker_star,
        list_marker_plus,
        list_marker_dash,
    ))
    .parse(input)
}

fn list_marker_star(input: &str) -> IResult<&str, ListKind> {
    map(char('*'), |_| ListKind::Bullet(ListBulletKind::Star)).parse(input)
}

fn list_marker_plus(input: &str) -> IResult<&str, ListKind> {
    map(char('+'), |_| ListKind::Bullet(ListBulletKind::Plus)).parse(input)
}

fn list_marker_dash(input: &str) -> IResult<&str, ListKind> {
    map(char('-'), |_| ListKind::Bullet(ListBulletKind::Dash)).parse(input)
}

fn list_marker_ordered(input: &str) -> IResult<&str, ListKind> {
    map(
        terminated(nom::character::complete::u64, one_of(".)")),
        |start| ListKind::Ordered(ListOrderedKindOptions { start }),
    )
    .parse(input)
}

/// The marker of a list item and what follows it on the line: the list kind, the
/// content indentation (marker width plus the spaces after it), the task state and
/// the first line of content (empty when the marker ends the line).
pub(crate) fn list_marker_with_span_size(
    input: &str,
) -> IResult<&str, (ListKind, usize, Option<TaskState>, &str)> {
    let (after_marker, kind) = preceded(char_m_n(0, 3, ' '), list_marker).parse(input)?;

    // Marker alone on its line:
    // 1.
    // 1.____
    if let Ok((tail, _)) = line_terminated(space0).parse(after_marker) {
        let consumed = input.len() - after_marker.len() + 1;
        return Ok((tail, (kind, consumed, None, "")));
    }

    // Marker followed by a task box alone on its line: `- [ ]`
    if let Ok((remaining, _)) = char_m_n(0, 3, ' ').parse(after_marker) {
        if let Ok((remaining, task_state)) = line_terminated(list_item_task_state).parse(remaining)
        {
            let consumed = input.len() - after_marker.len() + 1;
            return Ok((remaining, (kind, consumed, Some(task_state), "")));
        }
    }

    // Marker, one to four spaces, then content.
    let (remaining, _) = char_m_n(1, 4, ' ').parse(after_marker)?;
    let consumed = input.len() - remaining.len();
    let (remaining, task_state) =
        opt(terminated(list_item_task_state, char(' '))).parse(remaining)?;
    let (remaining, first_line) = line_terminated(not_eof_or_eol0).parse(remaining)?;

    Ok((remaining, (kind, consumed, task_state, first_line)))
}

/// One continuation line of a list item, as the pieces to append after a `\n`:
/// the blank lines that precede it (possibly empty) and the line itself.
fn list_item_rest_line(
    state: Rc<MarkdownParserState>,
    list_kind: ListKind,
    prefix_length: usize,
) -> impl FnMut(&str) -> IResult<&str, (&str, &str)> {
    move |input: &str| {
        // Stop parsing lines on EOF
        if input.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Eof,
            )));
        }

        let marker_parser = match list_kind {
            ListKind::Ordered(_) => list_marker_ordered,
            ListKind::Bullet(ListBulletKind::Star) => list_marker_star,
            ListKind::Bullet(ListBulletKind::Plus) => list_marker_plus,
            ListKind::Bullet(ListBulletKind::Dash) => list_marker_dash,
        };

        line_terminated(preceded(
            peek(not(alt((
                value(
                    (),
                    crate::parser::blocks::thematic_break::thematic_break(state.clone()),
                ),
                value(
                    (),
                    (
                        verify(
                            recognize(char_m_n(0, prefix_length, ' ')),
                            |indent: &str| indent.len() < prefix_length,
                        ),
                        marker_parser,
                    ),
                ),
            )))),
            alt((
                // If starts with 0 <= prefix_length spaces
                preceded(
                    char_m_n(0, prefix_length, ' '),
                    map(not_eof_or_eol1, |v| ("", v)),
                ),
                // If this is empty line, followed by prefix_length spaces
                (
                    recognize(many1(line_terminated(space0))),
                    preceded(char_m_n(prefix_length, prefix_length, ' '), not_eof_or_eol1),
                ),
            )),
        ))
        .parse(input)
    }
}

fn list_item_lines(
    state: Rc<MarkdownParserState>,
    list_kind: ListKind,
    prefix_length: usize,
) -> impl FnMut(&str) -> IResult<&str, Vec<(&str, &str)>> {
    move |input: &str| {
        many0(list_item_rest_line(
            state.clone(),
            list_kind.clone(),
            prefix_length,
        ))
        .parse(input)
    }
}

pub(crate) fn list_item(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&str) -> IResult<&str, (ListKind, ListItem)> {
    move |input: &str| {
        let (input, (list_kind, item_prefix_length, task_state, first_line)) =
            list_marker_with_span_size(input)?;

        let (input, rest_lines) =
            list_item_lines(state.clone(), list_kind.clone(), item_prefix_length).parse(input)?;

        // A one-line item is parsed in place; only a multi-line item is joined.
        let joined;
        let item_content: &str = if rest_lines.is_empty() {
            first_line
        } else {
            let total_size = first_line.len()
                + rest_lines
                    .iter()
                    .map(|(blank, line)| 1 + blank.len() + line.len())
                    .sum::<usize>();
            let mut buf = String::with_capacity(total_size);
            buf.push_str(first_line);
            for (blank, line) in rest_lines {
                buf.push('\n');
                buf.push_str(blank);
                buf.push_str(line);
            }
            joined = buf;
            &joined
        };

        let nested_state = Rc::new(state.nested());
        let (_, blocks) = crate::parser::blocks::blocks_many0(nested_state, item_content)
            .map_err(|err| err.map_input(|_| input))?;

        let item = ListItem {
            task: task_state,
            blocks,
        };
        Ok((input, (list_kind, item)))
    }
}

pub(crate) fn list(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&str) -> IResult<&str, crate::ast::List> {
    move |input: &str| {
        let (input, items) = many1(list_item(state.clone())).parse(input)?;

        // With many1(), first element always present
        let first_item = items.first().unwrap();

        let list = crate::ast::List {
            kind: first_item.0.clone(),
            items: items.into_iter().map(|(_, item)| item).collect(),
        };

        Ok((input, list))
    }
}
