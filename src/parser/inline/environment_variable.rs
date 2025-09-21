use crate::ast::Inline;
use nom::{
    branch::alt,
    character::complete::{alpha1, alphanumeric1, char},
    combinator::{map, recognize, verify},
    multi::many0,
    sequence::pair,
    IResult, Parser,
};

pub(crate) fn environment_variable(input: &str) -> IResult<&str, Inline> {
    map(
        verify(
            recognize(pair(
                alpha1,
                many0(alt((alphanumeric1, recognize(char('_'))))),
            )),
            |s: &str| is_likely_env_var(s),
        ),
        |var_name: &str| Inline::Text(var_name.to_string()),
    )
    .parse(input)
}

fn is_likely_env_var(s: &str) -> bool {
    // Must contain at least one underscore
    if !s.contains('_') {
        return false;
    }

    // Must not start or end with underscore
    if s.starts_with('_') || s.ends_with('_') {
        return false;
    }

    // Must not have consecutive underscores
    if s.contains("__") {
        return false;
    }

    // Should be reasonable length (heuristic)
    if s.len() < 3 || s.len() > 50 {
        return false;
    }

    true
}
