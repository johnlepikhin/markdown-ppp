use crate::ast::*;
use crate::ast_transform::visitor::VisitWith;
use crate::ast_transform::Visitor;

// Test helper to create test document with correct AST structure
fn create_test_doc() -> Document {
    Document {
        blocks: vec![
            // Simple paragraph
            Block::Paragraph(vec![
                Inline::Text("Hello ".to_string()),
                Inline::Emphasis(vec![Inline::Text("world".to_string())]),
                Inline::Text(" with ".to_string()),
                Inline::Strong(vec![
                    Inline::Text("strong ".to_string()),
                    Inline::Code("code".to_string()),
                ]),
                Inline::Text(" and ".to_string()),
                Inline::Link(Link {
                    destination: "http://example.com".to_string(),
                    title: Some("Example".to_string()),
                    children: vec![Inline::Text("link".to_string())],
                }),
            ]),
            // Heading with correct structure
            Block::Heading(Heading {
                kind: HeadingKind::Atx(2),
                content: vec![
                    Inline::Text("Heading with ".to_string()),
                    Inline::Strikethrough(vec![Inline::Text("strikethrough".to_string())]),
                    Inline::Text(" and ".to_string()),
                    Inline::Autolink("mailto:test@example.com".to_string()),
                ],
            }),
            // Code block with correct structure
            Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Fenced {
                    info: Some("rust".to_string()),
                },
                literal: "fn main() { println!(\"Hello\"); }".to_string(),
            }),
            // Blockquote with nested blocks
            Block::BlockQuote(vec![Block::Paragraph(vec![
                Inline::Text("Quoted text with ".to_string()),
                Inline::Html("<em>HTML</em>".to_string()),
            ])]),
            // List with correct structure
            Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Star),
                items: vec![
                    ListItem {
                        task: None,
                        blocks: vec![Block::Paragraph(vec![Inline::Text(
                            "First item".to_string(),
                        )])],
                    },
                    ListItem {
                        task: None,
                        blocks: vec![Block::Paragraph(vec![
                            Inline::Text("Second item with ".to_string()),
                            Inline::LinkReference(LinkReference {
                                label: vec![Inline::Text("ref".to_string())],
                                text: vec![Inline::Text("reference".to_string())],
                            }),
                        ])],
                    },
                ],
            }),
            // Table with correct structure (first row is header)
            Block::Table(Table {
                rows: vec![
                    // Header row
                    vec![
                        vec![Inline::Text("Header 1".to_string())],
                        vec![Inline::Text("Header 2".to_string())],
                    ],
                    // Data row
                    vec![
                        vec![Inline::Text("Cell 1".to_string())],
                        vec![
                            Inline::Code("table code".to_string()),
                            Inline::Text(" content".to_string()),
                        ],
                    ],
                ],
                alignments: vec![Alignment::Left, Alignment::Left],
            }),
            // Footnote definition
            Block::FootnoteDefinition(FootnoteDefinition {
                label: "note1".to_string(),
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "Footnote content".to_string(),
                )])],
            }),
            // GitHub Alert
            Block::GitHubAlert(GitHubAlert {
                alert_type: GitHubAlertType::Warning,
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "Warning message".to_string(),
                )])],
            }),
            // Link definition (not Definition)
            Block::Definition(LinkDefinition {
                label: vec![
                    Inline::Text("definition ".to_string()),
                    Inline::Emphasis(vec![Inline::Text("label".to_string())]),
                ],
                destination: "http://definition.com".to_string(),
                title: Some("Definition title".to_string()),
            }),
        ],
    }
}

// Basic visitor that collects all text content
struct TextCollector {
    texts: Vec<String>,
}

impl Visitor for TextCollector {
    fn visit_inline(&mut self, inline: &Inline) {
        if let Inline::Text(text) = inline {
            self.texts.push(text.clone());
        }
        self.walk_inline(inline);
    }
}

// Visitor that counts different types of nodes
struct NodeCounter {
    text_count: usize,
    emphasis_count: usize,
    strong_count: usize,
    link_count: usize,
    image_count: usize,
    code_count: usize,
    autolink_count: usize,
    html_count: usize,
    strikethrough_count: usize,
    link_ref_count: usize,
    footnote_ref_count: usize,

    paragraph_count: usize,
    heading_count: usize,
    blockquote_count: usize,
    list_count: usize,
    table_count: usize,
    code_block_count: usize,
    html_block_count: usize,
    thematic_break_count: usize,
    footnote_def_count: usize,
    github_alert_count: usize,
    definition_count: usize,
}

impl NodeCounter {
    fn new() -> Self {
        Self {
            text_count: 0,
            emphasis_count: 0,
            strong_count: 0,
            link_count: 0,
            image_count: 0,
            code_count: 0,
            autolink_count: 0,
            html_count: 0,
            strikethrough_count: 0,
            link_ref_count: 0,
            footnote_ref_count: 0,
            paragraph_count: 0,
            heading_count: 0,
            blockquote_count: 0,
            list_count: 0,
            table_count: 0,
            code_block_count: 0,
            html_block_count: 0,
            thematic_break_count: 0,
            footnote_def_count: 0,
            github_alert_count: 0,
            definition_count: 0,
        }
    }
}

impl Visitor for NodeCounter {
    fn visit_inline(&mut self, inline: &Inline) {
        match inline {
            Inline::Text(_) => self.text_count += 1,
            Inline::Emphasis(_) => self.emphasis_count += 1,
            Inline::Strong(_) => self.strong_count += 1,
            Inline::Link(_) => self.link_count += 1,
            Inline::Image(_) => self.image_count += 1,
            Inline::Code(_) => self.code_count += 1,
            Inline::Autolink(_) => self.autolink_count += 1,
            Inline::Html(_) => self.html_count += 1,
            Inline::Strikethrough(_) => self.strikethrough_count += 1,
            Inline::LinkReference(_) => self.link_ref_count += 1,
            Inline::FootnoteReference(_) => self.footnote_ref_count += 1,
            Inline::LineBreak => {}
            Inline::Empty => {}
        }
        self.walk_inline(inline);
    }

    fn visit_block(&mut self, block: &Block) {
        match block {
            Block::Paragraph(_) => self.paragraph_count += 1,
            Block::Heading(_) => self.heading_count += 1,
            Block::BlockQuote(_) => self.blockquote_count += 1,
            Block::List(_) => self.list_count += 1,
            Block::Table(_) => self.table_count += 1,
            Block::CodeBlock(_) => self.code_block_count += 1,
            Block::HtmlBlock(_) => self.html_block_count += 1,
            Block::ThematicBreak => self.thematic_break_count += 1,
            Block::FootnoteDefinition(_) => self.footnote_def_count += 1,
            Block::GitHubAlert(_) => self.github_alert_count += 1,
            Block::Definition(_) => self.definition_count += 1,
            Block::Empty => {}
        }
        self.walk_block(block);
    }
}

#[test]
fn test_basic_text_collection() {
    let doc = create_test_doc();
    let mut collector = TextCollector { texts: Vec::new() };

    doc.visit_with(&mut collector);

    // Verify all text nodes were collected
    assert!(collector.texts.contains(&"Hello ".to_string()));
    assert!(collector.texts.contains(&"world".to_string()));
    assert!(collector.texts.contains(&"strong ".to_string()));
    assert!(collector.texts.contains(&"link".to_string()));
    assert!(collector.texts.contains(&"Heading with ".to_string()));
    assert!(collector.texts.contains(&"strikethrough".to_string()));
    assert!(collector.texts.contains(&"Quoted text with ".to_string()));
    assert!(collector.texts.contains(&"First item".to_string()));
    assert!(collector.texts.contains(&"Second item with ".to_string()));
    assert!(collector.texts.contains(&"ref".to_string()));
    assert!(collector.texts.contains(&"reference".to_string()));
    assert!(collector.texts.contains(&"Header 1".to_string()));
    assert!(collector.texts.contains(&"Header 2".to_string()));
    assert!(collector.texts.contains(&"Cell 1".to_string()));
    assert!(collector.texts.contains(&"Footnote content".to_string()));
    assert!(collector.texts.contains(&"Warning message".to_string()));
    assert!(collector.texts.contains(&"definition ".to_string()));
}

#[test]
fn test_comprehensive_node_counting() {
    let doc = create_test_doc();
    let mut counter = NodeCounter::new();

    doc.visit_with(&mut counter);

    // Verify inline counts
    assert!(counter.text_count > 10); // Count all text nodes
    assert_eq!(counter.emphasis_count, 2); // One in paragraph, one in definition
    assert_eq!(counter.strong_count, 1);
    assert_eq!(counter.link_count, 1);
    assert_eq!(counter.code_count, 2); // One in strong, one in table
    assert_eq!(counter.autolink_count, 1);
    assert_eq!(counter.html_count, 1);
    assert_eq!(counter.strikethrough_count, 1);
    assert_eq!(counter.link_ref_count, 1);
    assert_eq!(counter.footnote_ref_count, 0); // None in our test doc

    // Verify block counts
    assert!(counter.paragraph_count >= 5); // Multiple paragraphs in different blocks
    assert_eq!(counter.heading_count, 1);
    assert_eq!(counter.blockquote_count, 1);
    assert_eq!(counter.list_count, 1);
    assert_eq!(counter.table_count, 1);
    assert_eq!(counter.code_block_count, 1);
    assert_eq!(counter.html_block_count, 0);
    assert_eq!(counter.thematic_break_count, 0);
    assert_eq!(counter.footnote_def_count, 1);
    assert_eq!(counter.github_alert_count, 1);
    assert_eq!(counter.definition_count, 1);
}

#[test]
fn test_visitor_with_empty_document() {
    let doc = Document { blocks: vec![] };
    let mut collector = TextCollector { texts: Vec::new() };

    doc.visit_with(&mut collector);

    assert!(collector.texts.is_empty());
}

#[test]
fn test_visitor_with_empty_paragraph() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![])],
    };
    let mut collector = TextCollector { texts: Vec::new() };

    doc.visit_with(&mut collector);

    assert!(collector.texts.is_empty());
}

#[test]
fn test_visitor_with_only_empty_inlines() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Empty, Inline::Empty])],
    };
    let mut collector = TextCollector { texts: Vec::new() };

    doc.visit_with(&mut collector);

    assert!(collector.texts.is_empty());
}

#[test]
fn test_visitor_deep_nesting() {
    // Create deeply nested structure
    let doc = Document {
        blocks: vec![Block::BlockQuote(vec![Block::List(List {
            kind: ListKind::Bullet(ListBulletKind::Dash),
            items: vec![ListItem {
                task: None,
                blocks: vec![Block::BlockQuote(vec![Block::Paragraph(vec![
                    Inline::Link(Link {
                        destination: "http://example.com".to_string(),
                        title: None,
                        children: vec![Inline::Strong(vec![Inline::Emphasis(vec![Inline::Text(
                            "Deeply nested text".to_string(),
                        )])])],
                    }),
                ])])],
            }],
        })])],
    };

    let mut collector = TextCollector { texts: Vec::new() };
    doc.visit_with(&mut collector);

    assert_eq!(collector.texts.len(), 1);
    assert_eq!(collector.texts[0], "Deeply nested text");
}

#[test]
fn test_visitor_with_footnote_reference() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Text with footnote".to_string()),
            Inline::FootnoteReference("note1".to_string()),
        ])],
    };

    let mut counter = NodeCounter::new();
    doc.visit_with(&mut counter);

    assert_eq!(counter.text_count, 1);
    assert_eq!(counter.footnote_ref_count, 1);
}

#[test]
fn test_visitor_with_thematic_break() {
    let doc = Document {
        blocks: vec![
            Block::Paragraph(vec![Inline::Text("Before".to_string())]),
            Block::ThematicBreak,
            Block::Paragraph(vec![Inline::Text("After".to_string())]),
        ],
    };

    let mut counter = NodeCounter::new();
    doc.visit_with(&mut counter);

    assert_eq!(counter.paragraph_count, 2);
    assert_eq!(counter.thematic_break_count, 1);
    assert_eq!(counter.text_count, 2);
}

#[test]
fn test_visitor_with_html_block() {
    let doc = Document {
        blocks: vec![
            Block::HtmlBlock("<div>HTML content</div>".to_string()),
            Block::Paragraph(vec![Inline::Html("<span>Inline HTML</span>".to_string())]),
        ],
    };

    let mut counter = NodeCounter::new();
    doc.visit_with(&mut counter);

    assert_eq!(counter.html_block_count, 1);
    assert_eq!(counter.html_count, 1);
    assert_eq!(counter.paragraph_count, 1);
}
