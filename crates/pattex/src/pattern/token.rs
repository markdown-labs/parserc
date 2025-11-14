use parserc::syntax::{Char, keyword};

/// backslash token `\`
pub type BackSlash<I> = Char<I, '\\'>;
/// caret token `^`
pub type Caret<I> = Char<I, '^'>;
/// brace start token `{`
pub type BraceStart<I> = Char<I, '{'>;
/// brace end token `}`
pub type BraceEnd<I> = Char<I, '}'>;
/// bracket start token `[`
pub type BracketStart<I> = Char<I, '['>;
/// bracket end token `]`
pub type BracketEnd<I> = Char<I, ']'>;
/// parenthesis start token `(`
pub type ParenStart<I> = Char<I, '('>;
/// parenthesis end token `)`
pub type ParenEnd<I> = Char<I, ')'>;
/// or token `|`
pub type Or<I> = Char<I, '|'>;
/// question token `?`
pub type Question<I> = Char<I, '?'>;
/// dot token `.`
pub type Dot<I> = Char<I, '.'>;
/// plus token `+`
pub type Plus<I> = Char<I, '+'>;
/// minus token `-`
pub type Minus<I> = Char<I, '-'>;
/// star token `*`
pub type Star<I> = Char<I, '*'>;
/// dollar token `$`
pub type Dollar<I> = Char<I, '$'>;

keyword!(BracketStartQeustionColon, "(?:");
keyword!(BracketStartQeustionEq, "(?=");
keyword!(BracketStartQeustionNot, "(?!");
keyword!(BracketStartQeustionLtEq, "(?<=");
keyword!(BracketStartQeustionLtNot, "(?<!");
