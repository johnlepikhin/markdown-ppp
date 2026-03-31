use crate::ast::*;
use crate::plaintext_printer::ToDoc;
use pretty::{Arena, DocAllocator, DocBuilder};

impl<'a> ToDoc<'a> for Vec<Inline> {
    fn to_doc(
        &self,
        state: &'a crate::plaintext_printer::State<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        state
            .arena
            .concat(self.iter().map(|inline| inline.to_doc(state)))
    }
}

impl<'a> ToDoc<'a> for Inline {
    fn to_doc(
        &self,
        state: &'a crate::plaintext_printer::State<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        match self {
            Inline::Text(t) => state.arena.text(t.clone()),
            Inline::LineBreak => state.arena.hardline(),
            Inline::Code(code) => state.arena.text(code.clone()),
            Inline::Html(_) => state.arena.nil(),
            Inline::Emphasis(children) => children.to_doc(state),
            Inline::Strong(children) => children.to_doc(state),
            Inline::Strikethrough(children) => children.to_doc(state),
            Inline::Link(Link { children, .. }) => children.to_doc(state),
            Inline::Image(Image { alt, .. }) => state.arena.text(alt.clone()),
            Inline::Autolink(link) => state.arena.text(link.clone()),
            Inline::FootnoteReference(label) => {
                let index = match state.get_footnote_index(label) {
                    Some(v) => v,
                    None => return state.arena.nil(),
                };
                state.arena.text(format!("[{index}]"))
            }
            Inline::LinkReference(v) => v.text.to_doc(state),
            Inline::Empty => state.arena.nil(),
        }
    }
}
