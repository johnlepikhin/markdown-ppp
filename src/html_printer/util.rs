use pretty::{Arena, DocAllocator, DocBuilder};

pub(crate) fn escape(value: &str) -> String {
    let mut escaped = String::new();
    for c in value.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(c),
        }
    }
    escaped
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
    let mut attrs = state.arena.nil();
    for (key, value) in attributes {
        let attr = state
            .arena
            .text(" ")
            .append(state.arena.text(key))
            .append(state.arena.text("=\""))
            .append(state.arena.text(escape(&value)))
            .append(state.arena.text("\""));
        attrs = attrs.append(attr);
    }
    let open_tag = state
        .arena
        .text("<")
        .append(state.arena.text(tag))
        .append(attrs)
        .append(state.arena.text(">"));
    let close_tag = state
        .arena
        .text("</")
        .append(state.arena.text(tag))
        .append(state.arena.text(">"));
    open_tag.append(inner).append(close_tag)
}
