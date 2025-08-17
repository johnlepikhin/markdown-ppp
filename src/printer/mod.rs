mod block;
mod blockquote;
pub mod config;
mod github_alert;
mod heading;
mod inline;
mod list;
mod table;
mod tests;

use crate::ast::*;
use pretty::{Arena, DocBuilder};
use std::rc::Rc;

/// Render Markdown AST to Markdown document.
pub fn render_markdown(ast: &Document, config: crate::printer::config::Config) -> String {
    let config = Rc::new(config);
    let arena = Arena::new();
    let doc = ast.to_doc(config.clone(), &arena);

    let mut buf = Vec::new();
    doc.render(config.width, &mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

trait ToDoc<'a> {
    fn to_doc(
        &self,
        config: Rc<crate::printer::config::Config>,
        arena: &'a Arena<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()>;
}

impl<'a> ToDoc<'a> for Document {
    fn to_doc(
        &self,
        config: Rc<crate::printer::config::Config>,
        arena: &'a Arena<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        self.blocks.to_doc(config, arena)
    }
}
