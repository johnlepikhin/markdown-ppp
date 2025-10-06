use crate::ast::*;
use crate::latex_printer::config::CodeBlockStyle;
use crate::latex_printer::util::{command, environment, escape_latex};
use crate::latex_printer::ToDoc;
use pretty::{Arena, DocAllocator, DocBuilder};

impl<'a> ToDoc<'a> for Vec<Block> {
    fn to_doc(&self, state: &'a crate::latex_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        let refs: Vec<_> = self.iter().collect();
        refs.to_doc(state)
    }
}

impl<'a> ToDoc<'a> for Vec<&Block> {
    fn to_doc(&self, state: &'a crate::latex_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        let mut acc = state.arena.nil();
        for (i, block) in self.iter().enumerate() {
            if i > 0 {
                acc = acc
                    .append(state.arena.hardline())
                    .append(state.arena.hardline());
            }
            acc = acc.append(block.to_doc(state));
        }
        acc
    }
}

impl<'a> ToDoc<'a> for Block {
    fn to_doc(&self, state: &'a crate::latex_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        match self {
            Block::Paragraph(inlines) => inlines.to_doc(state),

            Block::Heading(heading) => {
                let level = match heading.kind {
                    HeadingKind::Atx(level) => level,
                    HeadingKind::Setext(SetextHeading::Level1) => 1,
                    HeadingKind::Setext(SetextHeading::Level2) => 2,
                };

                let cmd_name = match level {
                    1 => "section",
                    2 => "subsection",
                    3 => "subsubsection",
                    4 => "paragraph",
                    5 => "subparagraph",
                    _ => "subparagraph", // 6+ levels default to subparagraph
                };

                command(&state.arena, cmd_name, &[], heading.content.to_doc(state))
            }

            Block::ThematicBreak => command(&state.arena, "hrule", &[], state.arena.nil()),

            Block::BlockQuote(blocks) => {
                environment(&state.arena, "quote", None, blocks.to_doc(state))
            }

            Block::List(list) => list.to_doc(state),

            Block::CodeBlock(code_block) => render_code_block(state, code_block),

            Block::HtmlBlock(html) => {
                // Render HTML as escaped text in LaTeX
                state.arena.text(escape_latex(html))
            }

            Block::Definition(_) => {
                // Link definitions are handled during inline processing
                state.arena.nil()
            }

            Block::Table(table) => table.to_doc(state),

            Block::FootnoteDefinition(def) => {
                if let Some(index) = state.get_footnote_index(&def.label) {
                    command(
                        &state.arena,
                        "footnotetext",
                        &[],
                        state
                            .arena
                            .text(format!("[{index}] "))
                            .append(def.blocks.to_doc(state)),
                    )
                } else {
                    state.arena.nil()
                }
            }

            Block::GitHubAlert(alert) => {
                // Render GitHub alerts as footnotes for now
                let alert_type = match alert.alert_type {
                    GitHubAlertType::Note => "Note".to_owned(),
                    GitHubAlertType::Tip => "Tip".to_owned(),
                    GitHubAlertType::Important => "Important".to_owned(),
                    GitHubAlertType::Warning => "Warning".to_owned(),
                    GitHubAlertType::Caution => "Caution".to_owned(),
                    GitHubAlertType::Custom(ref s) => s.clone(),
                };

                command(
                    &state.arena,
                    "footnote",
                    &[],
                    state
                        .arena
                        .text(format!("{alert_type}: "))
                        .append(alert.blocks.to_doc(state)),
                )
            }

            Block::Empty => state.arena.nil(),
        }
    }
}

impl<'a> ToDoc<'a> for List {
    fn to_doc(&self, state: &'a crate::latex_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        let env_name = match self.kind {
            ListKind::Ordered(_) => "enumerate",
            ListKind::Bullet(_) => "itemize",
        };

        let mut content = state.arena.nil();
        for item in &self.items {
            content = content.append(item.to_doc(state));
        }

        environment(&state.arena, env_name, None, content)
    }
}

impl<'a> ToDoc<'a> for ListItem {
    fn to_doc(&self, state: &'a crate::latex_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        let mut item_content = state.arena.text(r"\item ");

        // Handle task list checkbox
        if let Some(task_state) = self.task {
            let checkbox = match task_state {
                TaskState::Complete => r"$\boxtimes$ ",
                TaskState::Incomplete => r"$\square$ ",
            };
            item_content = item_content.append(state.arena.text(checkbox));
        }

        item_content = item_content.append(self.blocks.to_doc(state));
        item_content.append(state.arena.hardline())
    }
}

fn render_code_block<'a>(
    state: &'a crate::latex_printer::State<'a>,
    code_block: &CodeBlock,
) -> DocBuilder<'a, Arena<'a>, ()> {
    match state.config.code_block_style {
        CodeBlockStyle::Verbatim => environment(
            &state.arena,
            "verbatim",
            None,
            state.arena.text(code_block.literal.clone()),
        ),

        CodeBlockStyle::Listings => {
            let options = match &code_block.kind {
                CodeBlockKind::Fenced { info: Some(lang) } => Some(format!("language={lang}")),
                _ => None,
            };

            environment(
                &state.arena,
                "lstlisting",
                options.as_deref(),
                state.arena.text(code_block.literal.clone()),
            )
        }

        CodeBlockStyle::Minted => {
            let lang = match &code_block.kind {
                CodeBlockKind::Fenced { info: Some(lang) } => lang.as_str(),
                _ => "text",
            };

            let mut result = state.arena.text(format!(r"\begin{{minted}}{{{lang}}}"));
            result = result.append(state.arena.hardline());
            result = result.append(state.arena.text(code_block.literal.clone()));
            result = result.append(state.arena.hardline());
            result = result.append(state.arena.text(r"\end{minted}"));
            result
        }
    }
}
