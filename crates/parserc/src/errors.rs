use crate::Span;

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
#[derive(thiserror::Error, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Kind {
    #[error("Error from `next` combinator")]
    Next(Span),
    #[error("Error from `next_if` combinator")]
    NextIf(Span),
    #[error("Error from `keyword` combinator")]
    Keyword(Span),
}

/// A error type returns by parser combinators.
pub trait ParseError: From<(ControlFlow, Kind)> {
    fn control_flow(&self) -> ControlFlow;

    /// Ensure this error is an fatal error.
    fn into_fatal(self) -> Self;
}
