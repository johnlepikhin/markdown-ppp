[![crates.io](https://img.shields.io/crates/v/markdown-ppp.svg)](https://crates.io/crates/markdown-ppp)
[![docs.rs](https://docs.rs/markdown-ppp/badge.svg)](https://docs.rs/markdown-ppp)
[![CI](https://github.com/johnlepikhin/markdown-ppp/actions/workflows/ci.yml/badge.svg)](https://github.com/johnlepikhin/markdown-ppp/actions)
[![License: MIT](https://img.shields.io/crates/l/markdown-ppp.svg)](https://github.com/johnlepikhin/markdown-ppp/blob/main/LICENSE)

# markdown-ppp

**markdown-ppp** is a feature-rich, flexible, and lightweight Rust library for parsing and processing Markdown documents.

It provides a clean, well-structured Abstract Syntax Tree (AST) for parsed documents, making it suitable for pretty-printing, analyzing, transforming, or rendering Markdown.

---

## ✨ Features

- **Markdown Parsing** — Full Markdown parsing support with strict AST structure.
- **Pretty-printing and processing** — Build, modify, and reformat Markdown easily.
- **Render to HTML** — Convert Markdown AST to HTML.
- **GitHub Alerts** — Native support for GitHub-style markdown alerts ([!NOTE], [!TIP], [!WARNING], etc.).
- **Modular design** — You can disable parsing entirely and use only the AST types.

---

## 📦 Installation

Add the crate using Cargo:

```bash
cargo add markdown-ppp
```

If you want **only** the AST definitions without parsing functionality, disable default features manually:

```toml
[dependencies]
markdown-ppp = { version = "0.1.0", default-features = false }
```

---

## 🛠 Usage

### Parsing Markdown

The main entry point for parsing is the `parse_markdown` function, available at:

```rust
pub fn parse_markdown(
    state: MarkdownParserState,
    input: &str,
) -> Result<Document, nom::Err<nom::error::Error<&str>>>
```

Example:

```rust
use markdown_ppp::parse::parse_markdown;
use markdown_ppp::ast::Document;
use std::rc::Rc;

fn main() {
    let state = markdown_ppp::parse::MarkdownParserState::new();
    let input = "# Hello, World!";

    match parse_markdown(Rc::new(state), input) {
        Ok(document) => {
            println!("Parsed document: {:?}", document);
        }
        Err(err) => {
            eprintln!("Failed to parse Markdown: {:?}", err);
        }
    }
}
```

### MarkdownParserState

The `MarkdownParserState` controls parsing behavior and can be customized.

You can create a default state easily:

```rust
use markdown_ppp::parser::config::*;

let state = MarkdownParserState::default();
```

Alternatively, you can configure it manually by providing a `MarkdownParserConfig`:

```rust
use markdown_ppp::parser::config::*;

let config = MarkdownParserConfig::default()
    .with_block_blockquote_behavior(ElementBehavior::Ignore);

let ast = parse_markdown(MarkdownParserState::with_config(config), "hello world")?;
```

This allows you to control how certain Markdown elements are parsed or ignored.

---

## 🧩 Customizing the parsing behavior

You can control how individual Markdown elements are parsed at a fine-grained level using the [`MarkdownParserConfig`](https://docs.rs/markdown-ppp/latest/markdown_ppp/parser/config/struct.MarkdownParserConfig.html) API.

Each element type (block-level or inline-level) can be configured with an `ElementBehavior`:

```rust
pub enum ElementBehavior<ELT> {
    /// The parser will parse the element normally.
    Parse,

    /// The parser will ignore the element and not parse it. In this case, alternative
    /// parsers will be tried.
    Ignore,

    /// Parse the element but do not include it in the output.
    Skip,

    /// Parse the element and apply a custom function to it.
    Map(ElementMapFn<ELT>),

    /// Parse the element and apply a custom function to it which returns an array of elements.
    FlatMap(ElementFlatMapFn<ELT>),
}
```

These behaviors can be set via builder-style methods on the config. For example, to skip parsing of thematic breaks and transform blockquotes:

```rust
use markdown_ppp::parser::config::*;
use markdown_ppp::ast::Block;

let config = MarkdownParserConfig::default()
    .with_block_thematic_break_behavior(ElementBehavior::Skip)
    .with_block_blockquote_behavior(ElementBehavior::Map(|mut bq: Block| {
        // Example transformation: replace all blockquotes with empty paragraphs
        Block::Paragraph(vec![])
    }));

let ast = parse_markdown(MarkdownParserState::with_config(config), input)?;
```

This mechanism allows you to override, filter, or completely redefine how each Markdown element is treated during parsing, giving you deep control over the resulting AST.

### Registering custom parsers

You can also register your own custom block-level or inline-level parsers by providing parser functions via configuration. These parsers are executed before the built-in ones and can be used to support additional syntax or override behavior.

To register a custom block parser:

```rust
use markdown_ppp::parser::config::*;
use markdown_ppp::ast::Block;
use std::{rc::Rc, cell::RefCell};
use nom::IResult;

let custom_block: CustomBlockParserFn = Rc::new(RefCell::new(Box::new(|input: &str| {
    if input.starts_with("::note") {
        let block = Block::Paragraph(vec!["This is a note block".into()]);
        Ok(("", block))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)))
    }
})));

let config = MarkdownParserConfig::default()
    .with_custom_block_parser(custom_block);
```

Similarly, to register a custom inline parser:

```rust
use markdown_ppp::parser::config::*;
use markdown_ppp::ast::Inline;
use std::{rc::Rc, cell::RefCell};
use nom::IResult;

let custom_inline: CustomInlineParserFn = Rc::new(RefCell::new(Box::new(|input: &str| {
    if input.starts_with("@@") {
        Ok((&input[2..], Inline::Text("custom-inline".into())))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)))
    }
})));

let config = config.with_custom_inline_parser(custom_inline);
```

This extensibility allows you to integrate domain-specific syntax and behaviors into the Markdown parser while reusing the base logic and AST structure provided by `markdown-ppp`., filter, or completely redefine how each Markdown element is treated during parsing.

---

## 📄 AST structure

The complete Markdown Abstract Syntax Tree (AST) is defined inside the module `markdown_ppp::ast`.


The `Document` struct represents the root node, and from there you can traverse the full tree of blocks and inlines, such as headings, paragraphs, lists, emphasis, and more.

You can use the AST independently without the parsing functionality by disabling default features.

## 🖨️ Pretty-printing (AST → Markdown)

You can convert an AST (`Document`) back into a formatted Markdown string using the `render_markdown` function from the `printer` module.

This feature is enabled by default via the `printer` feature.

### Basic example

```rust
use markdown_ppp::printer::render_markdown;
use markdown_ppp::printer::config::Config;
use markdown_ppp::ast::Document;

// Assume you already have a parsed or constructed Document
let document = Document::default();

// Render it back to a Markdown string with default configuration
let markdown_output = render_markdown(&document, Config::default());

println!("{}", markdown_output);
```

This will format the Markdown with a default line width of 80 characters.

### Customizing output width

You can control the maximum width of lines in the generated Markdown by customizing the Config:

```rust
use markdown_ppp::printer::render_markdown;
use markdown_ppp::printer::config::Config;
use markdown_ppp::ast::Document;

// Set a custom maximum width, e.g., 120 characters
let config = Config::default().with_width(120);

let markdown_output = render_markdown(&Document::default(), config);

println!("{}", markdown_output);
```

This is useful if you want to control wrapping behavior or generate more compact or expanded Markdown documents.

## 🖨️ Pretty-printing (AST → HTML)

You can convert an AST (`Document`) back into a formatted HTML string using the `render_html` function from the `html_printer` module.

This feature is enabled by default via the `html-printer` feature.

### Basic example

```rust
use markdown_ppp::html_printer::render_html;
use markdown_ppp::html_printer::config::Config;
use markdown_ppp::ast::Document;

let config = Config::default();
let ast = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), "# Hello, World!")
    .unwrap();

println!("{}", render_html(&ast, config));
```

---

## 🔧 Optional features

| Feature         | Description                                                        |
|:----------------|:-------------------------------------------------------------------|
| `parser`        | Enables Markdown parsing support. Enabled by default.              |
| `printer`       | Enables AST → Markdown string conversion. Enabled by default.      |
| `html-printer`  | Enables AST → HTML string conversion. Enabled by default.          |
| `ast-serde`     | Adds `Serialize` and `Deserialize` traits to all AST types via `serde`. Disabled by default. |

If you only need the AST types without parsing functionality, you can add the crate without default features:

```bash
cargo add --no-default-features markdown-ppp
```

If you want to disable Markdown generation (AST → Markdown string conversion), disable the `printer` feature manually:

```bash
cargo add markdown-ppp --no-default-features --features parser
```

---

## 📚 Documentation

- [API Docs on docs.rs](https://docs.rs/markdown-ppp)
- [AI-generated documentation](https://deepwiki.com/johnlepikhin/markdown-ppp)
- [GitHub Alerts Usage](docs/github_alerts.md)

---

## 📝 License

Licensed under the [MIT License](LICENSE).

