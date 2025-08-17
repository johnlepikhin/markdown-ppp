use crate::ast::*;
use crate::html_printer::util::{escape, tag};
use crate::html_printer::ToDoc;
use pretty::{Arena, DocAllocator, DocBuilder};

impl<'a> ToDoc<'a> for Vec<Inline> {
    fn to_doc(&self, state: &'a crate::html_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        state.arena.concat(
            self.iter()
                .map(|inline| inline.to_doc(state))
                .collect::<Vec<_>>(),
        )
    }
}

impl<'a> ToDoc<'a> for Inline {
    fn to_doc(&self, state: &'a crate::html_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        match self {
            Inline::Text(t) => state.arena.text(escape(t)),
            Inline::LineBreak => tag(state, "br", Vec::new(), state.arena.nil()),
            Inline::Code(code) => tag(state, "code", Vec::new(), state.arena.text(escape(code))),
            Inline::Html(html) => state.arena.text(html.clone()),
            Inline::Emphasis(children) => tag(state, "em", Vec::new(), children.to_doc(state)),
            Inline::Strong(children) => tag(state, "b", Vec::new(), children.to_doc(state)),
            Inline::Strikethrough(children) => tag(state, "s", Vec::new(), children.to_doc(state)),
            Inline::Link(Link {
                destination,
                title,
                children,
            }) => {
                let mut attributes = vec![("href".to_owned(), escape(destination))];
                if let Some(title) = title {
                    attributes.push(("title".to_owned(), escape(title)))
                }
                tag(state, "a", attributes, children.to_doc(state))
            }
            Inline::Image(Image {
                destination,
                title,
                alt,
            }) => {
                let mut attributes = vec![
                    ("src".to_owned(), escape(destination)),
                    ("alt".to_owned(), escape(alt)),
                ];
                if let Some(title) = title {
                    attributes.push(("title".to_owned(), escape(title)))
                }
                tag(state, "img", attributes, state.arena.nil())
            }
            Inline::Autolink(link) => tag(
                state,
                "a",
                vec![("href".to_owned(), escape(link))],
                state.arena.text(escape(link)),
            ),
            Inline::FootnoteReference(label) => {
                let index = match state.get_footnote_index(label) {
                    Some(v) => v,
                    None => return state.arena.nil(),
                };
                tag(
                    state,
                    "a",
                    vec![
                        ("class".to_owned(), "markdown-footnote-reference".to_owned()),
                        (
                            "href".to_owned(),
                            escape(&format!("#{}{}", state.config.anchor_prefix, index)),
                        ),
                    ],
                    state.arena.text(format!("[{index}]")),
                )
            }
            Inline::Empty => state.arena.nil(),
            Inline::LinkReference(v) => {
                let definition = match state.get_link_definition(&v.label) {
                    Some(v) => v,
                    None => return state.arena.nil(),
                };
                let mut attributes =
                    vec![("href".to_owned(), escape(definition.destination.as_str()))];
                if let Some(title) = &definition.title {
                    attributes.push(("title".to_owned(), escape(title)))
                }
                tag(state, "a", attributes, v.text.to_doc(state))
            }
        }
    }
}
