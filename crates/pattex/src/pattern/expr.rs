use parserc::syntax::{Delimiter, Syntax};

use crate::input::PatternInput;
use crate::pattern::{
    Class, Escape, ParenEnd, ParenStart, Plus, Question, Repeat, Star, is_token_char,
};

/// Pattern of a sequence of characters.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(token = |c:char| { c == '-' || !is_token_char(c) })]
pub struct PatternChars<I>(pub I)
where
    I: PatternInput;

/// A non-root pattern sequence.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Pattern<I>
where
    I: PatternInput,
{
    /// A sub-pattern of a sequence of characters.
    Chars(PatternChars<I>),
    /// A escape sub-pattern.
    Escap(Escape<I>),
    /// A capture of sub-pattern sequence.
    Capture(Delimiter<ParenStart<I>, ParenEnd<I>, Vec<Pattern<I>>>),
    /// A repeat sub-pattern.
    Repeat(Repeat<I>),
    /// A start sub-pattern.
    Star(Star<I>),
    /// A question sub-pattern.
    Question(Question<I>),
    /// A plus sub-pattern.
    Plus(Plus<I>),
    /// A character class sub-pattern.
    Class(Class<I>),
}

#[cfg(test)]
mod tests {
    use parserc::syntax::{Char, Delimiter, InputSyntaxExt};

    use crate::{
        input::TokenStream,
        pattern::{
            BackSlash, BracketEnd, BracketStart, Caret, Class, ClassChars, Digits, Escape,
            ParenEnd, ParenStart, Pattern, PatternChars, Plus, Question, Repeat, Star,
        },
    };

    #[test]
    fn capture() {
        assert_eq!(
            TokenStream::from("(abc)").parse(),
            Ok(Pattern::Capture(Delimiter {
                start: ParenStart(TokenStream::from("(")),
                end: ParenEnd(TokenStream::from((4, ")"))),
                body: vec![Pattern::Chars(PatternChars(TokenStream::from((1, "abc"))))]
            }))
        );
    }

    #[test]
    fn class() {
        assert_eq!(
            TokenStream::from(r"[^\f\thello0-9]*").parse(),
            Ok(vec![
                Pattern::Class(Class(Delimiter {
                    start: BracketStart(TokenStream::from("[")),
                    end: BracketEnd(TokenStream::from((14, "]"))),
                    body: (
                        Some(Caret(TokenStream::from((1, "^")))),
                        vec![
                            ClassChars::Escape(Escape::FF(
                                BackSlash(TokenStream::from((2, r"\"))),
                                Char(TokenStream::from((3, "f")))
                            )),
                            ClassChars::Escape(Escape::TF(
                                BackSlash(TokenStream::from((4, r"\"))),
                                Char(TokenStream::from((5, "t")))
                            )),
                            ClassChars::Sequnce(TokenStream::from((6, "hello"))),
                            ClassChars::Range {
                                from: '0',
                                to: '9',
                                input: TokenStream::from((11, "0-9"))
                            }
                        ]
                    )
                })),
                Pattern::Star(Star(TokenStream::from((15, "*"))))
            ])
        )
    }

    #[test]
    fn repeat() {
        assert_eq!(
            TokenStream::from("abc{2}").parse(),
            Ok(vec![
                Pattern::Chars(PatternChars(TokenStream::from("abc"))),
                Pattern::Repeat(Repeat::Repeat {
                    n: Digits {
                        value: 2,
                        input: TokenStream::from((4, "2"))
                    },
                    input: TokenStream::from((3, "{2}"))
                })
            ])
        );

        assert_eq!(
            TokenStream::from("abc*").parse(),
            Ok(vec![
                Pattern::Chars(PatternChars(TokenStream::from("abc"))),
                Pattern::Star(Star(TokenStream::from((3, "*"))))
            ])
        );

        assert_eq!(
            TokenStream::from("abc?").parse(),
            Ok(vec![
                Pattern::Chars(PatternChars(TokenStream::from("abc"))),
                Pattern::Question(Question(TokenStream::from((3, "?"))))
            ])
        );

        assert_eq!(
            TokenStream::from("abc+").parse(),
            Ok(vec![
                Pattern::Chars(PatternChars(TokenStream::from("abc"))),
                Pattern::Plus(Plus(TokenStream::from((3, "+"))))
            ])
        );
    }
}
