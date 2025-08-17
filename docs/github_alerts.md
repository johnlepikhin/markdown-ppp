# GitHub Alerts Support

markdown-ppp now includes native support for GitHub-style markdown alerts. These are special blockquotes that render with distinctive styling and icons.

## Supported Alert Types

- `[!NOTE]` - Blue note alert with info icon
- `[!TIP]` - Green tip alert with lightbulb icon  
- `[!IMPORTANT]` - Blue important alert with report icon
- `[!WARNING]` - Yellow warning alert with alert triangle icon
- `[!CAUTION]` - Red caution alert with stop sign icon

## Usage

GitHub alerts are written as blockquotes with a special marker on the first line:

```markdown
> [!NOTE]
> This is a note with **formatting** support.
> It can span multiple lines.

> [!WARNING]
> This is a warning alert.
> 
> It can have multiple paragraphs.
```

## Example

```rust
use markdown_ppp::parser::{parse_markdown, MarkdownParserState};
use markdown_ppp::html_printer;

let markdown = r#"
> [!TIP]
> Here's a helpful tip:
> - Use bullet points
> - Include `code`
"#;

let document = parse_markdown(MarkdownParserState::default(), markdown).unwrap();
let html = html_printer::render_html(&document, html_printer::config::Config::default());
```

## HTML Output

GitHub alerts are rendered as `<div>` elements with appropriate CSS classes:

```html
<div class="markdown-alert markdown-alert-tip">
  <p class="markdown-alert-title">
    <svg class="octicon octicon-light-bulb mr-2">...</svg>
    Tip
  </p>
  <p>Here's a helpful tip:</p>
  <ul>
    <li>Use bullet points</li>
    <li>Include <code>code</code></li>
  </ul>
</div>
```

## Configuration

GitHub alerts are enabled by default. You can disable them using the parser configuration:

```rust
use markdown_ppp::parser::{MarkdownParserState, MarkdownParserConfig};
use markdown_ppp::parser::config::ElementBehavior;

let config = MarkdownParserConfig::default()
    .with_block_github_alert_behavior(ElementBehavior::Ignore);

let state = MarkdownParserState::new(config);
```

## CSS Styling

To style GitHub alerts like GitHub does, you can use CSS like this:

```css
.markdown-alert {
  padding: 0.5rem 1rem;
  margin-bottom: 1rem;
  border-left: 0.25rem solid;
  border-radius: 0.25rem;
}

.markdown-alert-note {
  border-left-color: #0969da;
  background-color: #e6f3ff;
}

.markdown-alert-tip {
  border-left-color: #1a7f37;
  background-color: #e6ffed;
}

.markdown-alert-important {
  border-left-color: #0969da;
  background-color: #e6f3ff;
}

.markdown-alert-warning {
  border-left-color: #bf8700;
  background-color: #fff8c5;
}

.markdown-alert-caution {
  border-left-color: #d1242f;
  background-color: #ffebe9;
}

.markdown-alert-title {
  display: flex;
  align-items: center;
  font-weight: 600;
  margin-bottom: 0.25rem;
}

.octicon {
  fill: currentColor;
  margin-right: 0.5rem;
}
```