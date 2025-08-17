use crate::ast::*;
use crate::html_printer::util::tag;
use crate::html_printer::ToDoc;
use pretty::{Arena, DocAllocator, DocBuilder};

impl<'a> ToDoc<'a> for Vec<Block> {
    fn to_doc(&self, state: &'a crate::html_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        state
            .arena
            .concat(self.iter().map(|block| block.to_doc(state)))
    }
}

impl<'a> ToDoc<'a> for Vec<&Block> {
    fn to_doc(&self, state: &'a crate::html_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        state
            .arena
            .concat(self.iter().map(|block| block.to_doc(state)))
    }
}

impl<'a> ToDoc<'a> for Block {
    fn to_doc(&self, state: &'a crate::html_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        match self {
            Block::Paragraph(inlines) => {
                let inner = state
                    .arena
                    .concat(inlines.iter().map(|inline| inline.to_doc(state)));
                tag(state, "p", Vec::new(), inner)
            }
            Block::Heading(v) => {
                let htag = match v.kind {
                    HeadingKind::Atx(1) => "h1",
                    HeadingKind::Atx(2) => "h2",
                    HeadingKind::Atx(3) => "h3",
                    HeadingKind::Atx(4) => "h4",
                    HeadingKind::Atx(5) => "h5",
                    HeadingKind::Atx(_) => "h6",
                    HeadingKind::Setext(SetextHeading::Level1) => "h1",
                    HeadingKind::Setext(SetextHeading::Level2) => "h2",
                };
                let inner = state
                    .arena
                    .concat(v.content.iter().map(|inline| inline.to_doc(state)));
                tag(state, htag, Vec::new(), inner)
            }
            Block::ThematicBreak => tag(state, "hr", Vec::new(), state.arena.nil()),
            Block::BlockQuote(inner) => {
                let inner = state
                    .arena
                    .concat(inner.iter().map(|inline| inline.to_doc(state)));
                tag(state, "blockquote", Vec::new(), inner)
            }
            Block::List(v) => v.to_doc(state),
            Block::CodeBlock(v) => v.to_doc(state),
            Block::HtmlBlock(html) => state.arena.text(html.clone()),
            Block::Definition(_) => state.arena.nil(),
            Block::Empty => state.arena.nil(),
            Block::Table(v) => v.to_doc(state),
            Block::FootnoteDefinition(def) => def.to_doc(state),
            Block::GitHubAlert(alert) => alert.to_doc(state),
        }
    }
}

impl<'a> ToDoc<'a> for List {
    fn to_doc(&self, state: &'a crate::html_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        let items = state
            .arena
            .concat(self.items.iter().map(|item| item.to_doc(state)));
        match self.kind {
            ListKind::Ordered(ListOrderedKindOptions { start }) => tag(
                state,
                "ol",
                vec![("start".to_owned(), format!("{}", start))],
                items,
            ),
            ListKind::Bullet(kind) => {
                let style = match kind {
                    ListBulletKind::Dash => "markdown-list-kind-dash",
                    ListBulletKind::Star => "markdown-list-kind-star",
                    ListBulletKind::Plus => "markdown-list-kind-plus",
                };
                tag(
                    state,
                    "ul",
                    vec![("class".to_owned(), style.to_owned())],
                    items,
                )
            }
        }
    }
}

impl<'a> ToDoc<'a> for ListItem {
    fn to_doc(&self, state: &'a crate::html_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        let task = match self.task {
            Some(TaskState::Complete) => tag(
                state,
                "span",
                vec![("class".to_owned(), "markdown-list-task-complete".to_owned())],
                state.arena.text("[X] "),
            ),
            Some(TaskState::Incomplete) => tag(
                state,
                "span",
                vec![(
                    "class".to_owned(),
                    "markdown-list-task-incomplete".to_owned(),
                )],
                state.arena.text("[ ] "),
            ),
            None => state.arena.nil(),
        };
        let content = task.append(
            state
                .arena
                .concat(self.blocks.iter().map(|block| block.to_doc(state))),
        );

        tag(state, "li", vec![], content)
    }
}

impl<'a> ToDoc<'a> for CodeBlock {
    fn to_doc(&self, state: &'a crate::html_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        tag(
            state,
            "pre",
            Vec::new(),
            tag(
                state,
                "code",
                Vec::new(),
                state
                    .arena
                    .text(crate::html_printer::util::escape(&self.literal)),
            ),
        )
    }
}

impl<'a> ToDoc<'a> for Table {
    fn to_doc(&self, state: &'a crate::html_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        let first_row = table_row_to_doc(state, self.rows.first().unwrap(), "th", &self.alignments);
        let mut acc = state.arena.nil();
        for row in self.rows.iter().skip(1) {
            acc = acc.append(table_row_to_doc(state, row, "td", &self.alignments));
        }

        let content =
            tag(state, "thead", Vec::new(), first_row).append(tag(state, "tbody", Vec::new(), acc));

        tag(state, "table", Vec::new(), content)
    }
}

fn table_row_to_doc<'a>(
    state: &'a crate::html_printer::State<'a>,
    row: &TableRow,
    row_tag: &'static str,
    alignments: &[Alignment],
) -> DocBuilder<'a, Arena<'a>, ()> {
    let mut acc = state.arena.nil();
    for (i, cell) in row.iter().enumerate() {
        let alignment = match alignments.get(i) {
            Some(Alignment::Left) | Some(Alignment::None) => "left",
            Some(Alignment::Right) => "right",
            Some(Alignment::Center) => "center",
            None => "left",
        };
        let alignment_class = format!("markdown-table-align-{}", alignment);
        let attributes = vec![("class".to_owned(), alignment_class.to_owned())];
        acc = acc.append(tag(state, row_tag, attributes, cell.to_doc(state)));
    }

    tag(state, "tr", Vec::new(), acc)
}

impl<'a> ToDoc<'a> for FootnoteDefinition {
    fn to_doc(&self, state: &'a crate::html_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        let index = match state.get_footnote_index(&self.label) {
            Some(v) => v,
            None => return state.arena.nil(),
        };
        tag(
            state,
            "div",
            vec![(
                "class".to_owned(),
                "markdown-footnote-definition".to_owned(),
            )],
            tag(
                state,
                "span",
                vec![(
                    "class".to_owned(),
                    "markdown-footnote-definition-index".to_owned(),
                )],
                state.arena.text(format!("{index}. ")),
            )
            .append(tag(
                state,
                "span",
                vec![(
                    "class".to_owned(),
                    "markdown-footnote-definition-content".to_owned(),
                )],
                self.blocks.to_doc(state),
            )),
        )
    }
}
