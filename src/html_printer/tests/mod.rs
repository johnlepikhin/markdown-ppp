#![cfg(test)]
use rstest::rstest;

#[rstest]
#[case("Hello, world!", "<p>Hello, world!</p>")]
#[case("Hello, **world**!", "<p>Hello, <b>world</b>!</p>")]
#[case("Hello, *world*!", "<p>Hello, <em>world</em>!</p>")]
#[case("Hello, __world__!", "<p>Hello, <b>world</b>!</p>")]
#[case("Hello, _world_!", "<p>Hello, <em>world</em>!</p>")]
#[case("Hello, ~~world~~!", "<p>Hello, <s>world</s>!</p>")]
#[case(
    "1. Item 1\n2. Item 2",
    "<ol start=\"1\"><li><p>Item 1</p></li><li><p>Item 2</p></li></ol>"
)]
#[case(
    "* Item 1\n* Item 2",
    "<ul class=\"markdown-list-kind-star\"><li><p>Item 1</p></li><li><p>Item 2</p></li></ul>"
)]
#[case("`code`", "<p><code>code</code></p>")]
#[case("```rust\nfn main() {}\n```", "<pre><code>fn main() {}</code></pre>")]
#[case(
    "[Google][1]\n\n[1]: https://www.google.com 'Search engine'",
    "<p><a href=\"https://www.google.com\" title=\"Search engine\">Google</a></p>"
)]
#[case(
    "Hello[^1]\n\n[^1]: This is a footnote.",
    "<p>Hello<a class=\"markdown-footnote-reference\" href=\"#1\">[1]</a></p><div class=\"markdown-footnote-definition\"><span class=\"markdown-footnote-definition-index\">1. </span><span class=\"markdown-footnote-definition-content\"><p>This is a footnote.</p></span></div>"
)]
#[case(
    "![alt text](https://example.com/image.png)",
    "<p><img src=\"https://example.com/image.png\" alt=\"alt text\"></img></p>"
)]
#[case(
    "| Header 1 | Header 2 |
| --- | --: |
| Row 1 Col 1 | Row 1 Col 2 |
| Row 2 Col 1 | Col 2       |",
    "<table><thead><tr><th class=\"markdown-table-align-left\">Header 1</th><th class=\"markdown-table-align-right\">Header 2</th></tr></thead><tbody><tr><td class=\"markdown-table-align-left\">Row 1 Col 1</td><td class=\"markdown-table-align-right\">Row 1 Col 2</td></tr><tr><td class=\"markdown-table-align-left\">Row 2 Col 1</td><td class=\"markdown-table-align-right\">Col 2</td></tr></tbody></table>"
)]
// Attribute values are escaped exactly once: `tag()` owns the escaping and callers
// pass them raw. A URL with a query string and an apostrophe in a title used to come
// out as `&amp;amp;` / `&amp;apos;`.
#[case(
    "[x](https://e.com/?a=1&b=2 \"it's\")",
    "<p><a href=\"https://e.com/?a=1&amp;b=2\" title=\"it&apos;s\">x</a></p>"
)]
#[case(
    "<https://e.com/?a=1&b=2>",
    "<p><a href=\"https://e.com/?a=1&amp;b=2\">https://e.com/?a=1&amp;b=2</a></p>"
)]
#[case(
    "[x][1]\n\n[1]: https://e.com/?a=1&b=2 'it&apos;s'",
    "<p><a href=\"https://e.com/?a=1&amp;b=2\" title=\"it&amp;apos;s\">x</a></p>"
)]
// Backslash escapes in an image description are resolved by the parser, and the
// resulting text is escaped once for HTML.
#[case(
    r#"![Dent d'Herens: Face N \(TD+\), arêtes NW et SW \(VN\)](https://example/x.jpg)"#,
    "<p><img src=\"https://example/x.jpg\" alt=\"Dent d&apos;Herens: Face N (TD+), arêtes NW et SW (VN)\"></img></p>"
)]
#[case(r"![a\](x)](u.jpg)", "<p><img src=\"u.jpg\" alt=\"a](x)\"></img></p>")]
#[case(
    r#"[x](u "a \" b")"#,
    "<p><a href=\"u\" title=\"a &quot; b\">x</a></p>"
)]
fn render_to_html(#[case] input: &str, #[case] expected: &str) {
    let config = crate::html_printer::config::Config::default();
    let ast = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();
    println!("{input:?} => {ast:#?}");
    let result = crate::html_printer::render_html(&ast, config);
    assert_eq!(expected, result);
}

/// A custom alert name cannot carry markup through the parser, but the AST is public
/// and can be built by hand, so the title has to be escaped as a text node.
#[test]
fn render_custom_github_alert_title_is_escaped() {
    use crate::ast::{Block, Document, GitHubAlert, GitHubAlertType, Inline};

    let ast = Document {
        blocks: vec![Block::GitHubAlert(GitHubAlert {
            alert_type: GitHubAlertType::Custom("<script>alert(1)</script>".to_owned()),
            blocks: vec![Block::Paragraph(vec![Inline::Text("body".to_owned())])],
        })],
    };
    let result =
        crate::html_printer::render_html(&ast, crate::html_printer::config::Config::default());

    assert!(
        result.contains("&lt;script&gt;alert(1)&lt;/script&gt;"),
        "custom alert title must be escaped, got: {result}"
    );
    assert!(!result.contains("<script>"), "raw markup leaked: {result}");
}
