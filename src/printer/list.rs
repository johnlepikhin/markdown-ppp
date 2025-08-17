use crate::ast::*;
use crate::printer::ToDoc;
use pretty::{Arena, DocAllocator, DocBuilder};
use std::rc::Rc;

impl<'a> ToDoc<'a> for List {
    fn to_doc(
        &self,
        config: Rc<crate::printer::config::Config>,
        arena: &'a Arena<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        let mut counter = if let ListKind::Ordered(v) = &self.kind {
            v.start
        } else {
            0
        };
        let prefix_length = match &self.kind {
            ListKind::Bullet(ListBulletKind::Dash) => 2 + config.spaces_before_list_item, // <space>-<space>
            ListKind::Bullet(ListBulletKind::Star) => 2 + config.spaces_before_list_item, // <space>*<space>
            ListKind::Bullet(ListBulletKind::Plus) => 2 + config.spaces_before_list_item, // <space>+<space>
            ListKind::Ordered(v) => {
                let last = v.start + self.items.len() as u64 - 1;
                let digits = last.to_string().len();
                digits + 2 + config.spaces_before_list_item // <space>1.<space>
            }
        };
        let items = self.items.iter().map(|item| {
            let marker = match self.kind {
                ListKind::Bullet(ListBulletKind::Dash) => "-".to_owned(),
                ListKind::Bullet(ListBulletKind::Star) => "*".to_owned(),
                ListKind::Bullet(ListBulletKind::Plus) => "+".to_owned(),
                ListKind::Ordered(_) => {
                    let r = format!("{counter}.");
                    counter += 1;
                    r
                }
            };

            let task_list_marker = match item.task {
                Some(TaskState::Complete) => arena.text("[X]").append(arena.space()),
                Some(TaskState::Incomplete) => arena.text("[ ]").append(arena.space()),
                None => arena.nil(),
            };

            let indent = " ".repeat(config.spaces_before_list_item);

            arena
                .text(indent)
                .append(arena.text(marker.clone()))
                .append(arena.space())
                .append(task_list_marker)
                .append(
                    item.blocks
                        .to_doc(config.clone(), arena)
                        .nest(prefix_length as isize)
                        .group(),
                )
        });

        arena.intersperse(items, arena.hardline())
    }
}
