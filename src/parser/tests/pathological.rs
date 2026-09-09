//! Regression tests for adversarial input: nested containers and inline delimiters
//! that used to take exponential time or overflow the stack.

use crate::ast::{Block, Inline};
use crate::parser::config::{ElementBehavior, MarkdownParserConfig};
use crate::parser::{parse_markdown, MarkdownParserState};
use std::time::{Duration, Instant};

/// Generous bound for a debug build; every case below runs in a few milliseconds
/// in release and used to take seconds or longer before the fix.
const TIME_LIMIT: Duration = Duration::from_secs(2);

fn parse(input: &str) -> Result<Vec<Block>, String> {
    parse_markdown(MarkdownParserState::default(), input)
        .map(|doc| doc.blocks)
        .map_err(|err| format!("{err:?}"))
}

fn assert_fast(label: &str, input: String) -> Result<Vec<Block>, String> {
    let started = Instant::now();
    let result = parse(&input);
    let elapsed = started.elapsed();
    assert!(
        elapsed < TIME_LIMIT,
        "{label}: {} bytes took {elapsed:?}",
        input.len()
    );
    result
}

fn is_too_large(result: &Result<Vec<Block>, String>) -> bool {
    matches!(result, Err(err) if err.contains("TooLarge"))
}

// --- containers -------------------------------------------------------------

#[test]
fn nested_blockquotes_are_linear() {
    let blocks = assert_fast("blockquote", format!("{}x", "> ".repeat(30))).unwrap();
    let mut depth = 0;
    let mut current = &blocks;
    while let [Block::BlockQuote(inner)] = current.as_slice() {
        depth += 1;
        current = inner;
    }
    assert_eq!(depth, 30);
    assert_eq!(current, &[Block::Paragraph(vec![Inline::Text("x".into())])]);
}

#[test]
fn nested_lists_are_linear() {
    assert_fast("bullet list", format!("{}x", "- ".repeat(30))).unwrap();
    assert_fast("ordered list", format!("{}x", "1. ".repeat(30))).unwrap();
}

#[test]
fn nested_mixed_containers_are_linear() {
    assert_fast("blockquote/list", format!("{}x", "> - ".repeat(15))).unwrap();
    assert_fast("footnote", format!("{}x", "[^a]: ".repeat(30))).unwrap();
}

#[test]
fn many_blockquote_lines_are_linear() {
    assert_fast("blockquote lines", "> a\n".repeat(20_000)).unwrap();
    assert_fast("list lines", "- a\n".repeat(20_000)).unwrap();
}

// --- inline -----------------------------------------------------------------

#[test]
fn unclosed_emphasis_is_linear() {
    assert_fast("stars", "*a ".repeat(5_000)).unwrap();
    assert_fast("underscores", "_a ".repeat(5_000)).unwrap();
    assert_fast(
        "mixed",
        format!("{}{}", "*a".repeat(5_000), "_b".repeat(5_000)),
    )
    .unwrap();
    assert_fast("far closer", format!("{}b*", "*a ".repeat(5_000))).unwrap();
    assert_fast(
        "nested strong",
        format!("{}a{}", "**".repeat(5_000), "**".repeat(5_000)),
    )
    .unwrap();
}

#[test]
fn brackets_are_linear() {
    assert_fast("open brackets", "[".repeat(20_000)).unwrap();
    assert_fast("unbalanced", format!("{}a]", "[".repeat(20_000))).unwrap();
    assert_fast(
        "nested labels",
        format!("{}a{}", "[".repeat(5_000), "]".repeat(5_000)),
    )
    .unwrap();
    assert_fast("links without closer", "[a](".repeat(5_000)).unwrap();
    assert_fast("images without closer", "![a ".repeat(5_000)).unwrap();
    assert_fast(
        "paren nest",
        format!("[a]({}b{})", "(".repeat(5_000), ")".repeat(5_000)),
    )
    .unwrap();
}

#[test]
fn nested_link_labels_do_not_explode_the_ast() {
    // A shortcut reference stores its label twice, so the AST doubles per level;
    // labels deeper than `MAX_LINK_LABEL_DEPTH` are left as text.
    let blocks = assert_fast(
        "deep labels",
        format!("{}a{}", "[".repeat(40), "]".repeat(40)),
    )
    .unwrap();
    let mut depth = 0;
    let mut current = match blocks.as_slice() {
        [Block::Paragraph(inlines)] => inlines.clone(),
        other => panic!("unexpected blocks: {other:?}"),
    };
    // Unmatched leading `[` become text before the first reference on each level.
    while let Some(Inline::LinkReference(reference)) = current
        .iter()
        .find(|inline| matches!(inline, Inline::LinkReference(_)))
    {
        depth += 1;
        current = reference.text.clone();
    }
    assert_eq!(depth, crate::parser::link_util::MAX_LINK_LABEL_DEPTH);
}

#[test]
fn html_and_entities_are_linear() {
    assert_fast("cdata", "<![CDATA[".repeat(5_000)).unwrap();
    assert_fast("entity", "&amp".repeat(20_000)).unwrap();
    assert_fast("autolink", "<a ".repeat(20_000)).unwrap();
}

// --- depth limit --------------------------------------------------------------

#[test]
fn default_depth_limit_is_32() {
    assert!(parse(&format!("{}x", "> ".repeat(32))).is_ok());
    assert!(is_too_large(&parse(&format!("{}x", "> ".repeat(33)))));
    assert!(parse(&format!("{}x", "- ".repeat(32))).is_ok());
    assert!(is_too_large(&parse(&format!("{}x", "- ".repeat(33)))));
}

#[test]
fn depth_limit_covers_inline_nesting() {
    // Blocks and inline elements share one depth counter: 32 quotes + emphasis = 33.
    let input = format!("{}*a*", "> ".repeat(32));
    assert!(is_too_large(&parse(&input)));
    let input = format!("{}*a*", "> ".repeat(31));
    assert!(parse(&input).is_ok());
    // Nested inline elements count one level each: 30 + 3 = 33.
    let input = format!("{}~~*_a_*~~", "> ".repeat(30));
    assert!(is_too_large(&parse(&input)));
    let input = format!("{}~~*_a_*~~", "> ".repeat(29));
    assert!(parse(&input).is_ok());
}

#[test]
fn depth_limit_is_configurable() {
    let config = MarkdownParserConfig::default().with_max_nesting_depth(3);
    let parse_with = |input: &str| {
        parse_markdown(MarkdownParserState::with_config(config.clone()), input)
            .map_err(|err| format!("{err:?}"))
    };
    assert!(parse_with(&"> ".repeat(3)).is_ok());
    assert!(parse_with(&format!("{}x", "> ".repeat(4)))
        .unwrap_err()
        .contains("TooLarge"));
}

#[test]
fn deep_nesting_does_not_overflow_a_small_stack() {
    let handle = std::thread::Builder::new()
        .stack_size(1 << 20)
        .spawn(|| {
            for input in [
                format!("{}x", "> ".repeat(10_000)),
                format!("{}x", "- ".repeat(10_000)),
                format!("{}a{}", "[".repeat(10_000), "]".repeat(10_000)),
                format!("{}a{}", "*".repeat(10_000), "*".repeat(10_000)),
                format!("[a]({}b{})", "(".repeat(10_000), ")".repeat(10_000)),
            ] {
                // Either a clean result or a depth error; never an abort.
                let _ = parse(&input);
            }
        })
        .unwrap();
    handle.join().unwrap();
}

// --- semantics preserved by the rewrite ----------------------------------------

fn paragraph(input: &str) -> Vec<Inline> {
    match parse(input).unwrap().as_slice() {
        [Block::Paragraph(inlines)] => inlines.clone(),
        other => panic!("expected a paragraph, got {other:?}"),
    }
}

#[test]
fn bare_marker_lines_stay_text() {
    assert_eq!(paragraph("a * b"), vec![Inline::Text("a * b".into())]);
    assert_eq!(paragraph("FOO_BAR"), vec![Inline::Text("FOO_BAR".into())]);
    assert_eq!(paragraph("xFOO_BAR"), vec![Inline::Text("xFOO_BAR".into())]);
    assert_eq!(paragraph("[ ]"), vec![Inline::Text("[ ]".into())]);
    assert_eq!(paragraph("**\\*"), vec![Inline::Text("**\\*".into())]);
    // Escapes are resolved at the start of a text element only (historical behaviour
    // the printer round-trip relies on).
    assert_eq!(paragraph("\\*x\\*"), vec![Inline::Text("*x\\*".into())]);
}

#[test]
fn empty_blockquote_interrupts_a_paragraph() {
    assert_eq!(
        parse("a\n>\nb").unwrap(),
        vec![
            Block::Paragraph(vec![Inline::Text("a".into())]),
            Block::BlockQuote(vec![]),
            Block::Paragraph(vec![Inline::Text("b".into())]),
        ]
    );
}

#[test]
fn bracketed_variants_keep_their_order_and_behaviors() {
    let link = |input: &str| paragraph(input).into_iter().next().unwrap();

    assert!(matches!(link("[foo](u)"), Inline::Link(_)));
    assert!(matches!(link("[^1](u)"), Inline::Link(_)));
    assert!(matches!(link("[^1]"), Inline::FootnoteReference(name) if name == "1"));
    assert!(matches!(link("[foo][bar]"), Inline::LinkReference(_)));
    assert!(matches!(link("[foo][]"), Inline::LinkReference(_)));
    assert!(matches!(link("[foo]"), Inline::LinkReference(_)));

    // Ignoring inline links falls through to the reference link, leaving `(u)` as text.
    let config = MarkdownParserConfig::default().with_inline_link_behavior(ElementBehavior::Ignore);
    let doc = parse_markdown(MarkdownParserState::with_config(config), "[foo](u)").unwrap();
    assert_eq!(
        doc.blocks,
        vec![Block::Paragraph(vec![
            Inline::LinkReference(crate::ast::LinkReference {
                label: vec![Inline::Text("foo".into())],
                text: vec![Inline::Text("foo".into())],
            }),
            Inline::Text("(u)".into()),
        ])]
    );

    // Ignoring footnote references makes `[^1]` a shortcut reference.
    let config = MarkdownParserConfig::default()
        .with_inline_footnote_reference_behavior(ElementBehavior::Ignore);
    let doc = parse_markdown(MarkdownParserState::with_config(config), "[^1]").unwrap();
    assert!(matches!(
        doc.blocks.as_slice(),
        [Block::Paragraph(inlines)] if matches!(inlines.as_slice(), [Inline::LinkReference(_)])
    ));
}

#[test]
fn link_titles_and_destinations_with_parens() {
    let link = |input: &str| match paragraph(input).into_iter().next().unwrap() {
        Inline::Link(link) => link,
        other => panic!("expected a link, got {other:?}"),
    };
    let l = link("[a](u(v) \"t\\\"x\")");
    assert_eq!(l.destination, "u(v)");
    assert_eq!(l.title.as_deref(), Some("t\"x"));
    let l = link("[a](u 'it\\'s')");
    assert_eq!(l.title.as_deref(), Some("it's"));
    let l = link("[a](u (paren))");
    assert_eq!(l.title.as_deref(), Some("paren"));
    // An unbalanced `)` ends the destination.
    assert_eq!(
        paragraph("[a](u) b"),
        vec![
            Inline::Link(crate::ast::Link {
                destination: "u".into(),
                title: None,
                children: vec![Inline::Text("a".into())],
            }),
            Inline::Text(" b".into()),
        ]
    );
}
