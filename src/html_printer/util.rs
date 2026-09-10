use pretty::{Arena, DocAllocator, DocBuilder};
use std::borrow::Cow;

/// Escape `& < > " '` for HTML. Borrows the input when there is nothing to escape,
/// which is the common case for text nodes.
pub(crate) fn escape(value: &str) -> Cow<'_, str> {
    let first = value
        .bytes()
        .position(|b| matches!(b, b'&' | b'<' | b'>' | b'"' | b'\''));
    let Some(first) = first else {
        return Cow::Borrowed(value);
    };
    let mut escaped = String::with_capacity(value.len() + 8);
    escaped.push_str(&value[..first]);
    for c in value[first..].chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(c),
        }
    }
    Cow::Owned(escaped)
}

/// Render `<tag attr="value" ...>inner</tag>`.
///
/// Attribute values are escaped here and must therefore be passed raw: escaping them
/// at the call site yields `&amp;apos;` for `'` and breaks every URL with a query
/// string. `inner` is inserted verbatim, so callers escape text nodes themselves.
pub(crate) fn tag<'a>(
    state: &'a crate::html_printer::State<'a>,
    tag: &'static str,
    attributes: Vec<(String, String)>,
    inner: DocBuilder<'a, Arena<'a>, ()>,
) -> DocBuilder<'a, Arena<'a>, ()> {
    // One text node per tag instead of one per token: the pretty-printer arena is
    // the dominant cost of rendering, and a tag never breaks across lines anyway.
    let mut open_tag = String::with_capacity(tag.len() + 2);
    open_tag.push('<');
    open_tag.push_str(tag);
    for (key, value) in attributes {
        open_tag.push(' ');
        open_tag.push_str(&key);
        open_tag.push_str("=\"");
        open_tag.push_str(&escape(&value));
        open_tag.push('"');
    }
    open_tag.push('>');
    let mut close_tag = String::with_capacity(tag.len() + 3);
    close_tag.push_str("</");
    close_tag.push_str(tag);
    close_tag.push('>');
    state
        .arena
        .text(open_tag)
        .append(inner)
        .append(state.arena.text(close_tag))
}
