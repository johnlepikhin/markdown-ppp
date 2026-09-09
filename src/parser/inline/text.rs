use crate::ast::Inline;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    character::complete::{anychar, char, one_of},
    combinator::{map, opt},
    multi::many1,
    sequence::preceded,
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn text<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Inline> {
    move |input: &'a str| {
        map(
            many1(alt((
                map(escaped_char, |c| c.to_string()),
                map(
                    crate::parser::inline::html_entity::html_entity(state.clone()),
                    |c| c.to_string(),
                ),
                map(plain_run, |c| c.to_string()),
            ))),
            |vec| Inline::Text(vec.join("")),
        )
        .parse(input)
    }
}

/// A character that may start an inline element but did not, consumed as literal text
/// together with the plain run that follows it.
///
/// Used as the last alternative of the inline parser. The following run is attached
/// here (instead of being left to the next `text` call) so that backslash escapes
/// are only interpreted at the start of a text element, exactly as before the
/// lookahead-free rewrite. Consecutive text elements are merged afterwards.
pub(crate) fn literal_char(input: &str) -> IResult<&str, Inline> {
    map((anychar, opt(plain_run)), |(c, run)| {
        let mut s = c.to_string();
        s.push_str(run.unwrap_or_default());
        Inline::Text(s)
    })
    .parse(input)
}

/// Characters that may start an inline element other than text.
///
/// The run of plain text stops at these positions so that the inline parser gets a
/// chance to match an element there. The set must contain the first character of
/// every inline element parser: `&` (entity), `<` (autolink), `[` (links, footnote
/// reference), `!` (image), `` ` `` (code span), `*` / `_` (emphasis) and `~`
/// (strikethrough). A backslash is only special before a line ending (hard line
/// break); elsewhere it stays in the run, see [`literal_char`].
fn is_special(c: char) -> bool {
    matches!(c, '&' | '<' | '[' | '!' | '`' | '*' | '_' | '~')
}

fn is_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Longest identifier the `environment_variable` parser would accept; longer ones are
/// rejected by it anyway, so skipping them keeps the scan linear.
const MAX_ENV_VAR_LEN: usize = 50;

/// The longest run of plain text starting at `input`.
///
/// This is a single linear pass instead of a full lookahead of every inline parser at
/// every character (which made the parse time exponential in the nesting depth):
///
/// - stops at [`is_special`] characters;
/// - stops at a backslash or a space that starts a hard line break;
/// - an identifier that looks like an environment variable (`FOO_BAR`) is consumed
///   whole, so that its underscores are not taken as emphasis markers. This mirrors
///   the `environment_variable` parser, which produces plain text anyway.
fn plain_run(input: &str) -> IResult<&str, &str> {
    let mut pos = 0;

    while pos < input.len() {
        let rest = &input[pos..];
        let c = rest.chars().next().unwrap();

        if is_special(c) {
            break;
        }

        if (c == ' ' || c == '\\') && is_hard_break_start(rest) {
            break;
        }

        if c.is_ascii_alphabetic() && identifier_fits(rest) {
            if let Ok((after, _)) =
                crate::parser::inline::environment_variable::environment_variable(rest)
            {
                pos += rest.len() - after.len();
                continue;
            }
        }

        pos += c.len_utf8();
    }

    if pos == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeWhile1,
        )));
    }

    Ok((&input[pos..], &input[..pos]))
}

/// Whether the `[A-Za-z0-9_]+` word starting at `rest` is short enough to be an
/// environment variable. Looks at a bounded window so that a long word is not
/// rescanned from every one of its characters.
fn identifier_fits(rest: &str) -> bool {
    rest.bytes()
        .take(MAX_ENV_VAR_LEN + 1)
        .position(|b| !is_word_char(b as char))
        .is_some_and(|len| len <= MAX_ENV_VAR_LEN)
        || rest.len() <= MAX_ENV_VAR_LEN
}

/// `\` or two or more spaces, followed by a line ending (see `hard_newline`).
fn is_hard_break_start(rest: &str) -> bool {
    let after = if let Some(after) = rest.strip_prefix('\\') {
        after
    } else {
        let trimmed = rest.trim_start_matches(' ');
        if rest.len() - trimmed.len() < 2 {
            return false;
        }
        trimmed
    };
    after.starts_with('\n') || after.starts_with("\r\n")
}

fn escaped_char(input: &str) -> IResult<&str, char> {
    preceded(char('\\'), one_of("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~")).parse(input)
}
