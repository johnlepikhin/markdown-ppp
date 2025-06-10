use crate::ast::{ListBulletKind, ListItem, ListKind, ListOrderedKindOptions, TaskState};
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::combinator::verify;
use nom::{
    branch::alt,
    character::complete::{char, one_of, space0},
    combinator::{map, not, opt, peek, recognize, value},
    multi::{many0, many1, many_m_n},
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

fn list_marker_followed_by_spaces(
    input: &str,
) -> IResult<&str, (ListKind, usize, Option<TaskState>)> {
    let (remaining, kind) = delimited(
        many_m_n(0, 3, char(' ')),
        list_marker,
        many_m_n(1, 4, char(' ')),
    )
    .parse(input)?;

    let consumed = input.len() - remaining.len();

    let (input, task_state) = opt(terminated(list_item_task_state, char(' '))).parse(remaining)?;

    Ok((input, (kind, consumed, task_state)))
}

fn list_marker_followed_by_newline(
    input: &str,
) -> IResult<&str, (ListKind, usize, Option<TaskState>)> {
    let (remaining, kind) = preceded(many_m_n(0, 3, char(' ')), list_marker).parse(input)?;

    // Cases:
    // 1.
    // 1.____
    if let Ok((tail, _)) = line_terminated(space0).parse(remaining) {
        // Calculate prefix length: consumed + 1 space
        let consumed = input.len() - remaining.len() + 1;

        return Ok((tail, (kind, consumed, None)));
    }

    let (remaining, _) = many_m_n(0, 3, char(' ')).parse(remaining)?;
    let consumed = input.len() - remaining.len() + 1;

    let (remaining, task_state) = line_terminated(list_item_task_state).parse(remaining)?;

    Ok((remaining, (kind, consumed, Some(task_state))))
}

pub(crate) fn list_marker_with_span_size(
    input: &str,
) -> IResult<&str, (ListKind, usize, Option<TaskState>, String)> {
    alt((
        map(
            list_marker_followed_by_newline,
            |(list_kind, prefix_length, task_state)| {
                (list_kind, prefix_length, task_state, String::new())
            },
        ),
        (map(
            (
                list_marker_followed_by_spaces,
                line_terminated(not_eof_or_eol0),
            ),
            |((list_kind, prefix_length, task_state), s)| {
                (list_kind, prefix_length, task_state, s.to_string())
            },
        )),
    ))
    .parse(input)
}

fn list_item_rest_line(
    state: Rc<MarkdownParserState>,
    list_kind: ListKind,
    prefix_length: usize,
) -> impl FnMut(&str) -> IResult<&str, Vec<&str>> {
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
                            recognize(many_m_n(0, prefix_length, char(' '))),
                            |indent: &str| indent.len() < prefix_length,
                        ),
                        marker_parser,
                    ),
                ),
            )))),
            alt((
                // If starts with 0 <= prefix_length spaces
                preceded(
                    many_m_n(0, prefix_length, char(' ')),
                    map(not_eof_or_eol1, |v| vec![v]),
                ),
                // If this is empty line, followed by prefix_length spaces
                map(
                    (
                        recognize(many1(line_terminated(space0))),
                        preceded(
                            many_m_n(prefix_length, prefix_length, char(' ')),
                            not_eof_or_eol1,
                        ),
                    ),
                    |(newlines, content)| vec![newlines, content],
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
) -> impl FnMut(&str) -> IResult<&str, Vec<Vec<&str>>> {
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

        let total_size = first_line.len() + rest_lines.len();
        let mut item_content = String::with_capacity(total_size);
        if !first_line.is_empty() {
            item_content.push_str(&first_line)
        }
        for line in rest_lines {
            item_content.push('\n');
            for subline in line {
                item_content.push_str(subline)
            }
        }

        let (_, blocks) = many0(crate::parser::blocks::block(state.clone()))
            .parse(&item_content)
            .map_err(|err| err.map_input(|_| input))?;

        let blocks = blocks.into_iter().flatten().collect();

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
