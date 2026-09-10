use crate::ast::Inline;
use crate::parser::inline::environment_variable::{
    fits, is_likely_env_var, scan_word_full, MAX_ENV_VAR_LEN,
};
use crate::parser::MarkdownParserState;
use nom::{
    character::complete::{anychar, char, one_of},
    combinator::{map, opt},
    sequence::preceded,
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn text<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Inline> {
    let mut entity = crate::parser::inline::html_entity::html_entity(state);
    move |input: &'a str| {
        // Hand-rolled `many1(alt(...))` that appends into one buffer instead of
        // allocating a `String` per fragment and joining them afterwards.
        let mut out = String::new();
        let mut rest = input;
        loop {
            if let Ok((after, c)) = escaped_char(rest) {
                out.push(c);
                rest = after;
            } else if let Ok((after, s)) = entity(rest) {
                out.push_str(&s);
                rest = after;
            } else if let Ok((after, run)) = plain_run(rest) {
                out.push_str(run);
                rest = after;
            } else {
                break;
            }
        }
        if rest.len() == input.len() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Many1,
            )));
        }
        Ok((rest, Inline::Text(out)))
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

/// How [`plain_run`] treats a byte.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ByteClass {
    /// Never starts anything: consumed in bulk. Non-ASCII bytes are plain too.
    Plain,
    /// May start an inline element other than text.
    ///
    /// The run of plain text stops at these positions so that the inline parser
    /// gets a chance to match an element there. The set must contain the first
    /// character of every inline element parser: `&` (entity), `<` (autolink),
    /// `[` (links, footnote reference), `!` (image), `` ` `` (code span), `*` / `_`
    /// (emphasis) and `~` (strikethrough).
    Special,
    /// Space or backslash: special only when it starts a hard line break; a
    /// backslash elsewhere stays in the run, see [`literal_char`].
    HardBreak,
    /// ASCII letter: start of a word that may be an environment variable.
    Alpha,
}

const BYTE_CLASS: [ByteClass; 256] = {
    let mut t = [ByteClass::Plain; 256];
    let mut b = 0usize;
    while b < 256 {
        let c = b as u8;
        t[b] = match c {
            b'&' | b'<' | b'[' | b'!' | b'`' | b'*' | b'_' | b'~' => ByteClass::Special,
            b' ' | b'\\' => ByteClass::HardBreak,
            _ if c.is_ascii_alphabetic() => ByteClass::Alpha,
            _ => ByteClass::Plain,
        };
        b += 1;
    }
    t
};

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
pub(crate) fn plain_run(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();
    let mut pos = 0;

    while pos < bytes.len() {
        match BYTE_CLASS[bytes[pos] as usize] {
            ByteClass::Plain => {
                pos += 1;
                while pos < bytes.len() && BYTE_CLASS[bytes[pos] as usize] == ByteClass::Plain {
                    pos += 1;
                }
            }
            ByteClass::Special => break,
            ByteClass::HardBreak => {
                if is_hard_break_start(&input[pos..]) {
                    break;
                }
                pos += 1;
            }
            ByteClass::Alpha => {
                let rest = &input[pos..];
                let (word, underscore) = scan_word_full(rest);
                if !fits(word) {
                    // Too long for an env var from here. Only a suffix short enough
                    // can be one, so the bytes before it are plain text, except
                    // that a `_` among them stops the run (it may open emphasis).
                    // Jump there instead of re-scanning the word byte by byte.
                    let cut = word - MAX_ENV_VAR_LEN;
                    pos += match underscore {
                        Some(u) if u < cut => u.max(1),
                        _ => cut,
                    };
                    continue;
                }
                pos += match underscore {
                    // A word without `_` is plain text in full.
                    None => word,
                    Some(_) if is_likely_env_var(&rest[..word]) => word,
                    // Neither the word nor any of its suffixes is an env var (they
                    // keep its trailing or doubled underscores and are shorter), so
                    // nothing special can occur before its first `_`. Stop there so
                    // that emphasis still gets its chance.
                    Some(u) => u.max(1),
                };
            }
        }
    }

    if pos == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeWhile1,
        )));
    }

    Ok((&input[pos..], &input[..pos]))
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
