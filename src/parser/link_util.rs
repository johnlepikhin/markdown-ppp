use nom::character::complete::{anychar, char, one_of};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{not, peek},
    multi::many0,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use std::borrow::Cow;
use std::rc::Rc;

use super::MarkdownParserState;

/// `[label]` with balanced nested brackets, without parsing the label content.
///
/// Returns the raw label text (escapes of `]` resolved). Uses the delimiter index of
/// the current inline slice when there is one (O(1)); otherwise falls back to a
/// recursive scan, which is linear in the input.
pub(crate) fn link_label_raw<'a>(
    state: &MarkdownParserState,
    input: &'a str,
) -> IResult<&'a str, Cow<'a, str>> {
    if !input.starts_with('[') {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let indexed = state
        .inline_index
        .borrow()
        .as_ref()
        .and_then(|index| index.covers(input).then(|| index.bracket_match(input)));
    match indexed {
        Some(Some(close)) => Ok((&input[close + 1..], unescape_label(&input[1..close]))),
        Some(None) => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ))),
        None => delimited(tag("["), balanced_brackets_content, tag("]")).parse(input),
    }
}

/// Label text between the brackets, with `\]` resolved to `]` and every other escape
/// pair kept verbatim (the same result `balanced_brackets_content` produces).
fn unescape_label(raw: &str) -> Cow<'_, str> {
    if !raw.contains('\\') {
        return Cow::Borrowed(raw);
    }
    let mut out = String::with_capacity(raw.len());
    let mut rest = raw;
    while let Some(pos) = rest.find('\\') {
        out.push_str(&rest[..pos]);
        let after = &rest[pos + 1..];
        match after.chars().next() {
            Some(']') => {
                out.push(']');
                rest = &after[1..];
            }
            Some(c) => {
                out.push('\\');
                out.push(c);
                rest = &after[c.len_utf8()..];
            }
            None => {
                out.push('\\');
                rest = "";
            }
        }
    }
    out.push_str(rest);
    Cow::Owned(out)
}

/// Parse the raw label text (from [`link_label_raw`]) as inline elements.
///
/// `outer` is the input the label was taken from; errors are reported against it
/// because `raw` does not outlive this call.
pub(crate) fn link_label_content<'a>(
    state: Rc<MarkdownParserState>,
    raw: &str,
    outer: &'a str,
) -> Result<Vec<crate::ast::Inline>, nom::Err<nom::error::Error<&'a str>>> {
    if !(raw.chars().any(|c| c != ' ' && c != '\n') && raw.len() < 1000)
        || state.link_label_depth >= MAX_LINK_LABEL_DEPTH
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            outer,
            nom::error::ErrorKind::Verify,
        )));
    }

    // Recursively parse the label content as inline elements
    let nested_state = Rc::new(state.deeper_link_label());
    let (_, label) = crate::parser::inline::inline_many1(nested_state)
        .parse(raw)
        .map_err(|err| err.map_input(|_| outer))?;

    Ok(label)
}

/// Link title: `"..."`, `'...'` or `(...)`, with backslash escapes resolved.
///
/// The closing delimiter comes from the delimiter index when the input is covered by
/// one; otherwise a linear scan is used. Without the index every `(` that is not a
/// title would be scanned up to the end of the paragraph.
pub(crate) fn link_title<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, String> {
    move |input: &'a str| {
        let error = || nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Char));
        let end_delim = match input.as_bytes().first() {
            Some(b'"') => b'"',
            Some(b'\'') => b'\'',
            Some(b'(') => b')',
            _ => return Err(error()),
        };
        let indexed = state
            .inline_index
            .borrow()
            .as_ref()
            .and_then(|index| index.title_end(input, end_delim));
        let close = match indexed {
            Some(found) => found,
            None => title_end_slow(input, end_delim),
        }
        .ok_or_else(error)?;
        let title = unescape_punctuation(&input[1..close]);
        Ok((&input[close + 1..], title))
    }
}

/// Offset of the first unescaped `delim` after the opening delimiter at offset 0.
fn title_end_slow(input: &str, delim: u8) -> Option<usize> {
    let bytes = input.as_bytes();
    let mut i = 1;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => i += 1 + input[i + 1..].chars().next().map_or(0, char::len_utf8),
            b if b == delim => return Some(i),
            _ => i += 1,
        }
    }
    None
}

/// Whether `\c` is a backslash escape per CommonMark: only ASCII punctuation can be
/// escaped, every other `\c` is two literal characters.
fn is_escapable(c: char) -> bool {
    c.is_ascii_punctuation()
}

/// Text with every backslash escape resolved, i.e. `\c` replaced by `c` when `c` is
/// ASCII punctuation. Used for content that is not parsed as inlines but where the
/// escapes still have to be honoured, such as an image `alt`.
pub(crate) fn unescape_punctuation(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut rest = raw;
    while let Some(pos) = rest.find('\\') {
        out.push_str(&rest[..pos]);
        let after = &rest[pos + 1..];
        match after.chars().next() {
            Some(c) if is_escapable(c) => {
                out.push(c);
                rest = &after[c.len_utf8()..];
            }
            Some(c) => {
                out.push('\\');
                out.push(c);
                rest = &after[c.len_utf8()..];
            }
            None => {
                out.push('\\');
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

/// Maximum nesting depth for square brackets to prevent stack overflow.
const MAX_BRACKET_DEPTH: usize = 32;

/// Maximum nesting of link labels (`[a [b [c]]]`) that is parsed as nested links;
/// deeper labels are left as literal text.
///
/// A shortcut or collapsed `LinkReference` stores the label content in both `label`
/// and `text`, so the AST doubles at every nesting level: without a limit, 32 nested
/// brackets (65 bytes of input) would produce 2^32 nodes.
pub(crate) const MAX_LINK_LABEL_DEPTH: usize = 8;

/// Parses content inside square brackets, handling nested brackets and escapes.
/// Returns the raw string content (including nested bracket pairs).
/// Escaped brackets (\[ and \]) are converted to their literal characters.
fn balanced_brackets_content(input: &str) -> IResult<&str, Cow<'_, str>> {
    balanced_brackets_content_with_depth(input, 0)
}

/// Internal implementation with depth tracking to prevent stack overflow.
///
/// A single linear scan: `\]` becomes `]`, any other escape pair is kept verbatim
/// for the inline parser, `[` opens a nested group (a literal `[` beyond
/// `MAX_BRACKET_DEPTH`), and the content ends at the `]` that closes the current
/// group, or at a trailing backslash with nothing after it.
fn balanced_brackets_content_with_depth(input: &str, depth: usize) -> IResult<&str, Cow<'_, str>> {
    // First pass: find the end without copying. Only a `\\]` changes the text, in
    // which case a second pass builds the unescaped string.
    let bytes = input.as_bytes();
    let mut level = depth;
    let mut pos = 0;
    let mut has_escaped_bracket = false;
    while pos < bytes.len() {
        match bytes[pos] {
            b'\\' => {
                let Some(c) = input[pos + 1..].chars().next() else {
                    break;
                };
                has_escaped_bracket |= c == ']';
                pos += 1 + c.len_utf8();
            }
            b'[' => {
                if level < MAX_BRACKET_DEPTH {
                    level += 1;
                }
                pos += 1;
            }
            b']' => {
                if level == depth {
                    break;
                }
                level -= 1;
                pos += 1;
            }
            _ => pos += 1,
        }
    }
    let raw = &input[..pos];
    if !has_escaped_bracket {
        return Ok((&input[pos..], Cow::Borrowed(raw)));
    }
    let mut out = String::with_capacity(raw.len());
    let mut rest = raw;
    while let Some(i) = rest.find("\\]") {
        out.push_str(&rest[..i]);
        out.push(']');
        rest = &rest[i + 2..];
    }
    out.push_str(rest);
    Ok((&input[pos..], Cow::Owned(out)))
}

pub(crate) fn link_destination<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, String> {
    move |input: &'a str| alt((link_destination1, link_destination2(&state))).parse(input)
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

/// Destination without `<...>`: a non-empty run of destination characters, escape
/// pairs and balanced paren groups. The length comes from the delimiter index when
/// the input is covered by one, otherwise from a linear scan.
fn link_destination2<'a, 'b>(
    state: &'b MarkdownParserState,
) -> impl FnMut(&'a str) -> IResult<&'a str, String> + 'b {
    move |input: &'a str| {
        let len = match state.inline_index.borrow().as_ref() {
            Some(index) => index.destination_len(input),
            None => crate::parser::inline::index::destination_len_slow(input),
        };
        if len == 0 {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Satisfy,
            )));
        }
        Ok((&input[len..], input[..len].to_string()))
    }
}
