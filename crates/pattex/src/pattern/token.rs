use parserc::syntax::Syntax;

use crate::input::PatternInput;

/// backslash token `\`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = '\\')]
pub struct BackSlash<I>(pub I)
where
    I: PatternInput;

/// caret token `^`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = '^')]
pub struct Caret<I>(pub I)
where
    I: PatternInput;

/// brace start token `{`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = '{')]
pub struct BraceStart<I>(pub I)
where
    I: PatternInput;

/// brace end token `}`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = '}')]
pub struct BraceEnd<I>(pub I)
where
    I: PatternInput;

/// bracket start token `[`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = '[')]
pub struct BracketStart<I>(pub I)
where
    I: PatternInput;

/// bracket end token `]`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = ']')]
pub struct BracketEnd<I>(pub I)
where
    I: PatternInput;

/// parenthesis start token `(`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = '(')]
pub struct ParenStart<I>(pub I)
where
    I: PatternInput;

/// parenthesis end token `)`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = ')')]
pub struct ParenEnd<I>(pub I)
where
    I: PatternInput;

/// or token `|`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = '|')]
pub struct Or<I>(pub I)
where
    I: PatternInput;

/// question token `?`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = '?')]
pub struct Question<I>(pub I)
where
    I: PatternInput;

/// dot token `.`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = '.')]
pub struct Dot<I>(pub I)
where
    I: PatternInput;

/// plus token `+`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = '+')]
pub struct Plus<I>(pub I)
where
    I: PatternInput;

/// minus token `-`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = '-')]
pub struct Minus<I>(pub I)
where
    I: PatternInput;

/// star token `*`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = '*')]
pub struct Star<I>(pub I)
where
    I: PatternInput;

/// dollar token `$`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(char = '$')]
pub struct Dollar<I>(pub I)
where
    I: PatternInput;

/// token `(?:`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(keyword = "(?:")]
pub struct BracketStartQeustionColon<I>(pub I)
where
    I: PatternInput;

/// token `(?=`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(keyword = "(?=")]
pub struct BracketStartQeustionEq<I>(pub I)
where
    I: PatternInput;

/// token `(?!`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(keyword = "(?!")]
pub struct BracketStartQeustionNot<I>(pub I)
where
    I: PatternInput;

/// token `(?<=`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(keyword = "(?<=")]
pub struct BracketStartQeustionLtEq<I>(pub I)
where
    I: PatternInput;

/// token `(?<!`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(keyword = "(?<!")]
pub struct BracketStartQeustionLtNot<I>(pub I)
where
    I: PatternInput;

#[inline]
pub(super) fn is_token_char(c: char) -> bool {
    match c {
        '\\' | '|' | '^' | '$' | '*' | '+' | '-' | '?' | '{' | '[' | ']' | '.' | '=' | '('
        | ')' => true,
        _ => false,
    }
}
