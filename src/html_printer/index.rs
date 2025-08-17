use crate::ast::*;
use std::collections::HashMap;

struct Index {
    footnote_indices: HashMap<String, usize>,
    link_definitions: HashMap<Vec<Inline>, LinkDefinition>,
    last_footnote_index: usize,
}

impl Index {
    pub fn new() -> Self {
        Index {
            footnote_indices: HashMap::new(),
            link_definitions: HashMap::new(),
            last_footnote_index: 1,
        }
    }

    pub fn add_footnote(&mut self, label: String) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.footnote_indices.entry(label) {
            e.insert(self.last_footnote_index);
            self.last_footnote_index += 1;
        }
    }
}

pub(crate) fn get_indicies(
    ast: &Document,
) -> (HashMap<String, usize>, HashMap<Vec<Inline>, LinkDefinition>) {
    let mut index = Index::new();

    for block in &ast.blocks {
        get_block_indicies(&mut index, block);
    }

    (index.footnote_indices, index.link_definitions)
}

fn get_block_indicies(index: &mut Index, block: &Block) {
    match block {
        Block::Paragraph(v) => {
            for inline in v {
                get_inline_indicies(index, inline);
            }
        }
        Block::Heading(v) => {
            for inline in &v.content {
                get_inline_indicies(index, inline);
            }
        }
        Block::ThematicBreak => (),
        Block::BlockQuote(v) => {
            for block in v {
                get_block_indicies(index, block);
            }
        }
        Block::List(v) => {
            for item in &v.items {
                for block in &item.blocks {
                    get_block_indicies(index, block);
                }
            }
        }
        Block::CodeBlock(_) => (),
        Block::HtmlBlock(_) => (),
        Block::Definition(v) => {
            index.link_definitions.insert(v.label.clone(), v.clone());
            for inline in &v.label {
                get_inline_indicies(index, inline);
            }
        }
        Block::Table(v) => {
            for row in &v.rows {
                for cell in row {
                    for inline in cell {
                        get_inline_indicies(index, inline);
                    }
                }
            }
        }
        Block::FootnoteDefinition(v) => {
            for block in &v.blocks {
                get_block_indicies(index, block);
            }
        }
        Block::Empty => (),
        Block::GitHubAlert(alert) => {
            for block in &alert.blocks {
                get_block_indicies(index, block);
            }
        }
    }
}

fn get_inline_indicies(index: &mut Index, inline: &Inline) {
    if let Inline::FootnoteReference(label) = inline {
        index.add_footnote(label.clone());
    }
}
