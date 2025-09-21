#![cfg(test)]
use rstest::rstest;

mod list;
mod text_formatting;

#[rstest(input,
         case("---"),
        case(
        r#"word1 word2"#),
        case(
        r#"paragraph1

paragraph2"#),
        case(
        r#"# heading1

## heading2

heading 3
=========="#),
        // case( "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."),
         case(r#" 100. item1 paragraph1
      
      item1 paragraph2
 101. item2 paragraph1
      
      item2 paragraph2
      
      item2 paragraph3
 102. item3 paragraph1
      
      item3 paragraph2"#),
        case(
        r#" 1. item 1
    
     * nested list item 1
     * nested list item 2
 2. item 2"#),
        case(
        r#"> line1 line1 line1 line1 line1 line1 line1 line1 line1 line1 line1 line1 line1
> line1 line1 line1 line1 line1 line1 line1 line1"#),
        case(
        r#"> line1 line1 line1 line1 line1 line1 line1 line1 line1 line1 line1 line1
> 
> > line1 line1 line1 line1 line1 line1 line1 line1 line1"#),
        case(
        r#"Это *курсив, но внутри **жирный и *обратно курсив*** снова жирный* конец."#),
        case(
        r#"Это \*не курсив\*, а просто звёздочки."#),
        case(
        r#"Вот [ссылка *с курсивом внутри*](https://example.com) и ещё текст."#),
        case(
        r#"Инлайн код `внутри *курсива*` не должен парситься как курсив."#),
        case(
        r#"Параграф первый с **жирным** текстом.

Параграф второй с *курсивом* и списком:

 - Первый пункт **жирный**
 - Второй пункт *курсивный*

Конец."#),
        case(
            r#"Список с задачами:

 - Первый пункт
 - [ ] Второй пункт
 - [X] Третий пункт

Конец."#),
        case(
        r#"> Список внутри цитаты:

>  - Пункт *первый*
>  - Пункт **второй**
>    
>     - Подпункт `третий`
> 
> Конец цитаты."#),
        case(
        r#"## Это *курсивный* заголовок с [ссылкой](https://example.com)"#),
        case(
        r#"Это просто текст** а потом *ещё* звёздочки."#),
        case(
        r#"[ссылка с `кодом` внутри](https://example.com)"#),
        case(
        r#"Здесь *курсив без конца и **жирный без конца"#),
        case(
        "**Всё жирное и *курсивное и `кодовое внутри курсивного` и снова курсивное* снова\nжирное**"),
        case(
        r#"[Ссылка с \*экранированной звездочкой\* внутри](https://example.com)"#),
        case(
        r#"Текст с сноской[^1].

[^1]: Это текст сноски."#),
        case(
        r#"Это *курсивный текст со сноской[^note]*.

[^note]: Сноска для курсивного текста."#),
        case(
        r#"[Ссылка с сноской[^linknote]](https://example.com)

[^linknote]: Сноска для ссылки."#),
        case(
        r#"# Заголовок со сноской[^headnote]

[^headnote]: Пояснение к заголовку."#),
        case(
        r#"| Заголовок 1 | Заголовок 2 | Заголовок 3 |
| ----------- | ----------: | :---------: |
| Ячейка 1    |    Ячейка 2 |  Ячейка 3   |
| Ячейка 4    |    Ячейка 5 |  Ячейка 6   |"#),
        case(
        r#"| **Заголовок 1** | Заголовок 2 | Заголовок 3 |
| --------------- | ----------: | :---------: |
| Ячейка 1        |    Ячейка 2 |  Ячейка 3   |
| Ячейка 4        |    Ячейка 5 |  Ячейка 6   |"#),
        case(
        r#"> | Заголовок 1 | Заголовок 2 | Заголовок 3 |
> | ----------- | ----------: | :---------: |
> | Ячейка 1    |    Ячейка 2 |  Ячейка 3   |
> | Ячейка 4    |    Ячейка 5 |  Ячейка 6   |"#),
        case(
            r#"> blockquote level 1
> 
> > blockquote level 2"#),
        case(
            r#"text

```rust
let s = "hello\n";

```"#),

        case(
            r#"Autolinks test: <http://example.com> and <johnlepikhin@gmail.com>"#),

        // GitHub Alert tests
        case(
            r#"> [!NOTE]
> This is a note"#),

        case(
            r#"> [!TIP]
> Here's a helpful tip with **bold** and *italic* text"#),

        case(
            r#"> [!IMPORTANT]
> This is important information
> 
> With multiple paragraphs"#),

        case(
            r#"> [!WARNING]
> Warning with a list:
> 
>  - First item
>  - Second item
>  - Third item"#),

        case(
            r#"> [!CAUTION]
> Caution alert with `inline code` and [a link](https://example.com)"#),

        // Edge case: Empty alert
        case(
            r#"> [!NOTE]"#),

        // Edge case: Alert with nested blockquote
        case(
            r#"> [!TIP]
> This contains a nested quote:
> 
> > Nested quote inside alert
> > 
> > Second line of nested quote"#),

        // Edge case: Alert with code block
        case(
            r#"> [!WARNING]
> Here's some code:
> 
> ```python
> def hello():
>     print("Hello, world!")
> ```
> 
> Be careful!"#),

        // Edge case: Multiple alerts in sequence
        case(
            r#"> [!NOTE]
> First alert

> [!TIP]
> Second alert"#),

        // Edge case: Alert with table
        case(
            r#"> [!IMPORTANT]
> Here's a table:
> 
> | Column 1 | Column 2 |
> | -------- | -------- |
> | Cell 1   | Cell 2   |"#),

        // Edge case: Alert with footnote
        case(
            r#"> [!CAUTION]
> Text with footnote[^1]

[^1]: This is the footnote"#),

        // Edge case: Alert with task list
        // Note: Current printer uses [X] for completed tasks - this is expected behavior
        case(
            r#"> [!TIP]
> Task list:
> 
>  - [X] Completed task
>  - [ ] Incomplete task
>  - [ ] Another task"#),

        // Edge case: Alert with horizontal rule
        case(
            r#"> [!NOTE]
> Before rule
> 
> ---
> 
> After rule"#),

        // Edge case: Alert with escaped content
        case(
            r#"> [!WARNING]
> This has \*escaped\* content and \[brackets\]"#),

        // Edge case: Alert with HTML entity
        // Note: Parser converts &copy; to © symbol
        case(
            r#"> [!IMPORTANT]
> Copyright © 2024"#),

        // Edge case: Alert with autolink
        case(
            r#"> [!CAUTION]
> Visit <https://example.com> for more info"#),

        // Edge case: Alert with strikethrough
        case(
            r#"> [!TIP]
> This is ~~deprecated~~ text"#),

        // Edge case: Alert with image
        case(
            r#"> [!NOTE]
> ![Alt text](image.png "Image title")"#),

        // Edge case: Alert with reference link
        case(
            r#"> [!WARNING]
> See [this link][ref] for details

[ref]: https://example.com"#),

        // Edge case: Alert with complex mixed content
        case(
            r#"> [!IMPORTANT]
> # Heading inside alert
> 
> This has **bold**, *italic*, `code`, and ~~strikethrough~~.
> 
>  - List item 1
>  - List item 2
> 
> ```js
> console.log("code block");
> ```
> 
> End of alert"#),

)]
fn symmetric_round_trip(input: &str) {
    let config = crate::printer::config::Config::default();
    let doc = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();
    println!("{:?} => {:#?}", input, doc);
    let result = crate::printer::render_markdown(&doc, config);
    assert_eq!(input, result);
}
