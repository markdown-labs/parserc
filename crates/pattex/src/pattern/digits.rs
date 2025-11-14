use parserc::{Parser, syntax::Syntax, take_while_range};

use crate::{errors::CompileError, input::PatternInput};

/// A non-empty digit character sequence.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Digits<I, const L: usize>(pub I)
where
    I: PatternInput;

impl<I, const L: usize> Syntax<I> for Digits<I, L>
where
    I: PatternInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        take_while_range(L..L, |c: char| c.is_ascii_digit())
            .map(|input| Self(input))
            .parse(input)
            .map_err(CompileError::Digits.map())
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        self.0.to_span()
    }
}

/// Matching two-byte hexadecimal numbers
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HexDigits<I, const L: usize>(pub I)
where
    I: PatternInput;

impl<const L: usize, I> Syntax<I> for HexDigits<I, L>
where
    I: PatternInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        take_while_range(L..L, |c: char| c.is_ascii_hexdigit())
            .map(|input| Self(input))
            .parse(input)
            .map_err(CompileError::Digits.map())
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        self.0.to_span()
    }
}
