//! Abstract sytax tree support.

use std::{fmt::Debug, marker::PhantomData};

use crate::{
    errors::{ParseError, Result},
    input::Input,
    lang::LangInput,
    parser::Parser,
};

/// An extension trait to help syntax struct parsing.
pub trait InputSyntaxExt: Input {
    /// Parse a specific `Syntax` type.
    #[inline]
    fn parse<S, E>(self) -> crate::errors::Result<S, Self, E>
    where
        Self: Sized,
        S: Syntax<Self, E>,
        E: ParseError<Self>,
    {
        S::parse(self)
    }
}

/// A syntax tree struct/enum should implment this trait
pub trait Syntax<I, E>: Sized
where
    I: Input,
    E: ParseError<I>,
{
    /// Parse input data and construct a new `Syntax` instance.
    fn parse(input: I) -> Result<Self, I, E>;

    /// Create a new `Parser` from this type.
    fn into_parser() -> impl Parser<I, Output = Self, Error = E> {
        SyntaxParser(Default::default(), Default::default(), Default::default())
    }
}

struct SyntaxParser<S, E, T>(PhantomData<S>, PhantomData<E>, PhantomData<T>);

impl<I, E, T> Parser<I> for SyntaxParser<I, E, T>
where
    I: Input,
    E: ParseError<I>,
    T: Syntax<I, E>,
{
    type Error = E;

    type Output = T;

    fn parse(self, input: I) -> Result<Self::Output, I, Self::Error> {
        T::parse(input)
    }
}

impl<T, I, E> Syntax<I, E> for PhantomData<T>
where
    I: Input,
    E: ParseError<I>,
{
    fn parse(input: I) -> Result<Self, I, E> {
        Ok((Self::default(), input))
    }
}

impl<T, I, E> Syntax<I, E> for Option<T>
where
    T: Syntax<I, E>,
    I: Input + Clone,
    E: ParseError<I>,
{
    fn parse(input: I) -> Result<Self, I, E> {
        T::into_parser().ok().parse(input)
    }
}

impl<T, I, E> Syntax<I, E> for Box<T>
where
    T: Syntax<I, E>,
    I: Input + Clone,
    E: ParseError<I>,
{
    fn parse(input: I) -> Result<Self, I, E> {
        T::into_parser().boxed().parse(input)
    }
}

impl<T, I, E> Syntax<I, E> for Vec<T>
where
    T: Syntax<I, E>,
    I: Input + Clone,
    E: ParseError<I>,
{
    fn parse(mut input: I) -> Result<Self, I, E> {
        let mut elms = vec![];
        loop {
            let elm;
            (elm, input) = T::into_parser().ok().parse(input)?;

            let Some(elm) = elm else {
                break;
            };

            elms.push(elm);
        }

        Ok((elms, input))
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

impl<I, E, Start, End, Body> Syntax<I, E> for Delimiter<Start, End, Body>
where
    I: Input,
    E: ParseError<I>,
    Start: Syntax<I, E>,
    End: Syntax<I, E>,
    Body: Syntax<I, E>,
{
    fn parse(input: I) -> Result<Self, I, E> {
        let (start, input) = Start::parse(input)?;
        let (body, input) = Body::into_parser().fatal().parse(input)?;
        let (end, input) = End::into_parser().fatal().parse(input)?;

        Ok((Self { start, body, end }, input))
    }
}

/// A punctuated sequence of syntax tree nodes of type T separated by punctuation of type P.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Punctuated<T, P> {
    /// (T,P) pairs
    pub pairs: Vec<(T, P)>,
    /// individual tail `T`
    pub tail: Option<Box<T>>,
}

impl<T, P, I, E> Syntax<I, E> for Punctuated<T, P>
where
    T: Syntax<I, E>,
    P: Syntax<I, E>,
    E: ParseError<I>,
    I: Input + Clone,
{
    fn parse(mut input: I) -> Result<Self, I, E> {
        let mut pairs = vec![];

        loop {
            let t;
            (t, input) = T::into_parser().ok().parse(input)?;

            let Some(t) = t else {
                return Ok((Self { pairs, tail: None }, input));
            };

            let p;
            (p, input) = P::into_parser().ok().parse(input)?;

            let Some(p) = p else {
                return Ok((
                    Self {
                        pairs,
                        tail: Some(Box::new(t)),
                    },
                    input,
                ));
            };

            pairs.push((t, p));
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Or<F, S> {
    First(F),
    Second(S),
}

impl<I, E, F, S> Syntax<I, E> for Or<F, S>
where
    I: LangInput,
    E: ParseError<I>,
    F: Syntax<I, E>,
    S: Syntax<I, E>,
{
    fn parse(input: I) -> Result<Self, I, E> {
        let (Some(first), input) = F::into_parser().ok().parse(input.clone())? else {
            let (s, input) = S::parse(input)?;

            return Ok((Self::Second(s), input));
        };

        Ok((Self::First(first), input))
    }
}

/// Use the parsed prefix to parse the syntax tree.
pub trait PartialSyntax<I, E, P>: Sized
where
    I: Input,
    E: ParseError<I>,
{
    ///  Use the parsed prefix to parse the syntax tree.
    fn parse_with_prefix(prefix: P, input: I) -> Result<Self, I, E>;

    /// Create a new `Parser` with parsed prefix subtree.
    fn into_parser_with_prefix(prefix: P) -> impl Parser<I, Output = Self, Error = E> {
        PartialSyntaxParser(
            prefix,
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }
}

struct PartialSyntaxParser<S, E, P, T>(P, PhantomData<S>, PhantomData<E>, PhantomData<T>);

impl<I, E, P, T> Parser<I> for PartialSyntaxParser<I, E, P, T>
where
    E: ParseError<I>,
    I: Input,
    T: PartialSyntax<I, E, P>,
{
    type Error = E;

    type Output = T;

    fn parse(self, input: I) -> Result<Self::Output, I, Self::Error> {
        T::parse_with_prefix(self.0, input)
    }
}

// implement Syntax for tuple (T1,T2,...) where T1: Syntax, T2: Syntax, ...
parserc_derive::derive_tuple_syntax!(16);

#[cfg(test)]
mod tests {
    use crate::{errors::ParseError, input::Input, syntax::Syntax};

    #[allow(unused)]
    struct Mock;

    impl<I, E> Syntax<I, E> for Mock
    where
        I: Input,
        E: ParseError<I>,
    {
        fn parse(input: I) -> crate::errors::Result<Self, I, E> {
            Ok((Mock, input))
        }
    }
}
