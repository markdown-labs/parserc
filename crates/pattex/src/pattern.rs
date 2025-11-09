//! Parser for `regular expression`s.

use parserc::{
    ControlFlow, ParseError, Parser, Span, next,
    syntax::{InputSyntaxExt, Syntax},
    take_while, take_while_n_to_m,
};

use crate::{
    errors::{PatternKind, RegexError},
    input::PatternInput,
};

/// A sequence of whitespaces.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct S<I>(pub I)
where
    I: PatternInput;

impl<I> Syntax<I> for S<I>
where
    I: PatternInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        take_while(|c: char| c.is_ascii_whitespace())
            .map(|input| Self(input))
            .parse(input)
    }

    fn to_span(&self) -> parserc::Span {
        self.0.to_span()
    }
}

/// A sequence of digits.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Digits<I>(pub I)
where
    I: PatternInput;

impl<I> Digits<I>
where
    I: PatternInput,
{
    fn as_usize(&self) -> usize {
        self.0.as_str().parse().unwrap()
    }
}

impl<I> Syntax<I> for Digits<I>
where
    I: PatternInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let content = take_while(|c: char| c.is_ascii_digit()).parse(input)?;

        if content.is_empty() {
            return Err(RegexError::Pattern(
                PatternKind::Digits,
                ControlFlow::Recovable,
                Span::Range(input.start()..input.start()),
            ));
        }

        Ok(Self(content))
    }

    fn to_span(&self) -> parserc::Span {
        self.0.to_span()
    }
}

/// A predicate of repeat expression.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Repeat<I>
where
    I: PatternInput,
{
    Star(I),
    Question(I),
    Plus(I),
    N(Digits<I>),
    RangeFrom(Digits<I>),
    Range { n: Digits<I>, m: Digits<I> },
}

impl<I> Syntax<I> for Repeat<I>
where
    I: PatternInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        // for simple pattern: `*`,`?`,`+`
        if let Some(repeat) = next('*')
            .map(|v| Self::Star(v))
            .or(next('+').map(|v| Self::Plus(v)))
            .or(next('?').map(|v| Self::Question(v)))
            .ok()
            .parse(input)?
        {
            return Ok(repeat);
        }

        let Some(_) = next('{').ok().parse(input)? else {
            return Err(RegexError::Pattern(
                PatternKind::Repeat,
                ControlFlow::Recovable,
                Span::Range(input.start()..input.start()),
            ));
        };

        _ = S::parse(input)?;

        let n = Digits::parse(input).map_err(|err| err.into_fatal())?;

        _ = S::parse(input)?;

        let Some(_) = next(',').ok().parse(input)? else {
            _ = next('}').parse(input).map_err(PatternKind::Repeat.map())?;
            return Ok(Self::N(n));
        };

        _ = S::parse(input)?;

        let m: Option<(Digits<_>, S<_>)> = input.parse().map_err(|err| err.into_fatal())?;

        _ = next('}').parse(input).map_err(PatternKind::Repeat.map())?;

        if let Some((m, _)) = m {
            if n.as_usize() > m.as_usize() {
                return Err(RegexError::Pattern(
                    PatternKind::RepeatRange,
                    ControlFlow::Fatal,
                    n.to_span() + m.to_span(),
                ));
            }

            Ok(Self::Range { n, m })
        } else {
            Ok(Self::RangeFrom(n))
        }
    }

    #[inline]
    fn to_span(&self) -> Span {
        match self {
            Repeat::N(digits) => digits.to_span(),
            Repeat::RangeFrom(digits) => digits.to_span(),
            Repeat::Range { n: from, m: to } => from.to_span() + to.to_span(),
            Repeat::Star(input) => input.to_span(),
            Repeat::Question(input) => input.to_span(),
            Repeat::Plus(input) => input.to_span(),
        }
    }
}

/// Identities escape seqence.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Escaped<I>
where
    I: PatternInput,
{
    /// `\b`
    Boundary(I),
    /// `\B`
    NonBoundary(I),
    /// `\d` equals to `[0-9]`
    Digit(I),
    /// `\D` equals to `[^0-9]`
    NonDigit(I),
    /// `\f`
    PF(I),
    /// `\n`
    LF(I),
    /// `\r`
    CR(I),
    /// `\s` equals to `[ \f\n\r\t\v]`
    S(I),
    /// `\S` equals to `[^ \f\n\r\t\v]`
    NonS(I),
    /// `\t`
    T(I),
    /// `\v`
    V(I),
    /// `\w` equals to [A-Za-z0-9_]
    Word(I),
    /// `\W` equals to [^A-Za-z0-9_]
    NonWord(I),
    /// hex escape, `\xnn`
    X { prefix: I, num: I },
    /// `\num`
    BackReference(I),
    /// `\u{xx}`
    Unicode {
        prefix: I,
        delimiter_start: I,
        num: I,
        delimiter_end: I,
    },
}

impl<I> Syntax<I> for Escaped<I>
where
    I: PatternInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let mut prefix = input.clone();

        let Some(_) = next('\\').ok().parse(input)? else {
            return Err(RegexError::Pattern(
                PatternKind::Escape,
                ControlFlow::Recovable,
                Span::Range(input.start()..input.start()),
            ));
        };

        match input.iter().next() {
            Some('b') => {
                *input = prefix.split_off(2);

                Ok(Self::Boundary(prefix))
            }
            Some('B') => {
                *input = prefix.split_off(2);

                Ok(Self::NonBoundary(prefix))
            }
            Some('d') => {
                *input = prefix.split_off(2);

                Ok(Self::Digit(prefix))
            }
            Some('D') => {
                *input = prefix.split_off(2);

                Ok(Self::NonDigit(prefix))
            }
            Some('f') => {
                *input = prefix.split_off(2);

                Ok(Self::PF(prefix))
            }
            Some('r') => {
                *input = prefix.split_off(2);

                Ok(Self::CR(prefix))
            }
            Some('n') => {
                *input = prefix.split_off(2);

                Ok(Self::LF(prefix))
            }
            Some('s') => {
                *input = prefix.split_off(2);

                Ok(Self::S(prefix))
            }
            Some('S') => {
                *input = prefix.split_off(2);

                Ok(Self::NonS(prefix))
            }
            Some('t') => {
                *input = prefix.split_off(2);

                Ok(Self::T(prefix))
            }
            Some('v') => {
                *input = prefix.split_off(2);

                Ok(Self::V(prefix))
            }
            Some('w') => {
                *input = prefix.split_off(2);

                Ok(Self::Word(prefix))
            }
            Some('W') => {
                *input = prefix.split_off(2);

                Ok(Self::NonWord(prefix))
            }
            Some('x') => {
                input.split_to(1);

                let nn = take_while_n_to_m(2..2, |c: char| c.is_ascii_hexdigit())
                    .parse(input)
                    .map_err(PatternKind::HexEscape.map_fatal())?;

                *input = prefix.split_off(2);

                Ok(Self::X { prefix, num: nn })
            }
            Some('u') => {
                input.split_to(1);

                let delimiter_start = next('{')
                    .parse(input)
                    .map_err(PatternKind::UnicodeEscape.map())?;

                _ = S::parse(input)?;

                let num = take_while(|c: char| c.is_ascii_hexdigit()).parse(input)?;

                if num.is_empty() {
                    return Err(RegexError::Pattern(
                        PatternKind::UnicodeEscape,
                        ControlFlow::Fatal,
                        Span::Range(prefix.start()..num.end()),
                    ));
                }

                _ = S::parse(input)?;

                let delimiter_end = next('}')
                    .parse(input)
                    .map_err(PatternKind::UnicodeEscape.map())?;

                Ok(Self::Unicode {
                    prefix: prefix.split_to(2),
                    delimiter_start,
                    num,
                    delimiter_end,
                })
            }
            _ => {
                // check if this is a back reference.
                if let Some(digits) = Digits::into_parser().ok().parse(input)? {
                    return Ok(Self::BackReference(prefix.split_to(1 + digits.0.len())));
                }

                return Err(RegexError::Pattern(
                    PatternKind::Escape,
                    ControlFlow::Recovable,
                    Span::Range(prefix.start()..input.start()),
                ));
            }
        }
    }

    #[inline]
    fn to_span(&self) -> Span {
        match self {
            Escaped::Boundary(input) => input.to_span(),
            Escaped::NonBoundary(input) => input.to_span(),
            Escaped::Digit(input) => input.to_span(),
            Escaped::NonDigit(input) => input.to_span(),
            Escaped::PF(input) => input.to_span(),
            Escaped::LF(input) => input.to_span(),
            Escaped::CR(input) => input.to_span(),
            Escaped::S(input) => input.to_span(),
            Escaped::NonS(input) => input.to_span(),
            Escaped::T(input) => input.to_span(),
            Escaped::V(input) => input.to_span(),
            Escaped::Word(input) => input.to_span(),
            Escaped::NonWord(input) => input.to_span(),
            Escaped::BackReference(input) => input.to_span(),
            Escaped::X { prefix, num } => prefix.to_span() + num.to_span(),
            Escaped::Unicode {
                prefix,
                delimiter_start: _,
                num: _,
                delimiter_end,
            } => prefix.to_span() + delimiter_end.to_span(),
        }
    }
}

#[cfg(test)]
mod tests {
    use parserc::{ControlFlow, Span, syntax::InputSyntaxExt};

    use crate::{
        errors::{PatternKind, RegexError},
        input::TokenStream,
        pattern::{Digits, Escaped, Repeat},
    };

    #[test]
    fn test_digits() {
        assert_eq!(
            TokenStream::from("1234hello").parse(),
            Ok(Digits(TokenStream::from("1234")))
        );

        assert_eq!(
            TokenStream::from("hello123").parse::<Digits<_>>(),
            Err(RegexError::Pattern(
                PatternKind::Digits,
                ControlFlow::Recovable,
                Span::Range(0..0)
            ))
        );
    }

    #[test]
    fn test_repeat() {
        assert_eq!(
            TokenStream::from("?").parse(),
            Ok(Repeat::Question(TokenStream::from("?")))
        );

        assert_eq!(
            TokenStream::from("+").parse(),
            Ok(Repeat::Plus(TokenStream::from("+")))
        );

        assert_eq!(
            TokenStream::from("*").parse(),
            Ok(Repeat::Star(TokenStream::from("*")))
        );

        assert_eq!(
            TokenStream::from("{ 10 }").parse(),
            Ok(Repeat::N(Digits(TokenStream::from((2, "10")))))
        );

        assert_eq!(
            TokenStream::from("{10 ,}").parse(),
            Ok(Repeat::RangeFrom(Digits(TokenStream::from((1, "10")))))
        );

        assert_eq!(
            TokenStream::from("{10, }").parse(),
            Ok(Repeat::RangeFrom(Digits(TokenStream::from((1, "10")))))
        );

        assert_eq!(
            TokenStream::from("{10, 30}").parse(),
            Ok(Repeat::Range {
                n: Digits(TokenStream::from((1, "10"))),
                m: Digits(TokenStream::from((5, "30")))
            })
        );

        assert_eq!(
            TokenStream::from("{30, 10}").parse::<Repeat<_>>(),
            Err(RegexError::Pattern(
                PatternKind::RepeatRange,
                ControlFlow::Fatal,
                Span::Range(1..7)
            ))
        );
    }

    #[test]
    fn test_escape() {
        assert_eq!(
            TokenStream::from(r"\u{00A9}").parse(),
            Ok(Escaped::Unicode {
                prefix: TokenStream::from(r"\u"),
                delimiter_start: TokenStream::from((2, "{")),
                num: TokenStream::from((3, "00A9")),
                delimiter_end: TokenStream::from((7, "}")),
            })
        );

        assert_eq!(
            TokenStream::from(r"\4").parse(),
            Ok(Escaped::BackReference(TokenStream::from(r"\4")))
        );

        assert_eq!(
            TokenStream::from(r"\x04a").parse(),
            Ok(Escaped::X {
                prefix: TokenStream::from(r"\x"),
                num: TokenStream::from((2, "04"))
            })
        );

        assert_eq!(
            TokenStream::from(r"\x4h").parse::<Escaped<_>>(),
            Err(RegexError::Pattern(
                PatternKind::HexEscape,
                ControlFlow::Fatal,
                Span::Range(2..3)
            ))
        );

        assert_eq!(
            TokenStream::from(r"\W+").parse(),
            Ok(Escaped::NonWord(TokenStream::from(r"\W")))
        );

        assert_eq!(
            TokenStream::from(r"\w*").parse(),
            Ok(Escaped::Word(TokenStream::from(r"\w")))
        );

        assert_eq!(
            TokenStream::from(r"\v*").parse(),
            Ok(Escaped::V(TokenStream::from(r"\v")))
        );

        assert_eq!(
            TokenStream::from(r"\t*").parse(),
            Ok(Escaped::T(TokenStream::from(r"\t")))
        );

        assert_eq!(
            TokenStream::from(r"\S").parse(),
            Ok(Escaped::NonS(TokenStream::from(r"\S")))
        );

        assert_eq!(
            TokenStream::from(r"\s").parse(),
            Ok(Escaped::S(TokenStream::from(r"\s")))
        );

        assert_eq!(
            TokenStream::from(r"\n").parse(),
            Ok(Escaped::LF(TokenStream::from(r"\n")))
        );

        assert_eq!(
            TokenStream::from(r"\r").parse(),
            Ok(Escaped::CR(TokenStream::from(r"\r")))
        );

        assert_eq!(
            TokenStream::from(r"\f").parse(),
            Ok(Escaped::PF(TokenStream::from(r"\f")))
        );

        assert_eq!(
            TokenStream::from(r"\D").parse(),
            Ok(Escaped::NonDigit(TokenStream::from(r"\D")))
        );

        assert_eq!(
            TokenStream::from(r"\d").parse(),
            Ok(Escaped::Digit(TokenStream::from(r"\d")))
        );

        assert_eq!(
            TokenStream::from(r"\B").parse(),
            Ok(Escaped::NonBoundary(TokenStream::from(r"\B")))
        );

        assert_eq!(
            TokenStream::from(r"\b").parse(),
            Ok(Escaped::Boundary(TokenStream::from(r"\b")))
        );
    }
}
