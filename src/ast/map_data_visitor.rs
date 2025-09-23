//! Non-recursive visitor-based implementation of MapData functionality
//!
//! This module provides a visitor-based approach to transforming user data in AST
//! without hitting compiler recursion limits.

use super::generic;

/// A visitor that can transform user data in AST nodes
pub trait MapDataVisitor<T, U> {
    /// Transform user data
    fn map_data(&mut self, data: T) -> U;

    /// Transform a document
    fn visit_document(&mut self, doc: generic::Document<T>) -> generic::Document<U> {
        generic::Document {
            blocks: doc
                .blocks
                .into_iter()
                .map(|b| self.visit_block(b))
                .collect(),
            user_data: self.map_data(doc.user_data),
        }
    }

    /// Transform a block
    fn visit_block(&mut self, block: generic::Block<T>) -> generic::Block<U> {
        match block {
            generic::Block::Paragraph { content, user_data } => generic::Block::Paragraph {
                content: content.into_iter().map(|i| self.visit_inline(i)).collect(),
                user_data: self.map_data(user_data),
            },
            generic::Block::Heading(heading) => {
                generic::Block::Heading(self.visit_heading(heading))
            }
            generic::Block::ThematicBreak { user_data } => generic::Block::ThematicBreak {
                user_data: self.map_data(user_data),
            },
            generic::Block::BlockQuote { blocks, user_data } => generic::Block::BlockQuote {
                blocks: blocks.into_iter().map(|b| self.visit_block(b)).collect(),
                user_data: self.map_data(user_data),
            },
            generic::Block::List(list) => generic::Block::List(self.visit_list(list)),
            generic::Block::CodeBlock(code_block) => {
                generic::Block::CodeBlock(self.visit_code_block(code_block))
            }
            generic::Block::HtmlBlock { content, user_data } => generic::Block::HtmlBlock {
                content,
                user_data: self.map_data(user_data),
            },
            generic::Block::Definition(def) => {
                generic::Block::Definition(self.visit_link_definition(def))
            }
            generic::Block::Table(table) => generic::Block::Table(self.visit_table(table)),
            generic::Block::FootnoteDefinition(footnote) => {
                generic::Block::FootnoteDefinition(self.visit_footnote_definition(footnote))
            }
            generic::Block::GitHubAlert(alert) => {
                generic::Block::GitHubAlert(self.visit_github_alert(alert))
            }
            generic::Block::Empty { user_data } => generic::Block::Empty {
                user_data: self.map_data(user_data),
            },
        }
    }

    /// Transform an inline element
    fn visit_inline(&mut self, inline: generic::Inline<T>) -> generic::Inline<U> {
        match inline {
            generic::Inline::Text { content, user_data } => generic::Inline::Text {
                content,
                user_data: self.map_data(user_data),
            },
            generic::Inline::LineBreak { user_data } => generic::Inline::LineBreak {
                user_data: self.map_data(user_data),
            },
            generic::Inline::Code { content, user_data } => generic::Inline::Code {
                content,
                user_data: self.map_data(user_data),
            },
            generic::Inline::Html { content, user_data } => generic::Inline::Html {
                content,
                user_data: self.map_data(user_data),
            },
            generic::Inline::Link(link) => generic::Inline::Link(self.visit_link(link)),
            generic::Inline::LinkReference(link_ref) => {
                generic::Inline::LinkReference(self.visit_link_reference(link_ref))
            }
            generic::Inline::Image(image) => generic::Inline::Image(self.visit_image(image)),
            generic::Inline::Emphasis { content, user_data } => generic::Inline::Emphasis {
                content: content.into_iter().map(|i| self.visit_inline(i)).collect(),
                user_data: self.map_data(user_data),
            },
            generic::Inline::Strong { content, user_data } => generic::Inline::Strong {
                content: content.into_iter().map(|i| self.visit_inline(i)).collect(),
                user_data: self.map_data(user_data),
            },
            generic::Inline::Strikethrough { content, user_data } => {
                generic::Inline::Strikethrough {
                    content: content.into_iter().map(|i| self.visit_inline(i)).collect(),
                    user_data: self.map_data(user_data),
                }
            }
            generic::Inline::Autolink { url, user_data } => generic::Inline::Autolink {
                url,
                user_data: self.map_data(user_data),
            },
            generic::Inline::FootnoteReference { label, user_data } => {
                generic::Inline::FootnoteReference {
                    label,
                    user_data: self.map_data(user_data),
                }
            }
            generic::Inline::Empty { user_data } => generic::Inline::Empty {
                user_data: self.map_data(user_data),
            },
        }
    }

    /// Transform a heading
    fn visit_heading(&mut self, heading: generic::Heading<T>) -> generic::Heading<U> {
        generic::Heading {
            kind: heading.kind,
            content: heading
                .content
                .into_iter()
                .map(|i| self.visit_inline(i))
                .collect(),
            user_data: self.map_data(heading.user_data),
        }
    }

    /// Transform a list
    fn visit_list(&mut self, list: generic::List<T>) -> generic::List<U> {
        generic::List {
            kind: list.kind,
            items: list
                .items
                .into_iter()
                .map(|i| self.visit_list_item(i))
                .collect(),
            user_data: self.map_data(list.user_data),
        }
    }

    /// Transform a list item
    fn visit_list_item(&mut self, item: generic::ListItem<T>) -> generic::ListItem<U> {
        generic::ListItem {
            task: item.task,
            blocks: item
                .blocks
                .into_iter()
                .map(|b| self.visit_block(b))
                .collect(),
            user_data: self.map_data(item.user_data),
        }
    }

    /// Transform a code block
    fn visit_code_block(&mut self, code_block: generic::CodeBlock<T>) -> generic::CodeBlock<U> {
        generic::CodeBlock {
            kind: code_block.kind,
            literal: code_block.literal,
            user_data: self.map_data(code_block.user_data),
        }
    }

    /// Transform a link definition
    fn visit_link_definition(
        &mut self,
        def: generic::LinkDefinition<T>,
    ) -> generic::LinkDefinition<U> {
        generic::LinkDefinition {
            label: def
                .label
                .into_iter()
                .map(|i| self.visit_inline(i))
                .collect(),
            destination: def.destination,
            title: def.title,
            user_data: self.map_data(def.user_data),
        }
    }

    /// Transform a table
    fn visit_table(&mut self, table: generic::Table<T>) -> generic::Table<U> {
        generic::Table {
            rows: table
                .rows
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|cell| cell.into_iter().map(|i| self.visit_inline(i)).collect())
                        .collect()
                })
                .collect(),
            alignments: table.alignments,
            user_data: self.map_data(table.user_data),
        }
    }

    /// Transform a footnote definition
    fn visit_footnote_definition(
        &mut self,
        footnote: generic::FootnoteDefinition<T>,
    ) -> generic::FootnoteDefinition<U> {
        generic::FootnoteDefinition {
            label: footnote.label,
            blocks: footnote
                .blocks
                .into_iter()
                .map(|b| self.visit_block(b))
                .collect(),
            user_data: self.map_data(footnote.user_data),
        }
    }

    /// Transform a GitHub alert
    fn visit_github_alert(
        &mut self,
        alert: generic::GitHubAlertNode<T>,
    ) -> generic::GitHubAlertNode<U> {
        generic::GitHubAlertNode {
            alert_type: alert.alert_type,
            blocks: alert
                .blocks
                .into_iter()
                .map(|b| self.visit_block(b))
                .collect(),
            user_data: self.map_data(alert.user_data),
        }
    }

    /// Transform a link
    fn visit_link(&mut self, link: generic::Link<T>) -> generic::Link<U> {
        generic::Link {
            destination: link.destination,
            title: link.title,
            children: link
                .children
                .into_iter()
                .map(|i| self.visit_inline(i))
                .collect(),
            user_data: self.map_data(link.user_data),
        }
    }

    /// Transform an image
    fn visit_image(&mut self, image: generic::Image<T>) -> generic::Image<U> {
        generic::Image {
            destination: image.destination,
            title: image.title,
            alt: image.alt,
            user_data: self.map_data(image.user_data),
        }
    }

    /// Transform a link reference
    fn visit_link_reference(
        &mut self,
        link_ref: generic::LinkReference<T>,
    ) -> generic::LinkReference<U> {
        generic::LinkReference {
            label: link_ref
                .label
                .into_iter()
                .map(|i| self.visit_inline(i))
                .collect(),
            text: link_ref
                .text
                .into_iter()
                .map(|i| self.visit_inline(i))
                .collect(),
            user_data: self.map_data(link_ref.user_data),
        }
    }
}

/// Simple implementation using a closure
pub struct ClosureMapDataVisitor<T, U, F>
where
    F: FnMut(T) -> U,
{
    f: F,
    _phantom: std::marker::PhantomData<(T, U)>,
}

impl<T, U, F> ClosureMapDataVisitor<T, U, F>
where
    F: FnMut(T) -> U,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, U, F> MapDataVisitor<T, U> for ClosureMapDataVisitor<T, U, F>
where
    F: FnMut(T) -> U,
{
    fn map_data(&mut self, data: T) -> U {
        (self.f)(data)
    }
}

/// Convenience function to transform user data using a closure
pub fn map_user_data<T, U, F>(doc: generic::Document<T>, f: F) -> generic::Document<U>
where
    F: FnMut(T) -> U,
{
    let mut visitor = ClosureMapDataVisitor::new(f);
    visitor.visit_document(doc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_data_visitor_with_u32() {
        let doc = generic::Document {
            blocks: vec![generic::Block::Paragraph {
                content: vec![generic::Inline::Text {
                    content: "Hello".to_string(),
                    user_data: 1u32,
                }],
                user_data: 2u32,
            }],
            user_data: 0u32,
        };

        // Transform u32 to String
        let transformed = map_user_data(doc, |id| format!("element_{id}"));

        assert_eq!(transformed.user_data, "element_0");
        match &transformed.blocks[0] {
            generic::Block::Paragraph { user_data, content } => {
                assert_eq!(user_data, "element_2");
                match &content[0] {
                    generic::Inline::Text { user_data, .. } => {
                        assert_eq!(user_data, "element_1");
                    }
                    _ => panic!("Expected text"),
                }
            }
            _ => panic!("Expected paragraph"),
        }
    }

    #[test]
    fn test_complex_ast_transformation() {
        let doc = generic::Document {
            blocks: vec![
                generic::Block::Heading(generic::Heading {
                    kind: crate::ast::HeadingKind::Atx(1),
                    content: vec![
                        generic::Inline::Text {
                            content: "Title".to_string(),
                            user_data: 1u32,
                        },
                        generic::Inline::Strong {
                            content: vec![generic::Inline::Text {
                                content: "Bold".to_string(),
                                user_data: 2u32,
                            }],
                            user_data: 3u32,
                        },
                    ],
                    user_data: 4u32,
                }),
                generic::Block::List(generic::List {
                    kind: generic::ListKind::Bullet(crate::ast::ListBulletKind::Dash),
                    items: vec![generic::ListItem {
                        task: None,
                        blocks: vec![generic::Block::Paragraph {
                            content: vec![generic::Inline::Text {
                                content: "Item".to_string(),
                                user_data: 5u32,
                            }],
                            user_data: 6u32,
                        }],
                        user_data: 7u32,
                    }],
                    user_data: 8u32,
                }),
            ],
            user_data: 9u32,
        };

        // Multiply all numbers by 10
        let transformed = map_user_data(doc, |n| n * 10);

        assert_eq!(transformed.user_data, 90);

        // Check heading
        match &transformed.blocks[0] {
            generic::Block::Heading(heading) => {
                assert_eq!(heading.user_data, 40);
                match &heading.content[1] {
                    generic::Inline::Strong { user_data, content } => {
                        assert_eq!(*user_data, 30);
                        match &content[0] {
                            generic::Inline::Text { user_data, .. } => {
                                assert_eq!(*user_data, 20);
                            }
                            _ => panic!("Expected text"),
                        }
                    }
                    _ => panic!("Expected strong"),
                }
            }
            _ => panic!("Expected heading"),
        }

        // Check list
        match &transformed.blocks[1] {
            generic::Block::List(list) => {
                assert_eq!(list.user_data, 80);
                assert_eq!(list.items[0].user_data, 70);
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_custom_visitor() {
        struct CountingVisitor {
            count: u32,
        }

        impl CountingVisitor {
            fn new() -> Self {
                Self { count: 0 }
            }
        }

        impl MapDataVisitor<String, u32> for CountingVisitor {
            fn map_data(&mut self, _data: String) -> u32 {
                let result = self.count;
                self.count += 1;
                result
            }
        }

        let doc = generic::Document {
            blocks: vec![generic::Block::Paragraph {
                content: vec![
                    generic::Inline::Text {
                        content: "First".to_string(),
                        user_data: "text1".to_string(),
                    },
                    generic::Inline::Text {
                        content: "Second".to_string(),
                        user_data: "text2".to_string(),
                    },
                ],
                user_data: "paragraph".to_string(),
            }],
            user_data: "document".to_string(),
        };

        let mut visitor = CountingVisitor::new();
        let transformed = visitor.visit_document(doc);

        // Check that each element got a unique incrementing number
        // The exact order depends on visitor traversal, so let's just check they're all different
        let doc_id = transformed.user_data;
        match &transformed.blocks[0] {
            generic::Block::Paragraph { user_data, content } => {
                let para_id = *user_data;
                let text1_id = match &content[0] {
                    generic::Inline::Text { user_data, .. } => *user_data,
                    _ => panic!("Expected text"),
                };
                let text2_id = match &content[1] {
                    generic::Inline::Text { user_data, .. } => *user_data,
                    _ => panic!("Expected text"),
                };

                // Ensure all IDs are unique
                let mut ids = vec![doc_id, para_id, text1_id, text2_id];
                ids.sort();
                ids.dedup();
                assert_eq!(ids.len(), 4, "All IDs should be unique");

                // Ensure all IDs are in expected range
                assert!(ids.iter().all(|&id| id <= 3), "IDs should be 0-3");
            }
            _ => panic!("Expected paragraph"),
        }
    }
}
