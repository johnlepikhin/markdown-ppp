//! Pre-processing indices for footnotes and link definitions.
//!
//! These functions perform a single pass over the AST to build lookup tables
//! used by renderers (HTML, LaTeX, plaintext) for cross-referencing.

use super::*;
use std::collections::HashMap;

/// Assign numeric indices (1, 2, 3, ...) to footnote definitions by document order.
///
/// Recursively traverses all nested blocks (lists, blockquotes, alerts)
/// to find every `FootnoteDefinition`.
pub fn get_footnote_indices(ast: &Document) -> HashMap<String, usize> {
    let mut index = HashMap::new();
    let mut counter = 1;

    fn process(blocks: &[Block], index: &mut HashMap<String, usize>, counter: &mut usize) {
        for block in blocks {
            match block {
                Block::FootnoteDefinition(def) => {
                    index.insert(def.label.clone(), *counter);
                    *counter += 1;
                }
                Block::List(list) => {
                    for item in &list.items {
                        process(&item.blocks, index, counter);
                    }
                }
                Block::BlockQuote(blocks) => process(blocks, index, counter),
                Block::GitHubAlert(alert) => process(&alert.blocks, index, counter),
                _ => {}
            }
        }
    }

    process(&ast.blocks, &mut index, &mut counter);
    index
}

/// Collect link reference definitions into a lookup table keyed by label.
///
/// Recursively traverses all nested blocks to find every `Definition`.
pub fn get_link_definitions(ast: &Document) -> HashMap<Vec<Inline>, LinkDefinition> {
    let mut defs = HashMap::new();

    fn process(blocks: &[Block], defs: &mut HashMap<Vec<Inline>, LinkDefinition>) {
        for block in blocks {
            match block {
                Block::Definition(def) => {
                    defs.insert(def.label.clone(), def.clone());
                }
                Block::List(list) => {
                    for item in &list.items {
                        process(&item.blocks, defs);
                    }
                }
                Block::BlockQuote(blocks) => process(blocks, defs),
                Block::GitHubAlert(alert) => process(&alert.blocks, defs),
                _ => {}
            }
        }
    }

    process(&ast.blocks, &mut defs);
    defs
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: empty document
    fn empty_doc() -> Document {
        Document { blocks: vec![] }
    }

    /// Helper: document with given top-level blocks
    fn doc(blocks: Vec<Block>) -> Document {
        Document { blocks }
    }

    // ── get_footnote_indices ──────────────────────────────────────────

    #[test]
    fn footnote_empty_document() {
        let indices = get_footnote_indices(&empty_doc());
        assert!(indices.is_empty());
    }

    #[test]
    fn footnote_multiple_sequential_indices() {
        let ast = doc(vec![
            Block::FootnoteDefinition(FootnoteDefinition {
                label: "a".into(),
                blocks: vec![],
            }),
            Block::FootnoteDefinition(FootnoteDefinition {
                label: "b".into(),
                blocks: vec![],
            }),
            Block::FootnoteDefinition(FootnoteDefinition {
                label: "c".into(),
                blocks: vec![],
            }),
        ]);
        let indices = get_footnote_indices(&ast);
        assert_eq!(indices.len(), 3);
        assert_eq!(indices["a"], 1);
        assert_eq!(indices["b"], 2);
        assert_eq!(indices["c"], 3);
    }

    #[test]
    fn footnote_inside_nested_list() {
        let ast = doc(vec![Block::List(List {
            kind: ListKind::Bullet(ListBulletKind::Dash),
            items: vec![ListItem {
                task: None,
                blocks: vec![Block::FootnoteDefinition(FootnoteDefinition {
                    label: "nested".into(),
                    blocks: vec![],
                })],
            }],
        })]);
        let indices = get_footnote_indices(&ast);
        assert_eq!(indices.len(), 1);
        assert_eq!(indices["nested"], 1);
    }

    #[test]
    fn footnote_inside_blockquote() {
        let ast = doc(vec![Block::BlockQuote(vec![
            Block::FootnoteDefinition(FootnoteDefinition {
                label: "quoted".into(),
                blocks: vec![],
            }),
        ])]);
        let indices = get_footnote_indices(&ast);
        assert_eq!(indices.len(), 1);
        assert_eq!(indices["quoted"], 1);
    }

    #[test]
    fn footnote_inside_nested_list_and_blockquote() {
        let ast = doc(vec![Block::BlockQuote(vec![Block::List(List {
            kind: ListKind::Bullet(ListBulletKind::Star),
            items: vec![ListItem {
                task: None,
                blocks: vec![Block::FootnoteDefinition(FootnoteDefinition {
                    label: "deep".into(),
                    blocks: vec![],
                })],
            }],
        })])]);
        let indices = get_footnote_indices(&ast);
        assert_eq!(indices.len(), 1);
        assert_eq!(indices["deep"], 1);
    }

    // ── get_link_definitions ─────────────────────────────────────────

    #[test]
    fn link_def_empty_document() {
        let defs = get_link_definitions(&empty_doc());
        assert!(defs.is_empty());
    }

    #[test]
    fn link_def_top_level() {
        let label = vec![Inline::Text("example".into())];
        let ast = doc(vec![Block::Definition(LinkDefinition {
            label: label.clone(),
            destination: "https://example.com".into(),
            title: None,
        })]);
        let defs = get_link_definitions(&ast);
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[&label].destination, "https://example.com");
    }

    #[test]
    fn link_def_inside_nested_list() {
        let label = vec![Inline::Text("nested-link".into())];
        let ast = doc(vec![Block::List(List {
            kind: ListKind::Ordered(ListOrderedKindOptions { start: 1 }),
            items: vec![ListItem {
                task: None,
                blocks: vec![Block::Definition(LinkDefinition {
                    label: label.clone(),
                    destination: "https://nested.example".into(),
                    title: Some("Nested".into()),
                })],
            }],
        })]);
        let defs = get_link_definitions(&ast);
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[&label].destination, "https://nested.example");
        assert_eq!(defs[&label].title.as_deref(), Some("Nested"));
    }

    #[test]
    fn link_def_inside_blockquote() {
        let label = vec![Inline::Text("bq-link".into())];
        let ast = doc(vec![Block::BlockQuote(vec![Block::Definition(
            LinkDefinition {
                label: label.clone(),
                destination: "https://bq.example".into(),
                title: None,
            },
        )])]);
        let defs = get_link_definitions(&ast);
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[&label].destination, "https://bq.example");
    }

    #[test]
    fn link_def_inside_nested_blockquote_and_list() {
        let label = vec![Inline::Text("deep-link".into())];
        let ast = doc(vec![Block::BlockQuote(vec![Block::List(List {
            kind: ListKind::Bullet(ListBulletKind::Plus),
            items: vec![ListItem {
                task: None,
                blocks: vec![Block::Definition(LinkDefinition {
                    label: label.clone(),
                    destination: "https://deep.example".into(),
                    title: None,
                })],
            }],
        })])]);
        let defs = get_link_definitions(&ast);
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[&label].destination, "https://deep.example");
    }
}
