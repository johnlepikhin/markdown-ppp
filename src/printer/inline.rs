use crate::ast::*;
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
                let separator = if allow_newlines {
                    arena.softline()
                } else {
                    arena.space()
                };
                let words_or_spaces = words_or_spaces.into_iter().map(|v| match v {
                    Some(v) => arena.text(v.to_string()),
                    None => separator.clone(),
                });
                arena.concat(words_or_spaces)
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
                let title = match title {
                    Some(v) => arena
                        .text(" \"")
                        .append(arena.text(v.clone()))
                        .append(arena.text("\"")),
                    None => arena.nil(),
                };
                arena
                    .text("[")
                    .append(children.to_doc_inline(allow_newlines, arena))
                    .append(arena.text("]("))
                    .append(arena.text(destination.clone()))
                    .append(title)
                    .append(")")
            }
            Inline::Image(Image {
                destination,
                title,
                alt,
            }) => {
                let title_part = title
                    .as_ref()
                    .map(|t| format!(" \"{t}\""))
                    .unwrap_or_default();
                arena
                    .text("![")
                    .append(arena.text(alt.clone()))
                    .append("](")
                    .append(arena.text(destination.clone()))
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
