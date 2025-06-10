use crate::ast::{Block, Inline};
use nom::{
    branch::alt,
    character::complete::{anychar, line_ending, not_line_ending, space0},
    combinator::{eof, fail, not, recognize, value},
    multi::{many0, many1},
    sequence::{preceded, terminated},
    IResult, Parser,
};

pub(crate) fn eof_or_eol(input: &str) -> IResult<&str, &str> {
    alt((line_ending, eof)).parse(input)
}

pub(crate) fn many_empty_lines0(input: &str) -> IResult<&str, Vec<&str>> {
    many0(preceded(space0, eof_or_eol)).parse(input)
}

pub(crate) fn not_eof_or_eol1(input: &str) -> IResult<&str, &str> {
    recognize(many1(preceded(not(eof_or_eol), anychar))).parse(input)
}

pub(crate) fn not_eof_or_eol0(input: &str) -> IResult<&str, &str> {
    alt((not_line_ending, eof)).parse(input)
}

pub(crate) fn line_terminated<'a, O, P>(
    inner: P,
) -> impl Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>
where
    P: Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>,
{
    terminated(inner, eof_or_eol)
}

// pub(crate) fn logged<'a, O, P>(
//     message: &'static str,
//     mut inner: P,
// ) -> impl Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>
// where
//     P: Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>,
//     O: std::fmt::Debug,
// {
//     move |input: &'a str| {
//         println!("Logged: {message}: {:?}", input);
//         let r = inner.parse(input);
//         println!("Logged out: {message}: {:?}", r);
//         r
//     }
// }

pub(crate) fn conditional<'a, O, P>(
    behavior: crate::parser::config::ElementBehavior<O>,
    default: Vec<O>,
    mut inner: P,
) -> impl Parser<&'a str, Output = Vec<O>, Error = nom::error::Error<&'a str>>
where
    P: Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>,
    O: Clone,
{
    move |input: &'a str| {
        let inner1 = |s: &'a str| inner.parse(s);
        match &behavior {
            crate::parser::config::ElementBehavior::Ignore => fail().map(|v| vec![v]).parse(input),
            crate::parser::config::ElementBehavior::Parse => inner1.map(|v| vec![v]).parse(input),
            crate::parser::config::ElementBehavior::Skip => {
                value(default.clone(), inner1.map(|v| vec![v])).parse(input)
            }
            crate::parser::config::ElementBehavior::Map(f) => {
                let (i, o) = inner.parse(input)?;
                let mut f1 = (**f).borrow_mut();
                let mapped = vec![(f1.as_mut())(o)];
                Ok((i, mapped))
            }
            crate::parser::config::ElementBehavior::FlatMap(f) => {
                let (i, o) = inner.parse(input)?;
                let mut f1 = (**f).borrow_mut();
                let mapped = (f1.as_mut())(o);
                Ok((i, mapped))
            }
        }
    }
}

pub(crate) fn conditional_block_unit<'a, P>(
    behavior: crate::parser::config::ElementBehavior<Block>,
    mut inner: P,
) -> impl Parser<&'a str, Output = Vec<()>, Error = nom::error::Error<&'a str>>
where
    P: Parser<&'a str, Output = (), Error = nom::error::Error<&'a str>>,
{
    let behavior: crate::parser::config::ElementBehavior<()> = match behavior {
        super::config::ElementBehavior::Parse => super::config::ElementBehavior::Parse,
        super::config::ElementBehavior::Ignore => super::config::ElementBehavior::Ignore,
        super::config::ElementBehavior::Skip => super::config::ElementBehavior::Skip,
        super::config::ElementBehavior::Map(_) => super::config::ElementBehavior::Parse,
        super::config::ElementBehavior::FlatMap(_) => super::config::ElementBehavior::Parse,
    };
    move |input: &'a str| {
        let inner1 = |s: &'a str| inner.parse(s);
        conditional(behavior.clone(), vec![()], inner1).parse(input)
    }
}

pub(crate) fn conditional_inline_unit<'a, P>(
    behavior: crate::parser::config::ElementBehavior<Inline>,
    mut inner: P,
) -> impl Parser<&'a str, Output = Vec<()>, Error = nom::error::Error<&'a str>>
where
    P: Parser<&'a str, Output = (), Error = nom::error::Error<&'a str>>,
{
    let behavior: crate::parser::config::ElementBehavior<()> = match behavior {
        super::config::ElementBehavior::Parse => super::config::ElementBehavior::Parse,
        super::config::ElementBehavior::Ignore => super::config::ElementBehavior::Ignore,
        super::config::ElementBehavior::Skip => super::config::ElementBehavior::Skip,
        super::config::ElementBehavior::Map(_) => super::config::ElementBehavior::Parse,
        super::config::ElementBehavior::FlatMap(_) => super::config::ElementBehavior::Parse,
    };
    move |input: &'a str| {
        let inner1 = |s: &'a str| inner.parse(s);
        conditional(behavior.clone(), vec![()], inner1).parse(input)
    }
}

pub(crate) fn conditional_block<'a, P>(
    behavior: crate::parser::config::ElementBehavior<Block>,
    mut inner: P,
) -> impl Parser<&'a str, Output = Vec<Block>, Error = nom::error::Error<&'a str>>
where
    P: Parser<&'a str, Output = Block, Error = nom::error::Error<&'a str>>,
{
    move |input: &'a str| {
        let inner1 = |s: &'a str| inner.parse(s);
        conditional(behavior.clone(), vec![Block::Empty], inner1).parse(input)
    }
}

pub(crate) fn conditional_inline<'a, P>(
    behavior: crate::parser::config::ElementBehavior<Inline>,
    mut inner: P,
) -> impl Parser<&'a str, Output = Vec<Inline>, Error = nom::error::Error<&'a str>>
where
    P: Parser<&'a str, Output = Inline, Error = nom::error::Error<&'a str>>,
{
    move |input: &'a str| {
        let inner1 = |s: &'a str| inner.parse(s);
        conditional(behavior.clone(), vec![Inline::Empty], inner1).parse(input)
    }
}
