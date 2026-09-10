use crate::parser::util::char_m_n;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{
        alpha1, alphanumeric1, anychar, char, line_ending, one_of, satisfy, space0, space1,
    },
    combinator::{eof, not, opt, peek, recognize, value, verify},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn html_block(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&str) -> IResult<&str, &str> {
    move |input: &str| {
        alt((
            html_block1(state.clone()),
            html_block2(state.clone()),
            html_block3(state.clone()),
            html_block4(state.clone()),
            html_block5(state.clone()),
            html_block6(state.clone()),
            html_block7(state.clone()),
        ))
        .parse(input)
    }
}

fn html_block1(_state: Rc<MarkdownParserState>) -> impl FnMut(&str) -> IResult<&str, &str> {
    move |input: &str| {
        let tag_variant_parser = || {
            alt((
                tag_no_case("script"),
                tag_no_case("pre"),
                tag_no_case("style"),
            ))
        };

        let end_parser = || delimited(tag("</"), tag_variant_parser(), char('>'));

        preceded(
            char_m_n(0, 3, ' '),
            recognize((
                char('<'),
                tag_variant_parser(),
                alt((
                    value((), char(' ')),
                    value((), char('>')),
                    value((), line_ending),
                )),
                many0(pair(peek(not(end_parser())), anychar)),
                end_parser(),
            )),
        )
        .parse(input)
    }
}

fn html_block2(_state: Rc<MarkdownParserState>) -> impl FnMut(&str) -> IResult<&str, &str> {
    move |input: &str| {
        preceded(
            char_m_n(0, 3, ' '),
            recognize((
                tag("<!--"),
                many0(pair(peek(not(tag("-->"))), anychar)),
                tag("-->"),
            )),
        )
        .parse(input)
    }
}

fn html_block3(_state: Rc<MarkdownParserState>) -> impl FnMut(&str) -> IResult<&str, &str> {
    move |input: &str| {
        preceded(
            char_m_n(0, 3, ' '),
            recognize((
                tag("<?"),
                many0(pair(peek(not(tag("?>"))), anychar)),
                tag("?>"),
            )),
        )
        .parse(input)
    }
}

fn html_block4(_state: Rc<MarkdownParserState>) -> impl FnMut(&str) -> IResult<&str, &str> {
    move |input: &str| {
        preceded(
            char_m_n(0, 3, ' '),
            recognize((
                tag("<!"),
                satisfy(|c| c.is_ascii_uppercase()),
                many0(pair(peek(not(char('>'))), anychar)),
                tag(">"),
            )),
        )
        .parse(input)
    }
}

fn html_block5(_state: Rc<MarkdownParserState>) -> impl FnMut(&str) -> IResult<&str, &str> {
    move |input: &str| {
        preceded(
            char_m_n(0, 3, ' '),
            recognize((
                tag("<![CDATA["),
                many0(pair(peek(not(tag("]]>"))), anychar)),
                tag("]]>"),
            )),
        )
        .parse(input)
    }
}

fn html_block6(_state: Rc<MarkdownParserState>) -> impl FnMut(&str) -> IResult<&str, &str> {
    move |input: &str| {
        let tag_variant = alt((
            alt((
                tag_no_case("address"),
                tag_no_case("article"),
                tag_no_case("aside"),
                tag_no_case("base"),
                tag_no_case("basefont"),
                tag_no_case("blockquote"),
                tag_no_case("body"),
                tag_no_case("caption"),
                tag_no_case("center"),
                tag_no_case("col"),
                tag_no_case("colgroup"),
            )),
            alt((
                tag_no_case("dd"),
                tag_no_case("details"),
                tag_no_case("dialog"),
                tag_no_case("dir"),
                tag_no_case("div"),
                tag_no_case("dl"),
                tag_no_case("dt"),
                tag_no_case("fieldset"),
                tag_no_case("figcaption"),
                tag_no_case("figure"),
                tag_no_case("footer"),
                tag_no_case("form"),
                tag_no_case("frame"),
                tag_no_case("frameset"),
            )),
            alt((
                tag_no_case("h1"),
                tag_no_case("h2"),
                tag_no_case("h3"),
                tag_no_case("h4"),
                tag_no_case("h5"),
                tag_no_case("h6"),
                tag_no_case("head"),
                tag_no_case("header"),
                tag_no_case("hr"),
                tag_no_case("html"),
                tag_no_case("iframe"),
                tag_no_case("legend"),
            )),
            alt((
                tag_no_case("li"),
                tag_no_case("link"),
                tag_no_case("main"),
                tag_no_case("menu"),
                tag_no_case("menuitem"),
                tag_no_case("nav"),
                tag_no_case("noframes"),
                tag_no_case("ol"),
                tag_no_case("optgroup"),
                tag_no_case("option"),
                tag_no_case("p"),
                tag_no_case("param"),
            )),
            alt((
                tag_no_case("section"),
                tag_no_case("source"),
                tag_no_case("summary"),
                tag_no_case("table"),
                tag_no_case("tbody"),
                tag_no_case("td"),
                tag_no_case("tfoot"),
                tag_no_case("th"),
                tag_no_case("thead"),
                tag_no_case("title"),
                tag_no_case("tr"),
                tag_no_case("track"),
                tag_no_case("ul"),
            )),
        ));
        let end_parser = || {
            alt((
                value((), terminated(line_ending, (space0, line_ending))),
                value((), eof),
            ))
        };

        preceded(
            char_m_n(0, 3, ' '),
            recognize((
                alt((value((), tag("</")), value((), char('<')))),
                tag_variant,
                alt((
                    value((), char(' ')),
                    value((), line_ending),
                    value((), tag("/>")),
                    value((), char('>')),
                )),
                many0(pair(peek(not(end_parser())), anychar)),
                opt(line_ending),
            )),
        )
        .parse(input)
    }
}

fn html_block7(_state: Rc<MarkdownParserState>) -> impl FnMut(&str) -> IResult<&str, &str> {
    move |input: &str| {
        let end_parser = || {
            alt((
                value((), (line_ending, space0, line_ending)),
                value((), eof),
            ))
        };

        preceded(
            char_m_n(0, 3, ' '),
            recognize((
                alt((
                    complete_open_html_tag(&["script", "pre", "style"]),
                    complete_closing_html_tag,
                )),
                alt((value((), line_ending), value((), char(' ')))),
                many0(pair(peek(not(end_parser())), anychar)),
                end_parser(),
            )),
        )
        .parse(input)
    }
}

fn complete_open_html_tag(
    restricted_tags: &'static [&'static str],
) -> impl FnMut(&str) -> IResult<&str, &str> {
    move |input: &str| {
        recognize((
            char('<'),
            verify(html_tag_name, |s: &str| {
                !restricted_tags
                    .iter()
                    .any(|tag| tag.eq_ignore_ascii_case(s))
            }),
            many0(html_tag_attribute),
            space0,
            opt(char('/')),
            char('>'),
        ))
        .parse(input)
    }
}

fn complete_closing_html_tag(input: &str) -> IResult<&str, &str> {
    recognize((tag("</"), html_tag_name, space0, char('>'))).parse(input)
}

fn html_tag_name(input: &str) -> IResult<&str, &str> {
    recognize((
        alpha1,
        many0(alt((value((), char('-')), value((), alphanumeric1)))),
    ))
    .parse(input)
}

fn html_tag_attribute(input: &str) -> IResult<&str, &str> {
    recognize((
        space1,
        html_tag_attribute_name,
        opt(html_tag_attribute_value_specification),
    ))
    .parse(input)
}

fn html_tag_attribute_name(input: &str) -> IResult<&str, &str> {
    recognize((
        alt((value((), alpha1), value((), one_of("_:")))),
        many0(alt((value((), one_of("_.:-")), value((), alphanumeric1)))),
    ))
    .parse(input)
}

fn html_tag_attribute_value_specification(input: &str) -> IResult<&str, &str> {
    recognize((space0, char('='), space0, html_tag_attribute_value)).parse(input)
}

fn html_tag_attribute_value(input: &str) -> IResult<&str, &str> {
    alt((
        html_tag_attribute_value_unquoted,
        html_tag_attribute_value_single_quoted,
        html_tag_attribute_value_double_quoted,
    ))
    .parse(input)
}

fn html_tag_attribute_value_unquoted(input: &str) -> IResult<&str, &str> {
    recognize(many1(pair(
        peek(not(alt((value((), space1), value((), one_of("\"'=<>`")))))),
        anychar,
    )))
    .parse(input)
}

fn html_tag_attribute_value_single_quoted(input: &str) -> IResult<&str, &str> {
    recognize(delimited(
        char('\''),
        pair(peek(not(char('\''))), anychar),
        char('\''),
    ))
    .parse(input)
}

fn html_tag_attribute_value_double_quoted(input: &str) -> IResult<&str, &str> {
    recognize(delimited(
        char('"'),
        pair(peek(not(char('"'))), anychar),
        char('"'),
    ))
    .parse(input)
}
