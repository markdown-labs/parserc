//! Parse combinator framework for `rust` language.
#![cfg_attr(docsrs, feature(doc_cfg))]

mod errors;
pub use errors::*;
mod input;
pub use input::*;
mod lexer;
pub use lexer::*;
mod parser;
pub use parser::*;

#[cfg(feature = "lang")]
#[cfg_attr(docsrs, doc(cfg(feature = "lang")))]
pub mod lang;

#[cfg(feature = "syntax")]
#[cfg_attr(docsrs, doc(cfg(feature = "syntax")))]
pub mod syntax;
