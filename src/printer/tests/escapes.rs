#![cfg(test)]
use rstest::rstest;

/// The parser resolves backslash escapes in an image description, so the printer has
/// to put them back: `parse -> print -> parse` must reach the same AST even when the
/// description contains `]` or `\`.
#[rstest(
    input,
    case(r"![a\](x)](u.jpg)"),
    case(r"![Face N \(TD+\)](u.jpg)"),
    case(r"![a\nb](u.jpg)"),
    case(r"![a\\b](u.jpg)"),
    case(r#"![Dent d'Herens: Face N \(TD+\)](https://example/x.jpg)"#)
)]
fn image_alt_survives_round_trip(input: &str) {
    assert_round_trip(input);
}

/// Titles are stored unescaped too, so a quote inside one must not end it on re-parse.
#[rstest(
    input,
    case(r#"[x](u "it's")"#),
    case(r#"[x](u "a \" b")"#),
    case(r#"[x](u 'a " b')"#),
    case(r#"[x](u "a \\ b")"#),
    case(r#"![x](u "a \" b")"#),
    case("[x][1]\n\n[1]: u \"a \\\" b\"")
)]
fn link_title_survives_round_trip(input: &str) {
    assert_round_trip(input);
}

/// A destination that the bare form cannot carry has to be printed as `<...>`.
#[rstest(
    input,
    case("[x](<a b>)"),
    case("[x](<>)"),
    case(r"[x](<a\<b>)"),
    case(r"[x](<a\>b>)"),
    case("[x](<a(b>)"),
    case("[x](a(b)c)"),
    case(r"[x](a\(b)"),
    case("[x](<a b> \"t\")"),
    case("[x][1]\n\n[1]: <a b> \"t\"")
)]
fn link_destination_survives_round_trip(input: &str) {
    assert_round_trip(input);
}

fn assert_round_trip(input: &str) {
    let state = || crate::parser::MarkdownParserState::default();
    let doc = crate::parser::parse_markdown(state(), input).unwrap();
    let printed = crate::printer::render_markdown(&doc, crate::printer::config::Config::default());
    let reparsed = crate::parser::parse_markdown(state(), &printed).unwrap();

    assert_eq!(doc, reparsed, "printed as {printed:?}");
}
