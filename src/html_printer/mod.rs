mod block;
pub mod config;
mod github_alert;
mod index;
mod inline;
mod tests;
mod util;

use crate::ast::*;
use pretty::{Arena, DocBuilder};
use std::{collections::HashMap, rc::Rc};

pub(crate) struct State<'a> {
    arena: Arena<'a>,
    config: crate::html_printer::config::Config,
    // Mapping of footnote labels to their indices in the footnote list.
    footnote_index: HashMap<String, usize>,
    // Mapping of link labels to their definitions.
    link_definitions: HashMap<Vec<Inline>, LinkDefinition>,
}

impl State<'_> {
    pub fn new(config: crate::html_printer::config::Config, ast: &Document) -> Self {
        let (footnote_index, link_definitions) = crate::html_printer::index::get_indicies(ast);
        let arena = Arena::new();
        Self {
            arena,
            config,
            footnote_index,
            link_definitions,
        }
    }

    pub fn get_footnote_index(&self, label: &str) -> Option<&usize> {
        self.footnote_index.get(label)
    }

    pub fn get_link_definition(&self, label: &Vec<Inline>) -> Option<&LinkDefinition> {
        self.link_definitions.get(label)
    }
}

/// Render the given Markdown AST to HTML.
pub fn render_html(ast: &Document, config: crate::html_printer::config::Config) -> String {
    let state = Rc::new(State::new(config, ast));
    let doc = ast.to_doc(&state);

    let mut buf = Vec::new();
    doc.render(state.config.width, &mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

trait ToDoc<'a> {
    fn to_doc(&self, state: &'a State<'a>) -> DocBuilder<'a, Arena<'a>, ()>;
}

impl<'a> ToDoc<'a> for Document {
    fn to_doc(&self, state: &'a State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        self.blocks.to_doc(state)
    }
}
