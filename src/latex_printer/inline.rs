use crate::ast::*;
use crate::latex_printer::util::{command, escape_latex};
use crate::latex_printer::ToDoc;
use pretty::{Arena, DocAllocator, DocBuilder};

impl<'a> ToDoc<'a> for Vec<Inline> {
    fn to_doc(&self, state: &'a crate::latex_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        state
            .arena
            .concat(self.iter().map(|inline| inline.to_doc(state)))
    }
}

impl<'a> ToDoc<'a> for Inline {
    fn to_doc(&self, state: &'a crate::latex_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        match self {
            Inline::Text(text) => {
                // Replace newlines with spaces for LaTeX
                let text = text.replace('\n', " ");

                // If text is only whitespace, preserve it as-is
                if text.trim().is_empty() {
                    return state.arena.text(escape_latex(&text));
                }

                let words_or_spaces: Vec<_> = split_with_spaces(&text);

                // Use softline to allow line breaks between words
                let words_or_spaces = words_or_spaces.into_iter().map(|v| match v {
                    Some(word) => state.arena.text(escape_latex(word)),
                    None => state.arena.softline(),
                });
                state.arena.concat(words_or_spaces)
            }

            Inline::LineBreak => state.arena.text(r"\\").append(state.arena.hardline()),

            Inline::Code(code) => command(
                &state.arena,
                "texttt",
                &[],
                state.arena.text(escape_latex(code)),
            ),

            Inline::Html(html) => {
                // For LaTeX, we'll just escape HTML as text
                state.arena.text(escape_latex(html))
            }

            Inline::Link(link) => {
                let text = link.children.to_doc(state);
                let url = escape_latex(&link.destination);

                if link.title.is_some() {
                    // LaTeX doesn't have a simple way to show link titles, so we'll use a footnote
                    let title = escape_latex(link.title.as_ref().unwrap());
                    command(&state.arena, "href", &[], state.arena.text(url))
                        .append(state.arena.text("{"))
                        .append(text)
                        .append(state.arena.text("}"))
                        .append(command(
                            &state.arena,
                            "footnote",
                            &[],
                            state.arena.text(title),
                        ))
                } else {
                    command(&state.arena, "href", &[], state.arena.text(url))
                        .append(state.arena.text("{"))
                        .append(text)
                        .append(state.arena.text("}"))
                }
            }

            Inline::LinkReference(link_ref) => {
                // Try to resolve the reference
                if let Some(definition) = state.get_link_definition(&link_ref.label) {
                    let url = escape_latex(&definition.destination);
                    let text = link_ref.text.to_doc(state);

                    command(&state.arena, "href", &[], state.arena.text(url))
                        .append(state.arena.text("{"))
                        .append(text)
                        .append(state.arena.text("}"))
                } else {
                    // Fallback: render as text
                    state
                        .arena
                        .text("[")
                        .append(link_ref.text.to_doc(state))
                        .append(state.arena.text("]["))
                        .append(link_ref.label.to_doc(state))
                        .append(state.arena.text("]"))
                }
            }

            Inline::Image(image) => {
                let url = escape_latex(&image.destination);
                let alt = escape_latex(&image.alt);

                // Use includegraphics for images
                let mut cmd = command(&state.arena, "includegraphics", &[], state.arena.nil());
                cmd = cmd
                    .append(state.arena.text("{"))
                    .append(state.arena.text(url))
                    .append(state.arena.text("}"));

                if !image.alt.is_empty() {
                    cmd = cmd.append(command(&state.arena, "caption", &[], state.arena.text(alt)));
                }

                cmd
            }

            Inline::Emphasis(content) => {
                command(&state.arena, "textit", &[], content.to_doc(state))
            }

            Inline::Strong(content) => command(&state.arena, "textbf", &[], content.to_doc(state)),

            Inline::Strikethrough(content) => {
                command(&state.arena, "sout", &[], content.to_doc(state))
            }

            Inline::Autolink(url) => {
                let escaped_url = escape_latex(url);
                command(&state.arena, "url", &[], state.arena.text(escaped_url))
            }

            Inline::FootnoteReference(label) => {
                if let Some(index) = state.get_footnote_index(label) {
                    command(
                        &state.arena,
                        "footnotemark",
                        &[],
                        state.arena.text(format!("[{index}]")),
                    )
                } else {
                    // Fallback: render as text
                    state
                        .arena
                        .text("[^")
                        .append(state.arena.text(escape_latex(label)))
                        .append(state.arena.text("]"))
                }
            }

            Inline::Empty => state.arena.nil(),
        }
    }
}

/// Split string by spaces, but keep the spaces in the result for proper word wrapping.
///
/// This function is similar to the one in printer/inline.rs but adapted for LaTeX output.
/// It returns `Some(word)` for actual words and `None` for whitespace positions,
/// allowing the pretty-printer to insert line breaks at word boundaries.
fn split_with_spaces(s: &str) -> Vec<Option<&str>> {
    let mut result = Vec::new();
    let mut word_start: Option<usize> = None;

    for (i, c) in s.char_indices() {
        if c.is_whitespace() {
            if let Some(start) = word_start {
                result.push(Some(&s[start..i]));
                word_start = None;
            }
            // Add whitespace marker if not already present
            if result.last().is_none_or(|x| x.is_some()) {
                result.push(None);
            }
        } else if word_start.is_none() {
            word_start = Some(i);
        }
    }

    // Add final word if string doesn't end with whitespace
    if let Some(start) = word_start {
        result.push(Some(&s[start..]));
    }

    result
}
