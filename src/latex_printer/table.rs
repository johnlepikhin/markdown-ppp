use crate::ast::*;
use crate::latex_printer::config::TableStyle;
use crate::latex_printer::util::environment;
use crate::latex_printer::ToDoc;
use pretty::{Arena, DocAllocator, DocBuilder};

impl<'a> ToDoc<'a> for Table {
    fn to_doc(&self, state: &'a crate::latex_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        match state.config.table_style {
            TableStyle::Tabular => render_tabular(state, self),
            TableStyle::Longtabu => render_longtabu(state, self),
            TableStyle::Booktabs => render_booktabs(state, self),
        }
    }
}

fn render_tabular<'a>(
    state: &'a crate::latex_printer::State<'a>,
    table: &Table,
) -> DocBuilder<'a, Arena<'a>, ()> {
    let column_spec = create_column_spec(&table.alignments);
    let options = Some(column_spec.as_str());

    let mut content = state.arena.nil();

    // Render header row
    if let Some(header_row) = table.rows.first() {
        content = content.append(render_table_row(state, header_row));
        content = content.append(state.arena.hardline());
        content = content.append(state.arena.text(r"\hline"));
        content = content.append(state.arena.hardline());
    }

    // Render data rows
    for row in table.rows.iter().skip(1) {
        content = content.append(render_table_row(state, row));
        content = content.append(state.arena.hardline());
    }

    environment(&state.arena, "tabular", options, content)
}

fn render_longtabu<'a>(
    state: &'a crate::latex_printer::State<'a>,
    table: &Table,
) -> DocBuilder<'a, Arena<'a>, ()> {
    let column_spec = create_column_spec(&table.alignments);
    let options_str = format!("X[l] to \\textwidth {{{column_spec}}}");
    let options = Some(options_str.as_str());

    let mut content = state.arena.nil();

    // Render header row
    if let Some(header_row) = table.rows.first() {
        content = content.append(render_table_row(state, header_row));
        content = content.append(state.arena.hardline());
        content = content.append(state.arena.text(r"\\ \hline"));
        content = content.append(state.arena.hardline());
    }

    // Render data rows
    for row in table.rows.iter().skip(1) {
        content = content.append(render_table_row(state, row));
        content = content.append(state.arena.text(r" \\"));
        content = content.append(state.arena.hardline());
    }

    environment(&state.arena, "longtabu", options, content)
}

fn render_booktabs<'a>(
    state: &'a crate::latex_printer::State<'a>,
    table: &Table,
) -> DocBuilder<'a, Arena<'a>, ()> {
    let column_spec = create_column_spec(&table.alignments);
    let options = Some(column_spec.as_str());

    let mut content = state.arena.text(r"\toprule");
    content = content.append(state.arena.hardline());

    // Render header row
    if let Some(header_row) = table.rows.first() {
        content = content.append(render_table_row(state, header_row));
        content = content.append(state.arena.hardline());
        content = content.append(state.arena.text(r"\midrule"));
        content = content.append(state.arena.hardline());
    }

    // Render data rows
    for row in table.rows.iter().skip(1) {
        content = content.append(render_table_row(state, row));
        content = content.append(state.arena.hardline());
    }

    content = content.append(state.arena.text(r"\bottomrule"));

    environment(&state.arena, "tabular", options, content)
}

fn render_table_row<'a>(
    state: &'a crate::latex_printer::State<'a>,
    row: &TableRow,
) -> DocBuilder<'a, Arena<'a>, ()> {
    let mut result = state.arena.nil();

    for (i, cell) in row.iter().enumerate() {
        if i > 0 {
            result = result.append(state.arena.text(" & "));
        }
        result = result.append(cell.to_doc(state));
    }

    result.append(state.arena.text(r" \\"))
}

fn create_column_spec(alignments: &[Alignment]) -> String {
    alignments
        .iter()
        .map(|align| match align {
            Alignment::Left | Alignment::None => "l",
            Alignment::Center => "c",
            Alignment::Right => "r",
        })
        .collect::<String>()
}
