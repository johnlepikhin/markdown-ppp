use crate::ast::*;
use crate::printer::{inline::ToDocInline, ToDoc};
use core::iter::Iterator;
use pretty::{Arena, DocAllocator, DocBuilder};
use std::rc::Rc;

impl<'a> ToDoc<'a> for Table {
    fn to_doc(
        &self,
        _config: Rc<crate::printer::config::Config>,
        arena: &'a Arena<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        if self.rows.is_empty() {
            return arena.nil();
        }

        let content = table_content(self);
        let columns_width = columns_width(&content, &self.alignments);
        let header = row_to_doc(&content[0], &columns_width, &self.alignments, arena);
        let separator = alignments_row_to_doc(&self.alignments, &columns_width, arena);

        let body = content
            .iter()
            .skip(1)
            .map(|row| row_to_doc(row, &columns_width, &self.alignments, arena))
            .collect::<Vec<_>>();

        let mut rows = vec![header, separator];
        rows.extend(body);

        // Render table with unlimited width to prevent line wrapping
        let table_doc = arena.intersperse(rows, arena.hardline());
        let mut buf = Vec::new();
        table_doc.render(usize::MAX, &mut buf).unwrap();
        let table_string = String::from_utf8(buf).unwrap();
        arena.text(table_string)
    }
}

fn alignments_row_to_doc<'a>(
    alignments: &[Alignment],
    columns_width: &[usize],
    arena: &'a Arena<'a>,
) -> DocBuilder<'a, Arena<'a>, ()> {
    let mut acc = arena.text("| ");
    for (i, alignment) in alignments.iter().enumerate() {
        if i > 0 {
            acc = acc.append(arena.text(" | "))
        }
        let column_width = columns_width.get(i).unwrap_or(&3);
        acc = acc.append(alignment_to_doc(*alignment, *column_width, arena))
    }
    acc.append(arena.text(" |"))
}

fn alignment_to_doc<'a>(
    alignment: Alignment,
    column_width: usize,
    arena: &'a Arena<'a>,
) -> DocBuilder<'a, Arena<'a>, ()> {
    match alignment {
        Alignment::None | Alignment::Left => {
            let repeat = if column_width > 1 { column_width } else { 1 };
            arena.text("-".repeat(repeat))
        }
        Alignment::Center => {
            let repeat = if column_width > 2 {
                column_width - 2
            } else {
                1
            };
            arena
                .text(":")
                .append(arena.text("-".repeat(repeat)))
                .append(arena.text(":"))
        }
        Alignment::Right => {
            let repeat = if column_width > 1 {
                column_width - 1
            } else {
                1
            };
            arena.text("-".repeat(repeat)).append(arena.text(":"))
        }
    }
}

fn row_to_doc<'a>(
    row: &[String],
    columns_width: &[usize],
    alignments: &[Alignment],
    arena: &'a Arena<'a>,
) -> DocBuilder<'a, Arena<'a>, ()> {
    let mut acc = arena.text("| ");
    for (i, cell) in row.iter().enumerate() {
        if i > 0 {
            acc = acc.append(arena.text(" | "))
        }
        let alignment = alignments.get(i).cloned().unwrap_or_default();
        // Unreachable code, because we already checked the length of the row
        let column_width = columns_width.get(i).unwrap_or(&3);
        acc = acc.append(cell_to_doc(cell, *column_width, alignment, arena))
    }
    acc.append(arena.text(" |"))
}

fn cell_to_doc<'a>(
    cell: &str,
    column_width: usize,
    alignment: Alignment,
    arena: &'a Arena<'a>,
) -> DocBuilder<'a, Arena<'a>, ()> {
    let content = match alignment {
        Alignment::None | Alignment::Left => {
            format!(
                "{}{}",
                cell,
                " ".repeat(column_width - cell.chars().count())
            )
        }
        Alignment::Center => {
            let padding = column_width - cell.chars().count();
            let left_padding = padding / 2;
            let right_padding = padding - left_padding;
            format!(
                "{}{}{}",
                " ".repeat(left_padding),
                cell,
                " ".repeat(right_padding)
            )
        }
        Alignment::Right => format!(
            "{}{}",
            " ".repeat(column_width - cell.chars().count()),
            cell
        ),
    };
    arena.text(content)
}

fn columns_width(table: &[Vec<String>], alignments: &[Alignment]) -> Vec<usize> {
    let mut widths = Vec::new();
    for i in 0..table[0].len() {
        let width = column_width(table, alignments, i);
        widths.push(width);
    }
    widths
}

fn column_width(table: &[Vec<String>], alignments: &[Alignment], column_index: usize) -> usize {
    let content_width = column_content_width(table, column_index);
    let alignment_width = match alignments.get(column_index) {
        Some(Alignment::Left) => 1,
        Some(Alignment::Center) => 3,
        Some(Alignment::Right) => 2,
        Some(Alignment::None) => 1,
        None => 1,
    };
    if content_width > alignment_width {
        content_width
    } else {
        alignment_width
    }
}

fn column_content_width(table: &[Vec<String>], column_index: usize) -> usize {
    let mut max_width = 0;
    for row in table {
        if column_index < row.len() {
            let cell_width = row[column_index].chars().count();
            if cell_width > max_width {
                max_width = cell_width;
            }
        }
    }

    max_width
}

fn table_content(table: &Table) -> Vec<Vec<String>> {
    let mut content = Vec::new();
    for row in &table.rows {
        let mut row_content = Vec::new();
        for cell in row {
            let cell_content = render_cell(cell);
            row_content.push(cell_content);
        }
        content.push(row_content);
    }
    content
}

fn render_cell(doc: &Vec<Inline>) -> String {
    let tmp_arena = Arena::new();
    let doc = doc.to_doc_inline(false, &tmp_arena);

    let mut buf = Vec::new();
    doc.render(usize::MAX, &mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}
