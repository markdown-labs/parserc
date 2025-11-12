use std::cmp;

use parserc::{
    ControlFlow, Parser, Span, next,
    syntax::{InputSyntaxExt, Syntax},
    take_while_range_from,
};

use crate::{
    errors::{CompileError, RegexError},
    input::PatternInput,
    pattern::{Escape, token_lookahead},
};

/// Char in character class.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Chars<I>
where
    I: PatternInput,
{
    /// Escape char sequence.
    Escape(Escape<I>),
    /// A sequence pattern chars.
    Sequnce(I),
    /// A range chars belike: `A-Z`,`0-9`
    Range { from: char, to: char, input: I },
}

impl<I> Syntax<I> for Chars<I>
where
    I: PatternInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        if let Some(escape) = Escape::into_parser().ok().parse(input)? {
            return Ok(Self::Escape(escape));
        }

        let mut content = input.clone();

        let sequnce = take_while_range_from(1, |c: char| !token_lookahead(c))
            .parse(&mut content)
            .map_err(CompileError::CharSequence.map())?;

        let mut iter = content.iter();

        if let Some('-') = iter.next() {
            if sequnce.len() == 1 {
                let Some(to) = iter.next() else {
                    return Err(RegexError::Compile(
                        CompileError::CharRange,
                        ControlFlow::Fatal,
                        Span::Range(
                            content.start() - 1..cmp::min(content.end(), content.start() + 1),
                        ),
                    ));
                };

                let from = sequnce.iter().next().unwrap();

                if !(from < to) {
                    return Err(RegexError::Compile(
                        CompileError::CharRange,
                        ControlFlow::Fatal,
                        Span::Range(content.start() - 1..content.start() + 2),
                    ));
                }

                return Ok(Self::Range {
                    from,
                    to,
                    input: input.split_to(3),
                });
            } else {
                return Ok(Self::Sequnce(input.split_to(sequnce.len() - 1)));
            }
        }

        return Ok(Self::Sequnce(sequnce));
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        match self {
            Chars::Escape(escape) => escape.to_span(),
            Chars::Sequnce(input) => input.to_span(),
            Chars::Range {
                from: _,
                to: _,
                input,
            } => input.to_span(),
        }
    }
}

/// Character class.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharClass<I>
where
    I: PatternInput,
{
    /// '['
    pub delimiter_start: I,
    /// negated flag `^`
    pub negated: Option<I>,
    /// Sequence of chars.
    pub chars: Vec<Chars<I>>,
    /// `]`
    pub delimiter_end: I,
}

impl<I> Syntax<I> for CharClass<I>
where
    I: PatternInput,
{
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let delimiter_start = next('[')
            .parse(input)
            .map_err(CompileError::CharClass.map())?;

        let negated = next('^')
            .ok()
            .parse(input)
            .map_err(CompileError::CharClass.map_fatal())?;

        let chars: Vec<Chars<_>> = input.parse().map_err(CompileError::CharClass.map_fatal())?;

        if chars.is_empty() {
            return Err(RegexError::Compile(
                CompileError::CharClass,
                ControlFlow::Fatal,
                Span::Range(delimiter_start.start()..delimiter_start.start()),
            ));
        }

        let delimiter_end = next(']')
            .parse(input)
            .map_err(CompileError::CharClass.map_fatal())?;

        Ok(Self {
            delimiter_start,
            negated,
            chars,
            delimiter_end,
        })
    }

    #[inline]
    fn to_span(&self) -> Span {
        self.delimiter_start.to_span() + self.delimiter_end.to_span()
    }
}

#[cfg(test)]
mod tests {
    use parserc::{ControlFlow, Span, syntax::InputSyntaxExt};

    use crate::{
        errors::{CompileError, RegexError},
        input::TokenStream,
        pattern::{CharClass, Chars, Escape},
    };

    #[test]
    fn test_chars() {
        assert_eq!(
            TokenStream::from("1234").parse(),
            Ok(Chars::Sequnce(TokenStream::from("1234")))
        );

        assert_eq!(
            TokenStream::from("1234-9").parse(),
            Ok(Chars::Sequnce(TokenStream::from("123")))
        );

        assert_eq!(
            TokenStream::from("a-z").parse(),
            Ok(Chars::Range {
                from: 'a',
                to: 'z',
                input: TokenStream::from("a-z")
            })
        );

        assert_eq!(
            TokenStream::from("0-9").parse(),
            Ok(Chars::Range {
                from: '0',
                to: '9',
                input: TokenStream::from("0-9")
            })
        );

        assert_eq!(
            TokenStream::from(r"\123a-z").parse(),
            Ok(Chars::Escape(Escape::BackReferences(TokenStream::from(
                r"\123"
            ))))
        );

        assert_eq!(
            TokenStream::from("z-a").parse::<Chars<_>>(),
            Err(RegexError::Compile(
                CompileError::CharRange,
                ControlFlow::Fatal,
                Span::Range(0..3)
            ))
        );

        assert_eq!(
            TokenStream::from("z-").parse::<Chars<_>>(),
            Err(RegexError::Compile(
                CompileError::CharRange,
                ControlFlow::Fatal,
                Span::Range(0..2)
            ))
        );
    }

    #[test]
    fn test_char_class() {
        assert_eq!(
            TokenStream::from(r"[^\f0-9]").parse(),
            Ok(CharClass {
                delimiter_start: TokenStream::from("["),
                negated: Some(TokenStream::from((1, "^"))),
                chars: vec![
                    Chars::Escape(Escape::FF(TokenStream::from((2, r"\f")))),
                    Chars::Range {
                        from: '0',
                        to: '9',
                        input: TokenStream::from((4, "0-9"))
                    }
                ],
                delimiter_end: TokenStream::from((7, "]"))
            })
        );

        assert_eq!(
            TokenStream::from(r"[\f\f\n]").parse(),
            Ok(CharClass {
                delimiter_start: TokenStream::from("["),
                negated: None,
                chars: vec![
                    Chars::Escape(Escape::FF(TokenStream::from((1, r"\f")))),
                    Chars::Escape(Escape::FF(TokenStream::from((3, r"\f")))),
                    Chars::Escape(Escape::LF(TokenStream::from((5, r"\n")))),
                ],
                delimiter_end: TokenStream::from((7, "]"))
            })
        );
    }
}
