//! Abstract sytax tree support.

use std::{fmt::Debug, marker::PhantomData};

use crate::{ControlFlow, Kind, Span};
use crate::{input::Input, parser::Parser};

/// An extension trait to help syntax struct parsing.
pub trait InputSyntaxExt: Input {
    /// Parse a specific `Syntax` type.
    #[inline]
    fn parse<S>(&mut self) -> Result<S, Self::Error>
    where
        Self: Sized,
        S: Syntax<Self>,
    {
        S::parse(self)
    }
}

impl<I> InputSyntaxExt for I where I: Input {}

/// A syntax tree struct/enum should implment this trait
pub trait Syntax<I>: Sized
where
    I: Input,
{
    /// Parse input data and construct a new `Syntax` instance.
    fn parse(input: &mut I) -> Result<Self, I::Error>;

    fn to_span(&self) -> Span;

    /// Create a new `Parser` from this type.
    fn into_parser() -> impl Parser<I, Output = Self> {
        SyntaxParser(Default::default(), Default::default())
    }
}

struct SyntaxParser<S, T>(PhantomData<S>, PhantomData<T>);

impl<I, T> Parser<I> for SyntaxParser<I, T>
where
    I: Input,
    T: Syntax<I>,
{
    type Output = T;

    #[inline]
    fn parse(self, input: &mut I) -> Result<Self::Output, I::Error> {
        T::parse(input)
    }
}

impl<T, I> Syntax<I> for PhantomData<T>
where
    I: Input,
{
    #[inline]
    fn parse(_input: &mut I) -> Result<Self, I::Error> {
        Ok(Self::default())
    }

    #[inline]
    fn to_span(&self) -> Span {
        Span::None
    }
}

impl<T, I> Syntax<I> for Option<T>
where
    T: Syntax<I>,
    I: Input + Clone,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, I::Error> {
        T::into_parser().ok().parse(input)
    }

    #[inline]
    fn to_span(&self) -> Span {
        self.as_ref().map_or(Span::None, |value| value.to_span())
    }
}

impl<T, I> Syntax<I> for Box<T>
where
    T: Syntax<I>,
    I: Input + Clone,
{
    fn parse(input: &mut I) -> Result<Self, I::Error> {
        T::into_parser().boxed().parse(input)
    }

    #[inline]
    fn to_span(&self) -> Span {
        self.as_ref().to_span()
    }
}

impl<T, I> Syntax<I> for Vec<T>
where
    T: Syntax<I>,
    I: Input + Clone,
{
    fn parse(input: &mut I) -> Result<Self, I::Error> {
        let mut elms = vec![];
        loop {
            let elm = T::into_parser().ok().parse(input)?;

            let Some(elm) = elm else {
                break;
            };

            elms.push(elm);
        }

        Ok(elms)
    }

    #[inline]
    fn to_span(&self) -> Span {
        let first = self.first().map_or(Span::None, |v| v.to_span());
        let last = self.last().map_or(Span::None, |v| v.to_span());

        first.union(&last)
    }
}

/// A short syntax for grouping token that surrounds a syntax body.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Delimiter<Start, End, Body> {
    /// Syntax start token.
    pub start: Start,
    /// Syntax end token.
    pub end: End,
    /// Syntax body.
    pub body: Body,
}

impl<I, Start, End, Body> Syntax<I> for Delimiter<Start, End, Body>
where
    I: Input + Clone,
    Start: Syntax<I>,
    End: Syntax<I>,
    Body: Syntax<I>,
{
    fn parse(input: &mut I) -> Result<Self, I::Error> {
        let start = Start::parse(input)?;
        let body = Body::into_parser().fatal().parse(input)?;
        let end = End::into_parser().fatal().parse(input)?;

        Ok(Self { start, body, end })
    }

    #[inline]
    fn to_span(&self) -> Span {
        let start = self.start.to_span();
        let end = self.end.to_span();

        start.union(&end)
    }
}

/// Limits the child `syntax` length.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LimitsTo<T, const N: usize>(pub T);

impl<I, T, const N: usize> Syntax<I> for LimitsTo<T, N>
where
    I: Input,
    T: Syntax<I>,
{
    fn parse(input: &mut I) -> Result<Self, <I as Input>::Error> {
        let start = input.to_span();

        let t = T::parse(input)?;

        let span = t.to_span();

        let len = match span {
            sourcespan::Span::None => 0,
            sourcespan::Span::Range(range) => range.len(),
            sourcespan::Span::RangeTo(range_to) => range_to.end,
            _ => {
                return Err(Kind::LimitsTo(ControlFlow::Recovable, start).into());
            }
        };

        if len > N {
            return Err(Kind::LimitsTo(ControlFlow::Recovable, start).into());
        }

        Ok(Self(t))
    }

    fn to_span(&self) -> Span {
        self.0.to_span()
    }
}

/// Limits the child `syntax` length between `lower` and `higher`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Limits<T, const LOWER: usize, const HIGHER: usize>(pub T);

impl<I, T, const LOWER: usize, const HIGHER: usize> Syntax<I> for Limits<T, LOWER, HIGHER>
where
    I: Input,
    T: Syntax<I>,
{
    fn parse(input: &mut I) -> Result<Self, <I as Input>::Error> {
        let start = input.to_span();

        let t = T::parse(input)?;

        let span = t.to_span();

        let len = match span {
            sourcespan::Span::None => 0,
            sourcespan::Span::Range(range) => range.len(),
            sourcespan::Span::RangeTo(range_to) => range_to.end,
            _ => {
                return Err(Kind::Limits(ControlFlow::Recovable, start).into());
            }
        };

        if len < LOWER || !(len < HIGHER) {
            return Err(Kind::Limits(ControlFlow::Recovable, start).into());
        }

        Ok(Self(t))
    }

    fn to_span(&self) -> Span {
        self.0.to_span()
    }
}

/// Limits the child `syntax` length must equal or greater than `LOWER`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LimitsFrom<T, const LOWER: usize>(pub T);

impl<I, T, const LOWER: usize> Syntax<I> for LimitsFrom<T, LOWER>
where
    I: Input,
    T: Syntax<I>,
{
    fn parse(input: &mut I) -> Result<Self, <I as Input>::Error> {
        let start = input.to_span();

        let t = T::parse(input)?;

        let span = t.to_span();

        let len = match span {
            sourcespan::Span::None => 0,
            sourcespan::Span::Range(range) => range.len(),
            sourcespan::Span::RangeTo(range_to) => range_to.end,
            _ => {
                return Err(Kind::Limits(ControlFlow::Recovable, start).into());
            }
        };

        if len < LOWER {
            return Err(Kind::Limits(ControlFlow::Recovable, start).into());
        }

        Ok(Self(t))
    }

    fn to_span(&self) -> Span {
        self.0.to_span()
    }
}

/// A punctuated sequence of syntax tree nodes of type T separated by punctuation of type P.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Punctuated<T, P> {
    /// (T,P) pairs
    pub pairs: Vec<(T, P)>,
    /// individual tail `T`
    pub tail: Option<Box<T>>,
}

impl<T, P, I> Syntax<I> for Punctuated<T, P>
where
    T: Syntax<I>,
    P: Syntax<I>,
    I: Input + Clone,
{
    fn parse(input: &mut I) -> Result<Self, I::Error> {
        let mut pairs = vec![];

        loop {
            let t = T::into_parser().ok().parse(input)?;

            let Some(t) = t else {
                return Ok(Self { pairs, tail: None });
            };

            let p = P::into_parser().ok().parse(input)?;

            let Some(p) = p else {
                return Ok(Self {
                    pairs,
                    tail: Some(Box::new(t)),
                });
            };

            pairs.push((t, p));
        }
    }

    #[inline]
    fn to_span(&self) -> Span {
        self.pairs.to_span().union(&self.tail.to_span())
    }
}

/// When merging two abstract syntax trees,
/// it first attempts to match the left subtree;
/// if unsuccessful, it proceeds to match the right subtree.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Or<F, S> {
    First(F),
    Second(S),
}

impl<I, F, S> Syntax<I> for Or<F, S>
where
    I: Input + Clone,
    F: Syntax<I>,
    S: Syntax<I>,
{
    fn parse(input: &mut I) -> Result<Self, I::Error> {
        let Some(first) = F::into_parser().ok().parse(input)? else {
            let s = S::parse(input)?;

            return Ok(Self::Second(s));
        };

        Ok(Self::First(first))
    }

    #[inline]
    fn to_span(&self) -> Span {
        match self {
            Or::First(v) => v.to_span(),
            Or::Second(v) => v.to_span(),
        }
    }
}

/// Use the parsed prefix to parse the syntax tree.
pub trait PartialSyntax<I, P>: Sized
where
    I: Input,
{
    ///  Use the parsed prefix to parse the syntax tree.
    fn parse_with_prefix(prefix: P, input: &mut I) -> Result<Self, I::Error>;

    /// Create a new `Parser` with parsed prefix subtree.
    fn into_parser_with_prefix(prefix: P) -> impl Parser<I, Output = Self> {
        PartialSyntaxParser(prefix, Default::default(), Default::default())
    }
}

struct PartialSyntaxParser<S, P, T>(P, PhantomData<S>, PhantomData<T>);

impl<I, P, T> Parser<I> for PartialSyntaxParser<I, P, T>
where
    I: Input,
    T: PartialSyntax<I, P>,
{
    type Output = T;

    fn parse(self, input: &mut I) -> Result<Self::Output, I::Error> {
        T::parse_with_prefix(self.0, input)
    }
}

// implement Syntax for tuple (T1,T2,...) where T1: Syntax, T2: Syntax, ...
parserc_derive::derive_tuple_syntax!(16);

pub use parserc_derive::Syntax;
pub use parserc_derive::keyword;
pub use parserc_derive::token;

#[cfg(test)]
mod tests {
    use crate::{input::Input, syntax::Syntax};

    #[allow(unused)]
    struct Mock;

    impl<I> Syntax<I> for Mock
    where
        I: Input,
    {
        fn parse(_input: &mut I) -> Result<Self, I::Error> {
            Ok(Mock)
        }

        fn to_span(&self) -> crate::Span {
            todo!()
        }
    }
}
