use parserc::{ControlFlow, syntax::Syntax};

use crate::{
    errors::{PatternKind, RegexError},
    input::PatternInput,
    pattern::{Escape, Repeat},
};

/// Token types of regular expression.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Token<I>
where
    I: PatternInput,
{
    /// [`Repeat`] token.
    Repeat(Repeat<I>),
    /// escape sequence.
    Escape(Escape<I>),
    /// |
    Or(I),
    /// ^
    Caret(I),
    /// $
    Dollar(I),
    /// *
    Star(I),
    /// +
    Plus(I),
    /// ?
    Question(I),
    /// [
    BracketStart(I),
    /// [:
    BracketStartColon(I),
    /// [.
    BracketStartDot(I),
    /// [=
    BracketStartEq(I),
    /// ]
    BracketEnd(I),
    /// :]
    ColonBracketEnd(I),
    /// .]
    DotBracketEnd(I),
    /// =]
    EqBracketEnd(I),
    /// (
    ParenStart(I),
    /// )
    ParenEnd(I),
    /// .
    Dot(I),
    /// ?:
    QuestionColon(I),
    /// ?=
    QuestionEq(I),
    /// ?!
    QuestionNot(I),
    /// ?<=
    QuestionLtEq(I),
    /// ?<!
    QuestionLtNot(I),
}

impl<I> Syntax<I> for Token<I>
where
    I: PatternInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let mut iter = input.iter();

        let Some(next) = iter.next() else {
            // if next char is `None`.
            return Err(RegexError::Pattern(
                PatternKind::Token,
                ControlFlow::Recovable,
                input.to_span_at(1),
            ));
        };

        // check next char.
        match next {
            '\\' => Escape::parse(input).map(|escape| Token::Escape(escape)),
            '|' => Ok(Self::Or(input.split_to(1))),
            '^' => Ok(Self::Caret(input.split_to(1))),
            '$' => Ok(Self::Dollar(input.split_to(1))),
            '*' => Ok(Self::Star(input.split_to(1))),
            '+' => Ok(Self::Plus(input.split_to(1))),
            '?' => match iter.next() {
                Some(':') => Ok(Self::QuestionColon(input.split_to(2))),
                Some('=') => Ok(Self::QuestionEq(input.split_to(2))),
                Some('!') => Ok(Self::QuestionNot(input.split_to(2))),
                Some('<') => match iter.next() {
                    Some('=') => Ok(Self::QuestionLtEq(input.split_to(3))),
                    Some('!') => Ok(Self::QuestionLtNot(input.split_to(3))),
                    _ => Err(RegexError::Pattern(
                        PatternKind::Token,
                        ControlFlow::Recovable,
                        input.to_span_at(2),
                    )),
                },
                _ => Ok(Self::Question(input.split_to(1))),
            },
            '{' => Repeat::parse(input).map(|repeat| Self::Repeat(repeat)),
            '[' => match iter.next() {
                Some(':') => Ok(Self::BracketStartColon(input.split_to(2))),
                Some('.') => Ok(Self::BracketStartDot(input.split_to(2))),
                Some('=') => Ok(Self::BracketStartEq(input.split_to(2))),
                _ => Ok(Self::BracketStart(input.split_to(1))),
            },
            ']' => Ok(Self::BracketEnd(input.split_to(1))),
            ':' => {
                if let Some(']') = iter.next() {
                    Ok(Self::ColonBracketEnd(input.split_to(2)))
                } else {
                    Err(RegexError::Pattern(
                        PatternKind::Token,
                        ControlFlow::Recovable,
                        input.to_span_at(1),
                    ))
                }
            }
            '.' => {
                if let Some(']') = iter.next() {
                    Ok(Self::DotBracketEnd(input.split_to(2)))
                } else {
                    Ok(Self::Dot(input.split_to(1)))
                }
            }
            '=' => {
                if let Some(']') = iter.next() {
                    Ok(Self::EqBracketEnd(input.split_to(2)))
                } else {
                    Err(RegexError::Pattern(
                        PatternKind::Token,
                        ControlFlow::Recovable,
                        input.to_span_at(1),
                    ))
                }
            }
            '(' => Ok(Self::ParenStart(input.split_to(1))),
            ')' => Ok(Self::ParenEnd(input.split_to(1))),
            _ => {
                return Err(RegexError::Pattern(
                    PatternKind::Token,
                    ControlFlow::Recovable,
                    input.to_span_at(1),
                ));
            }
        }
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        match self {
            Token::Or(input) => input.to_span(),
            Token::Caret(input) => input.to_span(),
            Token::Dollar(input) => input.to_span(),
            Token::Star(input) => input.to_span(),
            Token::Plus(input) => input.to_span(),
            Token::Question(input) => input.to_span(),
            Token::Repeat(input) => input.to_span(),
            Token::BracketStart(input) => input.to_span(),
            Token::BracketStartColon(input) => input.to_span(),
            Token::BracketStartDot(input) => input.to_span(),
            Token::BracketStartEq(input) => input.to_span(),
            Token::BracketEnd(input) => input.to_span(),
            Token::ColonBracketEnd(input) => input.to_span(),
            Token::DotBracketEnd(input) => input.to_span(),
            Token::EqBracketEnd(input) => input.to_span(),
            Token::ParenStart(input) => input.to_span(),
            Token::ParenEnd(input) => input.to_span(),
            Token::Dot(input) => input.to_span(),
            Token::QuestionColon(input) => input.to_span(),
            Token::QuestionEq(input) => input.to_span(),
            Token::QuestionNot(input) => input.to_span(),
            Token::QuestionLtEq(input) => input.to_span(),
            Token::QuestionLtNot(input) => input.to_span(),
            Token::Escape(escape) => escape.to_span(),
        }
    }
}

#[cfg(test)]
mod tests {
    use parserc::syntax::InputSyntaxExt;

    use crate::{
        input::TokenStream,
        pattern::{Digits, Escape, Repeat, Token},
    };

    #[test]
    fn test_tokens() {
        macro_rules! make_test {
            ($ty:ident,$input:literal) => {
                (
                    TokenStream::from($input),
                    Token::$ty(TokenStream::from($input)),
                )
            };
        }

        let tokens = [
            make_test!(Or, "|"),
            make_test!(Caret, "^"),
            make_test!(Dollar, "$"),
            make_test!(Star, "*"),
            make_test!(Plus, "+"),
            make_test!(Question, "?"),
            make_test!(BracketStart, "["),
            make_test!(BracketStartColon, "[:"),
            make_test!(BracketStartDot, "[."),
            make_test!(BracketStartEq, "[="),
            make_test!(BracketEnd, "]"),
            make_test!(ColonBracketEnd, ":]"),
            make_test!(DotBracketEnd, ".]"),
            make_test!(EqBracketEnd, "=]"),
            make_test!(ParenStart, "("),
            make_test!(ParenEnd, ")"),
            make_test!(Dot, "."),
            make_test!(QuestionColon, "?:"),
            make_test!(QuestionEq, "?="),
            make_test!(QuestionNot, "?!"),
            make_test!(QuestionLtEq, "?<="),
            make_test!(QuestionLtNot, "?<!"),
        ];

        for (mut input, token) in tokens {
            assert_eq!(input.parse(), Ok(token));
        }

        assert_eq!(
            TokenStream::from(r"\12").parse(),
            Ok(Token::Escape(Escape::BackReferences(TokenStream::from(
                r"\12"
            ))))
        );

        assert_eq!(
            TokenStream::from("{10,40}100").parse(),
            Ok(Token::Repeat(Repeat::Range {
                n: Digits {
                    value: 10,
                    input: TokenStream::from((1, "10"))
                },
                m: Digits {
                    value: 40,
                    input: TokenStream::from((4, "40"))
                },
                input: TokenStream::from("{10,40}")
            }))
        );
    }
}
