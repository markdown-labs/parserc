//! Parse combinator framework for `rust` language.
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod input;

#[cfg(feature = "lang")]
#[cfg_attr(docsrs, doc(cfg(feature = "lang")))]
pub mod lang;

pub mod errors;
pub mod lexer;
pub mod parser;
pub mod syntax;
