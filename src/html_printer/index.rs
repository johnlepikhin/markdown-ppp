use crate::ast::*;
use std::collections::HashMap;

struct FootnoteIndex {
    indices: HashMap<String, usize>,
    counter: usize,
}

impl FootnoteIndex {
    fn new() -> Self {
        FootnoteIndex {
            indices: HashMap::new(),
            counter: 1,
        }
    }

    fn add(&mut self, label: String) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.indices.entry(label) {
            e.insert(self.counter);
            self.counter += 1;
        }
    }
}

/// Build footnote indices ordered by first reference appearance in inline content,
/// and collect link definitions.
///
/// Unlike `ast::index::get_footnote_indices` which orders by definition position,
/// this orders by the first `FootnoteReference` encountered in inline traversal.
pub(crate) fn get_indices(
    ast: &Document,
) -> (HashMap<String, usize>, HashMap<Vec<Inline>, LinkDefinition>) {
    let mut footnotes = FootnoteIndex::new();

    for block in &ast.blocks {
        collect_footnote_refs(&mut footnotes, block);
    }

    let link_definitions = crate::ast::index::get_link_definitions(ast);

    (footnotes.indices, link_definitions)
}

fn collect_footnote_refs(footnotes: &mut FootnoteIndex, block: &Block) {
    match block {
        Block::Paragraph(v) => {
            for inline in v {
                collect_inline_refs(footnotes, inline);
            }
        }
        Block::Heading(v) => {
            for inline in &v.content {
                collect_inline_refs(footnotes, inline);
            }
        }
        Block::BlockQuote(v) => {
            for block in v {
                collect_footnote_refs(footnotes, block);
            }
        }
        Block::List(v) => {
            for item in &v.items {
                for block in &item.blocks {
                    collect_footnote_refs(footnotes, block);
                }
            }
        }
        Block::Definition(v) => {
            for inline in &v.label {
                collect_inline_refs(footnotes, inline);
            }
        }
        Block::Table(v) => {
            for row in &v.rows {
                for cell in row {
                    for inline in cell {
                        collect_inline_refs(footnotes, inline);
                    }
                }
            }
        }
        Block::FootnoteDefinition(v) => {
            for block in &v.blocks {
                collect_footnote_refs(footnotes, block);
            }
        }
        Block::GitHubAlert(alert) => {
            for block in &alert.blocks {
                collect_footnote_refs(footnotes, block);
            }
        }
        Block::ThematicBreak | Block::CodeBlock(_) | Block::HtmlBlock(_) | Block::Empty => (),
    }
}

fn collect_inline_refs(footnotes: &mut FootnoteIndex, inline: &Inline) {
    match inline {
        Inline::FootnoteReference(label) => {
            footnotes.add(label.clone());
        }
        Inline::Emphasis(children)
        | Inline::Strong(children)
        | Inline::Strikethrough(children) => {
            for child in children {
                collect_inline_refs(footnotes, child);
            }
        }
        Inline::Link(Link { children, .. }) => {
            for child in children {
                collect_inline_refs(footnotes, child);
            }
        }
        Inline::LinkReference(LinkReference { text, .. }) => {
            for child in text {
                collect_inline_refs(footnotes, child);
            }
        }
        Inline::Text(_)
        | Inline::LineBreak
        | Inline::Code(_)
        | Inline::Html(_)
        | Inline::Image(_)
        | Inline::Autolink(_)
        | Inline::Empty => {}
    }
}
