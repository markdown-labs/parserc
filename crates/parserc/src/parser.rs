//! Traits for parser combinators.

use crate::{
    errors::{ControlFlow, ParseError, Result},
    input::Input,
};

/// A parsing combinator should implement this trait.
pub trait Parser<I>
where
    I: Input,
{
    type Output;
    /// error type returns by this `Parser`.
    type Error: ParseError<I>;

    /// Consumes itself and parses the input stream to generate the `output` product.
    fn parse(self, input: I) -> Result<Self::Output, I, Self::Error>;

    /// Creates a new parser that converts `non-fatal` error into `None` value.
    #[inline]
    fn ok(self) -> impl Parser<I, Output = Option<Self::Output>, Error = Self::Error>
    where
        I: Clone,
        Self: Sized,
    {
        IsOk(self)
    }

    /// On success, use func `F` to convert origin output to type `O`
    #[inline]
    fn map<F, O>(self, f: F) -> impl Parser<I, Output = O, Error = Self::Error>
    where
        F: FnOnce(Self::Output) -> O,
        Self: Sized,
    {
        Map(self, f)
    }

    /// Creates a parser that convert all `non-fatal` error into [`fatal`](ControlFlow::Fatal) error.
    #[inline]
    fn fatal(self) -> impl Parser<I, Output = Self::Output, Error = Self::Error>
    where
        Self: Sized,
    {
        Fatal(self)
    }

    /// Map output into `Box<Self::Output>`, this func is short for code `Parser::map(|v|Box::new(v))`
    #[inline]
    fn boxed(self) -> impl Parser<I, Output = Box<Self::Output>, Error = Self::Error>
    where
        Self: Sized,
    {
        self.map(|v| Box::new(v))
    }

    /// Executre another `Parser` if this one returns a `non-fatal` error.
    #[inline]
    fn or<R>(self, parser: R) -> impl Parser<I, Output = Self::Output, Error = Self::Error>
    where
        I: Clone,
        R: Parser<I, Output = Self::Output, Error = Self::Error>,
        Self: Sized,
    {
        Or(self, parser)
    }
}

/// Implement [`Parser`] for all `FnOnce(I) -> Result<O, I, E>`
impl<O, I, E, F> Parser<I> for F
where
    I: Input,
    F: FnOnce(I) -> Result<O, I, E>,
    E: ParseError<I>,
{
    type Output = O;
    type Error = E;

    #[inline]
    fn parse(self, input: I) -> Result<Self::Output, I, Self::Error> {
        self(input)
    }
}

struct IsOk<P>(P);

impl<P, I> Parser<I> for IsOk<P>
where
    I: Input + Clone,
    P: Parser<I>,
{
    type Output = Option<P::Output>;

    type Error = P::Error;

    #[inline]
    fn parse(self, input: I) -> Result<Self::Output, I, Self::Error> {
        // for retrospective analysis, we clone the input stream.
        match self.0.parse(input.clone()) {
            Ok((t, input)) => Ok((Some(t), input)),
            Err(err) if err.control_flow() == ControlFlow::Fatal => Err(err),
            Err(_) => Ok((None, input)),
        }
    }
}

struct Map<P, F>(P, F);

impl<P, I, F, O> Parser<I> for Map<P, F>
where
    I: Input,
    P: Parser<I>,
    F: FnOnce(P::Output) -> O,
{
    type Output = O;
    type Error = P::Error;

    #[inline]
    fn parse(self, input: I) -> Result<Self::Output, I, Self::Error> {
        self.0
            .parse(input)
            .map(|(output, input)| ((self.1)(output), input))
    }
}
struct Fatal<P>(P);

impl<P, I> Parser<I> for Fatal<P>
where
    I: Input,
    P: Parser<I>,
{
    type Output = P::Output;

    type Error = P::Error;

    #[inline]
    fn parse(self, input: I) -> Result<Self::Output, I, Self::Error> {
        match self.0.parse(input) {
            Err(err) => Err(err.into_fatal()),
            r => r,
        }
    }
}

struct Or<L, R>(L, R);

impl<L, R, I, O, E> Parser<I> for Or<L, R>
where
    I: Input + Clone,
    E: ParseError<I>,
    L: Parser<I, Output = O, Error = E>,
    R: Parser<I, Output = O, Error = E>,
{
    type Output = O;

    type Error = E;

    #[inline]
    fn parse(self, input: I) -> Result<Self::Output, I, Self::Error> {
        if let (Some(v), input) = self.0.ok().parse(input.clone())? {
            return Ok((v, input));
        }

        self.1.parse(input)
    }
}
