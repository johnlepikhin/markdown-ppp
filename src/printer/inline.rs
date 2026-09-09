use crate::ast::*;
use crate::printer::markdown_syntax_detector::is_safe_line_break_before;
use pretty::{Arena, DocAllocator, DocBuilder};

pub(crate) trait ToDocInline<'a> {
    fn to_doc_inline(
        &self,
        allow_newlines: bool,
        arena: &'a Arena<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()>;
}

impl<'a> ToDocInline<'a> for Vec<Inline> {
    fn to_doc_inline(
        &self,
        allow_newlines: bool,
        arena: &'a Arena<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        arena.concat(
            self.iter()
                .map(|inline| inline.to_doc_inline(allow_newlines, arena))
                .collect::<Vec<_>>(),
        )
    }
}

impl<'a> ToDocInline<'a> for Inline {
    fn to_doc_inline(
        &self,
        allow_newlines: bool,
        arena: &'a Arena<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        match self {
            Inline::Text(t) => {
                let t = t.replace('\n', " ");
                let words_or_spaces: Vec<_> = split_with_spaces(&t);

                if !allow_newlines {
                    // If newlines are not allowed, use simple space separators
                    let words_or_spaces = words_or_spaces.into_iter().map(|v| match v {
                        Some(v) => arena.text(v.to_string()),
                        None => arena.space(),
                    });
                    arena.concat(words_or_spaces)
                } else {
                    // Use smart line breaking when newlines are allowed
                    safe_text_layout(&words_or_spaces, arena)
                }
            }
            // TODO parametrize format
            Inline::LineBreak => arena.text("  \n"),
            Inline::Code(code) => arena.text("`").append(code.clone()).append(arena.text("`")),
            Inline::Html(html) => arena.text(html.clone()),
            Inline::Emphasis(children) => arena
                .text("*")
                .append(children.to_doc_inline(allow_newlines, arena))
                .append(arena.text("*")),
            Inline::Strong(children) => arena
                .text("**")
                .append(children.to_doc_inline(allow_newlines, arena))
                .append(arena.text("**")),
            Inline::Strikethrough(children) => arena
                .text("~~")
                .append(children.to_doc_inline(allow_newlines, arena))
                .append(arena.text("~~")),
            Inline::Link(Link {
                destination,
                title,
                children,
            }) => {
                let title = arena.text(link_title_suffix(title.as_ref()));
                arena
                    .text("[")
                    .append(children.to_doc_inline(allow_newlines, arena))
                    .append(arena.text("]("))
                    .append(arena.text(link_destination_to_string(destination)))
                    .append(title)
                    .append(")")
            }
            Inline::Image(Image {
                destination,
                title,
                alt,
            }) => {
                let title_part = link_title_suffix(title.as_ref());
                arena
                    .text("![")
                    .append(arena.text(escape_image_alt(alt)))
                    .append("](")
                    .append(arena.text(link_destination_to_string(destination)))
                    .append(arena.text(title_part))
                    .append(arena.text(")"))
            }
            Inline::Autolink(link) => arena.text(format!("<{link}>")),
            Inline::FootnoteReference(label) => arena.text(format!("[^{label}]")),
            Inline::Empty => arena.nil(),
            Inline::LinkReference(v) => {
                if v.label == v.text {
                    return arena
                        .text("[")
                        .append(v.label.to_doc_inline(allow_newlines, arena))
                        .append("]");
                }
                arena
                    .text("[")
                    .append(v.text.to_doc_inline(allow_newlines, arena))
                    .append("][")
                    .append(v.label.to_doc_inline(allow_newlines, arena))
                    .append(arena.text("]"))
            }
        }
    }
}

/// Render a link destination so that re-parsing it yields the same string.
///
/// The bare form only accepts a non-empty run of characters that are neither spaces
/// nor control characters nor `<`, with balanced parentheses; everything else has to
/// go into `<...>`, where `<` and `>` are escaped. Inside `<...>` a backslash is only
/// an escape before `<` or `>`, so the rest of the string is emitted verbatim — which
/// is also how the parser stores it.
pub(crate) fn link_destination_to_string(destination: &str) -> String {
    if is_bare_destination(destination) {
        return destination.to_owned();
    }
    let mut out = String::with_capacity(destination.len() + 2);
    out.push('<');
    for c in destination.chars() {
        if c == '<' || c == '>' {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('>');
    out
}

/// Whether `destination` can be printed without the surrounding `<...>`.
fn is_bare_destination(destination: &str) -> bool {
    if destination.is_empty() {
        return false;
    }
    let mut depth = 0usize;
    let mut chars = destination.chars();
    while let Some(c) = chars.next() {
        match c {
            // An escape pair is opaque: its second character is literal.
            '\\' => {
                if chars.next().is_none() {
                    return false;
                }
            }
            '(' => depth += 1,
            ')' => match depth.checked_sub(1) {
                Some(v) => depth = v,
                None => return false,
            },
            c if c.is_whitespace() || c.is_control() || c == '<' => return false,
            _ => {}
        }
    }
    depth == 0
}

/// Render a link or image title as ` "title"`, or nothing when there is none.
///
/// The parser resolves backslash escapes in a title, so `"` and `\` have to be escaped
/// again; otherwise a title containing a quote ends the title early on re-parse.
pub(crate) fn link_title_suffix(title: Option<&String>) -> String {
    let Some(title) = title else {
        return String::new();
    };
    let mut out = String::with_capacity(title.len() + 3);
    out.push_str(" \"");
    for c in title.chars() {
        if c == '\\' || c == '"' {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('"');
    out
}

/// Escape an image `alt` so that re-parsing `![alt]` yields it back.
///
/// The parser stores `alt` with its backslash escapes resolved and does not parse it
/// as inline content, so only the characters that end the description (`]`) and the
/// escape marker itself have to be escaped again.
fn escape_image_alt(alt: &str) -> String {
    let mut out = String::with_capacity(alt.len());
    for c in alt.chars() {
        if c == '\\' || c == ']' {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// Split string by spaces, but keep the spaces in the result.
fn split_with_spaces(s: &str) -> Vec<Option<&str>> {
    let mut result = Vec::new();
    let mut word_start: Option<usize> = None;

    for (i, c) in s.char_indices() {
        if c.is_whitespace() {
            if let Some(start) = word_start {
                result.push(Some(&s[start..i]));
                word_start = None;
            }
            if result.last().is_none_or(|x| x.is_some()) {
                result.push(None);
            }
        } else if word_start.is_none() {
            word_start = Some(i);
        }
    }

    if let Some(start) = word_start {
        result.push(Some(&s[start..]));
    }

    result
}

/// Safely layout text with intelligent line breaking that avoids markdown syntax conflicts
///
/// This function takes a sequence of words and spaces and creates a document builder
/// that intelligently chooses between soft line breaks and hard spaces to avoid
/// creating unwanted markdown syntax at the beginning of lines.
///
/// # Arguments
///
/// * `words_or_spaces` - A vector where Some(word) represents a word and None represents a space
/// * `arena` - The pretty-printing arena for creating document builders
///
/// # Returns
///
/// A DocBuilder that will render the text with safe line breaks
fn safe_text_layout<'a>(
    words_or_spaces: &[Option<&str>],
    arena: &'a Arena<'a>,
) -> DocBuilder<'a, Arena<'a>, ()> {
    if words_or_spaces.is_empty() {
        return arena.nil();
    }

    let mut result = arena.nil();
    let mut i = 0;

    while i < words_or_spaces.len() {
        match words_or_spaces[i] {
            Some(word) => {
                result = result.append(arena.text(word.to_string()));
                i += 1;
            }
            None => {
                // This is a space position - decide whether to use softline or hard space
                // Look ahead to see what the next word would be
                let next_word = find_next_word(&words_or_spaces[i + 1..]);

                let separator = if let Some(next_word) = next_word {
                    if is_safe_line_break_before(next_word, &[]) {
                        // Safe to break line here
                        arena.softline()
                    } else {
                        // Not safe - force a space to prevent line break
                        arena.space()
                    }
                } else {
                    // No next word, safe to use softline
                    arena.softline()
                };

                result = result.append(separator);
                i += 1;
            }
        }
    }

    result
}

/// Find the next word in a sequence of words and spaces
///
/// # Arguments
///
/// * `words_or_spaces` - A slice starting from a potential space position
///
/// # Returns
///
/// The next word that would appear, or None if no word is found
fn find_next_word<'a>(words_or_spaces: &'a [Option<&'a str>]) -> Option<&'a str> {
    words_or_spaces.iter().flatten().next().copied()
}
