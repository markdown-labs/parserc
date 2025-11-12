use parserc::{ControlFlow, Parser, syntax::Syntax, take_while_range_from};

use crate::{
    errors::{CompileError, RegexError},
    input::PatternInput,
};

/// A digit sequence
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Digits<I>
where
    I: PatternInput,
{
    pub value: u64,
    pub input: I,
}

impl<I> Syntax<I> for Digits<I>
where
    I: PatternInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let content = take_while_range_from(1, |c: char| c.is_ascii_digit())
            .parse(input)
            .map_err(CompileError::Digits.map())?;

        let value = content.as_str().parse().map_err(|_| {
            RegexError::Compile(CompileError::Digits, ControlFlow::Fatal, content.to_span())
        })?;

        Ok(Self {
            value,
            input: content,
        })
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        self.input.to_span()
    }
}
