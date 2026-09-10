use crate::ast::{Block, Inline};
use nom::{
    branch::alt,
    character::complete::{line_ending, space0},
    combinator::{eof, fail},
    multi::many0,
    sequence::{preceded, terminated},
    IResult, Parser,
};

/// `many_m_n(min, max, char(c))` without the `Vec<char>`: the matched run as a slice.
/// The count of `c` is the slice length, since `c` is ASCII.
pub(crate) fn char_m_n(min: usize, max: usize, c: char) -> impl Fn(&str) -> IResult<&str, &str> {
    debug_assert!(c.is_ascii());
    move |input: &str| {
        let bytes = input.as_bytes();
        let limit = bytes.len().min(max);
        let mut n = 0;
        while n < limit && bytes[n] == c as u8 {
            n += 1;
        }
        if n < min {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::ManyMN,
            )));
        }
        Ok((&input[n..], &input[..n]))
    }
}

pub(crate) fn eof_or_eol(input: &str) -> IResult<&str, &str> {
    alt((line_ending, eof)).parse(input)
}

pub(crate) fn many_empty_lines0(input: &str) -> IResult<&str, Vec<&str>> {
    many0(preceded(space0, eof_or_eol)).parse(input)
}

/// One or more characters up to (not including) the first line ending (`\n` or
/// `\r\n`) or the end of input. A lone `\r` is an ordinary character.
pub(crate) fn not_eof_or_eol1(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();
    let mut end = bytes
        .iter()
        .position(|&b| b == b'\n')
        .unwrap_or(bytes.len());
    if end < bytes.len() && end > 0 && bytes[end - 1] == b'\r' {
        end -= 1;
    }
    if end == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Many1,
        )));
    }
    Ok((&input[end..], &input[..end]))
}

/// Zero or more characters up to (not including) the first line ending (`\n` or
/// `\r\n`) or the end of input. Like nom's `not_line_ending`, a lone `\r` is an error.
pub(crate) fn not_eof_or_eol0(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();
    let end = bytes
        .iter()
        .position(|&b| b == b'\n' || b == b'\r')
        .unwrap_or(bytes.len());
    if end < bytes.len() && bytes[end] == b'\r' && bytes.get(end + 1) != Some(&b'\n') {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    Ok((&input[end..], &input[..end]))
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

/// What one element parser produces: usually one element, sometimes several (a
/// `FlatMap` behavior, a custom parser) or none (`Skip`). Avoids a heap-allocated
/// `Vec` for the common single element.
#[derive(Clone, Debug)]
pub(crate) enum Pieces<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> Pieces<T> {
    /// Append the pieces to `out`.
    pub(crate) fn extend_into(self, out: &mut Vec<T>) {
        match self {
            Pieces::One(item) => out.push(item),
            Pieces::Many(items) => out.extend(items),
        }
    }
}

/// Apply an [`ElementBehavior`](crate::parser::config::ElementBehavior) to `inner`.
///
/// `default` is only evaluated for `Skip`, so the common `Parse` path costs no
/// allocation beyond the returned vector; this closure runs at every input position
/// for every inline alternative.
pub(crate) fn conditional<'a, O, P>(
    behavior: crate::parser::config::ElementBehavior<O>,
    default: impl Fn() -> O,
    mut inner: P,
) -> impl Parser<&'a str, Output = Pieces<O>, Error = nom::error::Error<&'a str>>
where
    P: Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>,
{
    move |input: &'a str| {
        let mut inner1 = |s: &'a str| inner.parse(s);
        match &behavior {
            crate::parser::config::ElementBehavior::Ignore => fail().parse(input),
            crate::parser::config::ElementBehavior::Parse => inner1.map(Pieces::One).parse(input),
            crate::parser::config::ElementBehavior::Skip => {
                let (i, _) = inner1(input)?;
                Ok((i, Pieces::One(default())))
            }
            crate::parser::config::ElementBehavior::Map(f) => {
                let (i, o) = inner.parse(input)?;
                let mut f1 = (**f).borrow_mut();
                let mapped = (f1.as_mut())(o);
                Ok((i, Pieces::One(mapped)))
            }
            crate::parser::config::ElementBehavior::FlatMap(f) => {
                let (i, o) = inner.parse(input)?;
                let mut f1 = (**f).borrow_mut();
                let mapped = (f1.as_mut())(o);
                Ok((i, Pieces::Many(mapped)))
            }
        }
    }
}

pub(crate) fn conditional_block_unit<'a, P>(
    behavior: crate::parser::config::ElementBehavior<Block>,
    mut inner: P,
) -> impl Parser<&'a str, Output = Pieces<()>, Error = nom::error::Error<&'a str>>
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
        conditional(behavior.clone(), || (), inner1).parse(input)
    }
}

pub(crate) fn conditional_block<'a, P>(
    behavior: crate::parser::config::ElementBehavior<Block>,
    mut inner: P,
) -> impl Parser<&'a str, Output = Pieces<Block>, Error = nom::error::Error<&'a str>>
where
    P: Parser<&'a str, Output = Block, Error = nom::error::Error<&'a str>>,
{
    move |input: &'a str| {
        let inner1 = |s: &'a str| inner.parse(s);
        conditional(behavior.clone(), || Block::Empty, inner1).parse(input)
    }
}

pub(crate) fn conditional_inline<'a, P>(
    behavior: crate::parser::config::ElementBehavior<Inline>,
    mut inner: P,
) -> impl Parser<&'a str, Output = Pieces<Inline>, Error = nom::error::Error<&'a str>>
where
    P: Parser<&'a str, Output = Inline, Error = nom::error::Error<&'a str>>,
{
    move |input: &'a str| {
        let inner1 = |s: &'a str| inner.parse(s);
        conditional(behavior.clone(), || Inline::Empty, inner1).parse(input)
    }
}

pub(crate) fn conditional_block_vec<'a, P>(
    behavior: crate::parser::config::ElementBehavior<Block>,
    mut inner: P,
) -> impl Parser<&'a str, Output = Pieces<Block>, Error = nom::error::Error<&'a str>>
where
    P: Parser<&'a str, Output = Vec<Block>, Error = nom::error::Error<&'a str>>,
{
    move |input: &'a str| {
        let mut inner1 = |s: &'a str| inner.parse(s);
        match &behavior {
            crate::parser::config::ElementBehavior::Ignore => fail().parse(input),
            crate::parser::config::ElementBehavior::Skip => {
                let (remaining, _) = inner1(input)?;
                Ok((remaining, Pieces::One(Block::Empty)))
            }
            // Map and FlatMap do not apply to a parser that yields several blocks.
            crate::parser::config::ElementBehavior::Parse
            | crate::parser::config::ElementBehavior::Map(_)
            | crate::parser::config::ElementBehavior::FlatMap(_) => {
                let (remaining, blocks) = inner1(input)?;
                Ok((remaining, Pieces::Many(blocks)))
            }
        }
    }
}
