use crate::Span;

/// A variant type to control error handle.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
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
    Next(ControlFlow, Span),
    #[error("Error from `next_if` combinator")]
    NextIf(ControlFlow, Span),
    #[error("Error from `keyword` combinator")]
    Keyword(ControlFlow, Span),
    #[error("Error from parsing syntax `{0}`")]
    Syntax(&'static str, ControlFlow, Span),
}

/// A error type returns by parser combinators.
pub trait ParseError: From<Kind> {
    /// Returns the span of this error indicates to.
    fn span(&self) -> Span;
    /// Returns `ControlFlow` code of this error.
    fn control_flow(&self) -> ControlFlow;

    /// Ensure this error is an fatal error.
    fn into_fatal(self) -> Self;
}

impl ParseError for Kind {
    fn control_flow(&self) -> ControlFlow {
        match self {
            Kind::Next(control_flow, _) => *control_flow,
            Kind::NextIf(control_flow, _) => *control_flow,
            Kind::Keyword(control_flow, _) => *control_flow,
            Kind::Syntax(_, control_flow, _) => *control_flow,
        }
    }

    fn into_fatal(self) -> Self {
        match self {
            Kind::Next(_, span) => Kind::Next(ControlFlow::Fatal, span),
            Kind::NextIf(_, span) => Kind::NextIf(ControlFlow::Fatal, span),
            Kind::Keyword(_, span) => Kind::Keyword(ControlFlow::Fatal, span),
            Kind::Syntax(name, _, span) => Kind::Syntax(name, ControlFlow::Fatal, span),
        }
    }

    fn span(&self) -> Span {
        match self {
            Kind::Next(_, span) => span.clone(),
            Kind::NextIf(_, span) => span.clone(),
            Kind::Keyword(_, span) => span.clone(),
            Kind::Syntax(_, _, span) => span.clone(),
        }
    }
}
