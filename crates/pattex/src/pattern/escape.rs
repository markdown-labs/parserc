use parserc::{ControlFlow, Parser, next, syntax::Syntax, take_while_range, take_while_range_from};

use crate::{
    errors::{CompileError, RegexError},
    input::PatternInput,
};

/// A escape token for regular expression.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Escape<I>
where
    I: PatternInput,
{
    /// |
    Or(I),
    /// \
    BackSlash(I),
    /// ^
    Caret(I),
    /// $
    Dollar(I),
    /// *
    Star(I),
    /// +
    Plus(I),
    /// -
    Minus(I),
    /// ?
    Question(I),
    /// {
    BraceStart(I),
    /// [
    BracketStart(I),
    /// (
    ParenStart(I),
    /// )
    ParenEnd(I),
    /// .
    Dot(I),
    /// \n
    BackReferences(I),
    /// \u{nnnn}
    Unicode(I),
    ///  \b
    Boundery(I),
    ///  \B
    NonBoundery(I),
    ///  \d
    Digit(I),
    ///  \D
    NonDigit(I),
    /// \f
    FF(I),
    /// \n
    LF(I),
    /// \r
    CR(I),
    ///  \s
    S(I),
    ///  \S
    NonS(I),
    ///  \t
    TF(I),
    ///  \v
    VF(I),
    ///  \w
    Word(I),
    ///  \W
    NonWord(I),
    /// \xnn
    Hex(I),
}

impl<I> Syntax<I> for Escape<I>
where
    I: PatternInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let mut iter = input.iter();

        let Some('\\') = iter.next() else {
            return Err(RegexError::Compile(
                CompileError::Escape,
                ControlFlow::Recovable,
                input.to_span_at(1),
            ));
        };

        match iter.next() {
            Some('|') => Ok(Self::Or(input.split_to(2))),
            Some('\\') => Ok(Self::BackSlash(input.split_to(2))),
            Some('^') => Ok(Self::Caret(input.split_to(2))),
            Some('$') => Ok(Self::Dollar(input.split_to(2))),
            Some('*') => Ok(Self::Star(input.split_to(2))),
            Some('+') => Ok(Self::Plus(input.split_to(2))),
            Some('-') => Ok(Self::Minus(input.split_to(2))),
            Some('?') => Ok(Self::Question(input.split_to(2))),
            Some('{') => Ok(Self::BraceStart(input.split_to(2))),
            Some('[') => Ok(Self::BracketStart(input.split_to(2))),
            Some('(') => Ok(Self::ParenStart(input.split_to(2))),
            Some(')') => Ok(Self::ParenEnd(input.split_to(2))),
            Some('.') => Ok(Self::Dot(input.split_to(2))),
            Some('b') => Ok(Self::Boundery(input.split_to(2))),
            Some('B') => Ok(Self::NonBoundery(input.split_to(2))),
            Some('d') => Ok(Self::Digit(input.split_to(2))),
            Some('D') => Ok(Self::NonDigit(input.split_to(2))),
            Some('f') => Ok(Self::FF(input.split_to(2))),
            Some('n') => Ok(Self::LF(input.split_to(2))),
            Some('r') => Ok(Self::CR(input.split_to(2))),
            Some('s') => Ok(Self::S(input.split_to(2))),
            Some('S') => Ok(Self::NonS(input.split_to(2))),
            Some('t') => Ok(Self::TF(input.split_to(2))),
            Some('v') => Ok(Self::VF(input.split_to(2))),
            Some('w') => Ok(Self::Word(input.split_to(2))),
            Some('W') => Ok(Self::NonWord(input.split_to(2))),
            Some('x') => {
                take_while_range(2..2, |c: char| c.is_ascii_hexdigit())
                    .parse(&mut input.clone().split_off(2))
                    .map_err(CompileError::EscapeHex.map_fatal())?;

                Ok(Self::Hex(input.split_to(4)))
            }
            Some('u') => {
                let mut content = input.clone().split_off(2);

                next('{')
                    .parse(&mut content)
                    .map_err(CompileError::EscapeUnicode.map_fatal())?;

                let num = take_while_range_from(1, |c: char| c.is_ascii_hexdigit())
                    .parse(&mut content)
                    .map_err(CompileError::EscapeUnicode.map_fatal())?;

                next('}')
                    .parse(&mut content)
                    .map_err(CompileError::EscapeUnicode.map_fatal())?;

                Ok(Self::Unicode(input.split_to(4 + num.len())))
            }
            _ => {
                if let Some(num) = take_while_range_from(1, |c: char| c.is_ascii_digit())
                    .ok()
                    .parse(&mut input.clone().split_off(1))
                    .map_err(CompileError::Escape.map())?
                {
                    Ok(Self::BackReferences(input.split_to(1 + num.len())))
                } else {
                    Err(RegexError::Compile(
                        CompileError::Escape,
                        ControlFlow::Recovable,
                        input.to_span_at(1),
                    ))
                }
            }
        }
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        match self {
            Escape::Or(input) => input.to_span(),
            Escape::BackSlash(input) => input.to_span(),
            Escape::Caret(input) => input.to_span(),
            Escape::Dollar(input) => input.to_span(),
            Escape::Star(input) => input.to_span(),
            Escape::Plus(input) => input.to_span(),
            Escape::Question(input) => input.to_span(),
            Escape::BraceStart(input) => input.to_span(),
            Escape::BracketStart(input) => input.to_span(),
            Escape::ParenStart(input) => input.to_span(),
            Escape::ParenEnd(input) => input.to_span(),
            Escape::Dot(input) => input.to_span(),
            Escape::BackReferences(input) => input.to_span(),
            Escape::Unicode(input) => input.to_span(),
            Escape::Boundery(input) => input.to_span(),
            Escape::NonBoundery(input) => input.to_span(),
            Escape::Digit(input) => input.to_span(),
            Escape::NonDigit(input) => input.to_span(),
            Escape::FF(input) => input.to_span(),
            Escape::LF(input) => input.to_span(),
            Escape::CR(input) => input.to_span(),
            Escape::S(input) => input.to_span(),
            Escape::NonS(input) => input.to_span(),
            Escape::TF(input) => input.to_span(),
            Escape::VF(input) => input.to_span(),
            Escape::Word(input) => input.to_span(),
            Escape::NonWord(input) => input.to_span(),
            Escape::Hex(input) => input.to_span(),
            Escape::Minus(input) => input.to_span(),
        }
    }
}

#[cfg(test)]
mod tests {
    use parserc::syntax::InputSyntaxExt;

    use super::*;
    use crate::input::TokenStream;

    #[test]
    fn test_escape() {
        macro_rules! make_test {
            ($ty:ident,$match:literal,$input:literal) => {
                (
                    TokenStream::from($input),
                    Escape::$ty(TokenStream::from($match)),
                )
            };
        }

        let tests = [
            make_test!(Or, r"\|", r"\|~~~~"),
            make_test!(BackSlash, r"\\", r"\\~~~~"),
            make_test!(Caret, r"\^", r"\^~~~~"),
            make_test!(Dollar, r"\$", r"\$~~~~"),
            make_test!(Star, r"\*", r"\*~~~~"),
            make_test!(Plus, r"\+", r"\+~~~~"),
            make_test!(Question, r"\?", r"\?~~~~"),
            make_test!(BraceStart, r"\{", r"\{~~~~"),
            make_test!(BracketStart, r"\[", r"\[~~~~"),
            make_test!(ParenStart, r"\(", r"\(~~~~"),
            make_test!(ParenEnd, r"\)", r"\)~~~~"),
            make_test!(Dot, r"\.", r"\.~~~~"),
            make_test!(BackReferences, r"\123", r"\123~~~~"),
            make_test!(Unicode, r"\u{00A9}", r"\u{00A9}~~~~"),
            make_test!(Boundery, r"\b", r"\b~~~~"),
            make_test!(NonBoundery, r"\B", r"\B~~~~"),
            make_test!(Digit, r"\d", r"\d~~~~"),
            make_test!(NonDigit, r"\D", r"\D~~~~"),
            make_test!(FF, r"\f", r"\f~~~~"),
            make_test!(LF, r"\n", r"\n~~~~"),
            make_test!(CR, r"\r", r"\r~~~~"),
            make_test!(S, r"\s", r"\s~~~~"),
            make_test!(NonS, r"\S", r"\S~~~~"),
            make_test!(TF, r"\t", r"\t~~~~"),
            make_test!(VF, r"\v", r"\v~~~~"),
            make_test!(Word, r"\w", r"\w~~~~"),
            make_test!(NonWord, r"\W", r"\W~~~~"),
            make_test!(Hex, r"\x0a", r"\x0a~~~~"),
        ];

        for (mut input, token) in tests {
            assert_eq!(input.parse(), Ok(token));
        }
    }
}
