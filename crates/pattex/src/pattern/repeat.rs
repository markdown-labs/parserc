use parserc::{ControlFlow, Parser, next, syntax::Syntax};

use crate::{
    errors::{CompileError, RegexError},
    input::PatternInput,
    pattern::Digits,
};

/// A repeat token.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Repeat<I>
where
    I: PatternInput,
{
    Repeat {
        n: Digits<I>,
        input: I,
    },
    From {
        n: Digits<I>,
        input: I,
    },
    Range {
        n: Digits<I>,
        m: Digits<I>,
        input: I,
    },
}

impl<I> Syntax<I> for Repeat<I>
where
    I: PatternInput,
{
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let mut span = input.clone();

        next('{').parse(input).map_err(CompileError::Repeat.map())?;

        let n = Digits::parse(input).map_err(CompileError::Repeat.map_fatal())?;

        if let Some(_) = next(',')
            .ok()
            .parse(input)
            .map_err(CompileError::Repeat.map_fatal())?
        {
            if let Some(m) = Digits::into_parser()
                .ok()
                .parse(input)
                .map_err(CompileError::Repeat.map_fatal())?
            {
                let end = next('}')
                    .parse(input)
                    .map_err(CompileError::Repeat.map_fatal())?;

                if n.value > m.value {
                    return Err(RegexError::Compile(
                        CompileError::Repeat,
                        ControlFlow::Fatal,
                        span.to_span_at(end.end() - span.start()),
                    ));
                }

                return Ok(Self::Range {
                    n,
                    m,
                    input: span.split_to(end.end() - span.start()),
                });
            } else {
                let end = next('}')
                    .parse(input)
                    .map_err(CompileError::Repeat.map_fatal())?;

                return Ok(Self::From {
                    n,
                    input: span.split_to(end.end() - span.start()),
                });
            }
        }

        let end = next('}')
            .parse(input)
            .map_err(CompileError::Repeat.map_fatal())?;

        Ok(Self::Repeat {
            n,
            input: span.split_to(end.end() - span.start()),
        })
    }

    fn to_span(&self) -> parserc::Span {
        match self {
            Repeat::Repeat { n: _, input: span } => span.to_span(),
            Repeat::From { n: _, input: span } => span.to_span(),
            Repeat::Range {
                n: _,
                m: _,
                input: span,
            } => span.to_span(),
        }
    }
}

#[cfg(test)]
mod tests {
    use parserc::{ControlFlow, Span, syntax::InputSyntaxExt};

    use super::*;
    use crate::{errors::RegexError, input::TokenStream};

    #[test]
    fn test_repeat() {
        assert_eq!(
            TokenStream::from("{10}100").parse(),
            Ok(Repeat::Repeat {
                n: Digits {
                    value: 10,
                    input: TokenStream::from((1, "10"))
                },
                input: TokenStream::from("{10}")
            })
        );

        assert_eq!(
            TokenStream::from("{10,}100").parse(),
            Ok(Repeat::From {
                n: Digits {
                    value: 10,
                    input: TokenStream::from((1, "10"))
                },
                input: TokenStream::from("{10,}")
            })
        );

        assert_eq!(
            TokenStream::from("{10,40}100").parse(),
            Ok(Repeat::Range {
                n: Digits {
                    value: 10,
                    input: TokenStream::from((1, "10"))
                },
                m: Digits {
                    value: 40,
                    input: TokenStream::from((4, "40"))
                },
                input: TokenStream::from("{10,40}")
            })
        );

        assert_eq!(
            TokenStream::from("{ 10} ").parse::<Repeat<_>>(),
            Err(RegexError::Compile(
                CompileError::Repeat,
                ControlFlow::Fatal,
                Span::Range(1..1)
            ))
        );

        assert_eq!(
            TokenStream::from("{10 } ").parse::<Repeat<_>>(),
            Err(RegexError::Compile(
                CompileError::Repeat,
                ControlFlow::Fatal,
                Span::Range(3..4)
            ))
        );

        assert_eq!(
            TokenStream::from("{10, } ").parse::<Repeat<_>>(),
            Err(RegexError::Compile(
                CompileError::Repeat,
                ControlFlow::Fatal,
                Span::Range(4..5)
            ))
        );

        assert_eq!(
            TokenStream::from("{10, 20} ").parse::<Repeat<_>>(),
            Err(RegexError::Compile(
                CompileError::Repeat,
                ControlFlow::Fatal,
                Span::Range(4..5)
            ))
        );

        assert_eq!(
            TokenStream::from("{10 ,20} ").parse::<Repeat<_>>(),
            Err(RegexError::Compile(
                CompileError::Repeat,
                ControlFlow::Fatal,
                Span::Range(3..4)
            ))
        );

        assert_eq!(
            TokenStream::from("{10,5} ").parse::<Repeat<_>>(),
            Err(RegexError::Compile(
                CompileError::Repeat,
                ControlFlow::Fatal,
                Span::Range(0..6)
            ))
        );
    }
}
