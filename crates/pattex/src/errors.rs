//! Error types for regex parsing.

use parserc::{ControlFlow, Kind, ParseError, Span};

/// Kind of parsing `regular expressions` error.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum PatternKind {
    #[error("whitespaces")]
    S,
    #[error("digits")]
    Digits,
    #[error("repeat")]
    Repeat,
    #[error("repeat range")]
    RepeatRange,
    #[error("escape")]
    Escape,
    #[error("hexidecimal escape")]
    HexEscape,
    #[error("Unicode escape")]
    UnicodeEscape,
}

impl PatternKind {
    /// Map underlying error into `PatternKind`.
    pub fn map(self) -> impl FnOnce(RegexError) -> RegexError {
        |err: RegexError| RegexError::Pattern(self, err.control_flow(), err.span())
    }

    /// Map underlying error into `PatternKind` fatal error.
    pub fn map_fatal(self) -> impl FnOnce(RegexError) -> RegexError {
        |err: RegexError| RegexError::Pattern(self, ControlFlow::Fatal, err.span())
    }
}

/// Error type returns by `regular expressions` parser.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum RegexError {
    /// Unhandle error kind.
    #[error("{0:?}")]
    Other(#[from] Kind),
    /// Identified parsing errors
    #[error("failed to parsing `{0:?}`, {1:?}, {2:?}")]
    Pattern(PatternKind, ControlFlow, Span),
}

impl ParseError for RegexError {
    fn span(&self) -> Span {
        match self {
            RegexError::Other(kind) => kind.span(),
            RegexError::Pattern(_, _, span) => span.clone(),
        }
    }

    fn control_flow(&self) -> ControlFlow {
        match self {
            RegexError::Other(kind) => kind.control_flow(),
            RegexError::Pattern(_, control_flow, _) => *control_flow,
        }
    }

    fn into_fatal(self) -> Self {
        match self {
            RegexError::Other(kind) => RegexError::Other(kind.into_fatal()),
            RegexError::Pattern(kind, _, span) => {
                RegexError::Pattern(kind, ControlFlow::Fatal, span)
            }
        }
    }
}
