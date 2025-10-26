//! Error types/traits used by `parserc`.

use crate::input::Input;

/// A variant type to control error handle.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ControlFlow {
    /// A fatal error must broke the parsing process.
    Fatal,
    /// A recovable error generally lead to a retrospective parsing process.
    Recovable,
    /// This error means that the parsing process failed because it reached the end of the input stream.
    Incomplete,
}

/// Error kind returns by builtin parser combinators.
#[derive(Debug, thiserror::Error, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Kind {
    #[error("Error from `next` combinator")]
    Next,
    #[error("Error from `next_if` combinator")]
    NextIf,
    #[error("Error from `take_until` combinator")]
    TakeUntil,
    #[error("Error from `keyword` combinator")]
    Keyword,
}

/// A error type returns by parser combinators.
pub trait ParseError<I>: From<(ControlFlow, Kind, I)>
where
    I: Input,
{
    fn control_flow(&self) -> ControlFlow;

    /// Ensure this error is an fatal error.
    fn into_fatal(self) -> Self;
}

/// `Result` type used by `parserc`
pub type Result<T, I, E> = std::result::Result<(T, I), E>;
