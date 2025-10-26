//！ Parser combinator for `rust` language.
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod input;

#[cfg(feature = "lang")]
#[cfg_attr(docsrs, doc(cfg(feature = "lang")))]
pub mod lang;

pub mod errors;
pub mod parser;
pub mod syntax;
pub mod tokenizer;
