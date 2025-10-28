//! Parse combinator framework for `rust` language.
#![cfg_attr(docsrs, feature(doc_cfg))]

mod input;
pub use input::*;

mod errors;
pub use errors::*;

mod span;
pub use span::*;

mod parser;
pub use parser::*;

mod lexer;
pub use lexer::*;

#[cfg(feature = "lang")]
#[cfg_attr(docsrs, doc(cfg(feature = "lang")))]
pub mod lang;

#[cfg(feature = "syntax")]
#[cfg_attr(docsrs, doc(cfg(feature = "syntax")))]
pub mod syntax;
