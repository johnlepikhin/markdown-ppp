#![cfg(test)]
use rstest::rstest;

mod list;

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

)]
fn symmetric_round_trip(input: &str) {
    let config = crate::printer::config::Config::default();
    let doc = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();
    println!("{:?} => {:#?}", input, doc);
    let result = crate::printer::render_markdown(&doc, config);
    assert_eq!(input, result);
}
