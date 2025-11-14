use parserc::syntax::{Char, Syntax};

use crate::{
    input::PatternInput,
    pattern::{
        BackSlash, BraceStart, BracketStart, Caret, Dollar, Dot, Minus, Or, ParenStart, Plus,
        Question, Star,
    },
};

/// Escape token sequence.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Escape<I>
where
    I: PatternInput,
{
    /// `\\`
    BackSlash(BackSlash<I>, BackSlash<I>),
    /// `\^`
    Caret(BackSlash<I>, Caret<I>),
    /// `\*`
    Star(BackSlash<I>, Star<I>),
    /// `\$`
    Dollar(BackSlash<I>, Dollar<I>),
    /// `\?`
    Question(BackSlash<I>, Question<I>),
    /// `\+`
    Plus(BackSlash<I>, Plus<I>),
    /// `\-`
    Minus(BackSlash<I>, Minus<I>),
    /// `\.`
    Dot(BackSlash<I>, Dot<I>),
    /// `\|`
    Or(BackSlash<I>, Or<I>),
    /// `\{`
    BraceStart(BackSlash<I>, BraceStart<I>),
    /// `\[`
    BracketStart(BackSlash<I>, BracketStart<I>),
    /// `\(`
    ParenStart(BackSlash<I>, ParenStart<I>),
    ///  \b
    Boundery(BackSlash<I>, Char<I, 'b'>),
    ///  \B
    NonBoundery(BackSlash<I>, Char<I, 'B'>),
    ///  \d
    Digit(BackSlash<I>, Char<I, 'd'>),
    ///  \D
    NonDigit(BackSlash<I>, Char<I, 'D'>),
    /// \f
    FF(BackSlash<I>, Char<I, 'f'>),
    /// \n
    LF(BackSlash<I>, Char<I, 'n'>),
    /// \r
    CR(BackSlash<I>, Char<I, 'r'>),
    ///  \s
    S(BackSlash<I>, Char<I, 's'>),
    ///  \S
    NonS(BackSlash<I>, Char<I, 'S'>),
    ///  \t
    TF(BackSlash<I>, Char<I, 't'>),
    ///  \v
    VF(BackSlash<I>, Char<I, 'v'>),
    ///  \w
    Word(BackSlash<I>, Char<I, 'w'>),
    ///  \W
    NonWord(BackSlash<I>, Char<I, 'W'>),
}
