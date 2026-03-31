use crate::ast::*;
use crate::plaintext_printer::ToDoc;
use pretty::{Arena, DocAllocator, DocBuilder};

fn is_visible_block(block: &Block) -> bool {
    !matches!(block, Block::HtmlBlock(_) | Block::Definition(_) | Block::Empty)
}

impl<'a> ToDoc<'a> for Vec<Block> {
    fn to_doc(
        &self,
        state: &'a crate::plaintext_printer::State<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        let visible: Vec<_> = self.iter().filter(|b| is_visible_block(b)).collect();
        let len = visible.len();
        let mut result = state.arena.nil();
        for (i, block) in visible.into_iter().enumerate() {
            result = result.append(block.to_doc(state));
            if i + 1 < len {
                result = result.append(state.arena.hardline()).append(state.arena.hardline());
            }
        }
        result
    }
}

impl<'a> ToDoc<'a> for Block {
    fn to_doc(
        &self,
        state: &'a crate::plaintext_printer::State<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        match self {
            Block::Paragraph(inlines) => inlines.to_doc(state),
            Block::Heading(v) => v.content.to_doc(state),
            Block::ThematicBreak => state.arena.text("---"),
            Block::BlockQuote(blocks) => blocks.to_doc(state),
            Block::List(v) => v.to_doc(state),
            Block::CodeBlock(v) => state.arena.text(v.literal.trim_end_matches('\n').to_string()),
            Block::HtmlBlock(_) => state.arena.nil(),
            Block::Definition(_) => state.arena.nil(),
            Block::Table(v) => v.to_doc(state),
            Block::FootnoteDefinition(def) => def.to_doc(state),
            Block::GitHubAlert(alert) => alert.to_doc(state),
            Block::Empty => state.arena.nil(),
        }
    }
}

impl<'a> ToDoc<'a> for List {
    fn to_doc(
        &self,
        state: &'a crate::plaintext_printer::State<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        let mut result = state.arena.nil();
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                result = result.append(state.arena.hardline());
            }

            let prefix = match self.kind {
                ListKind::Ordered(ListOrderedKindOptions { start }) => {
                    format!("{}. ", start.saturating_add(i as u64))
                }
                ListKind::Bullet(_) => "- ".into(),
            };

            let task = match item.task {
                Some(TaskState::Complete) => "[x] ",
                Some(TaskState::Incomplete) => "[ ] ",
                None => "",
            };

            let content = item.blocks.to_doc(state);
            result = result
                .append(state.arena.text(prefix))
                .append(state.arena.text(task))
                .append(content);
        }
        result
    }
}

impl<'a> ToDoc<'a> for Table {
    fn to_doc(
        &self,
        state: &'a crate::plaintext_printer::State<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        let mut result = state.arena.nil();
        for (i, row) in self.rows.iter().enumerate() {
            if i > 0 {
                result = result.append(state.arena.hardline());
            }
            let mut row_doc = state.arena.nil();
            for (j, cell) in row.iter().enumerate() {
                if j > 0 {
                    row_doc = row_doc.append(state.arena.text(" | "));
                }
                row_doc = row_doc.append(cell.to_doc(state));
            }
            result = result.append(row_doc);
        }
        result
    }
}

impl<'a> ToDoc<'a> for FootnoteDefinition {
    fn to_doc(
        &self,
        state: &'a crate::plaintext_printer::State<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        let index = match state.get_footnote_index(&self.label) {
            Some(v) => v,
            None => return state.arena.nil(),
        };
        state
            .arena
            .text(format!("[{index}] "))
            .append(self.blocks.to_doc(state))
    }
}

impl<'a> ToDoc<'a> for GitHubAlert {
    fn to_doc(
        &self,
        state: &'a crate::plaintext_printer::State<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        let title = match &self.alert_type {
            GitHubAlertType::Note => "Note",
            GitHubAlertType::Tip => "Tip",
            GitHubAlertType::Important => "Important",
            GitHubAlertType::Warning => "Warning",
            GitHubAlertType::Caution => "Caution",
            GitHubAlertType::Custom(s) => s.as_str(),
        };
        state
            .arena
            .text(format!("[{title}]"))
            .append(state.arena.hardline())
            .append(self.blocks.to_doc(state))
    }
}
