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
fn render_to_html(#[case] input: &str, #[case] expected: &str) {
    let config = crate::html_printer::config::Config::default();
    let ast = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();
    println!("{input:?} => {ast:#?}");
    let result = crate::html_printer::render_html(&ast, config);
    assert_eq!(expected, result);
}
