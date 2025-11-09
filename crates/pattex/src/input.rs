//! Input type for regex engine.

use std::{
    fmt::Debug,
    str::{CharIndices, Chars},
};

use memchr::memmem;
use parserc::{AsBytes, AsStr, Find, Input, StartWith};

use crate::errors::RegexError;

/// Input for regex engine.
pub trait PatternInput:
    Input<Item = char, Error = RegexError>
    + AsBytes
    + AsStr
    + StartWith<&'static str>
    + Find<&'static str>
    + Clone
    + Debug
    + PartialEq
{
}

/// `Input` for compute language parsing.
#[derive(Eq, PartialOrd, Ord, Hash)]
pub struct TokenStream<'a> {
    /// offset in the whole token stream.
    pub offset: usize,
    /// current segement string int the whole token stream.
    pub value: &'a str,
}

impl<'a> Clone for TokenStream<'a> {
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            value: self.value,
        }
    }
}

impl<'a> Debug for TokenStream<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenStream")
            .field("offset", &self.offset)
            .field("value", &self.value)
            .finish()
    }
}

impl<'a> PartialEq for TokenStream<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset && self.value == other.value
    }
}

impl<'a> From<&'a str> for TokenStream<'a> {
    fn from(value: &'a str) -> Self {
        TokenStream { offset: 0, value }
    }
}

impl<'a> From<(usize, &'a str)> for TokenStream<'a> {
    fn from(value: (usize, &'a str)) -> Self {
        TokenStream {
            offset: value.0,
            value: value.1,
        }
    }
}

impl<'a> Input for TokenStream<'a> {
    type Item = char;

    type Error = RegexError;

    type Iter = Chars<'a>;

    type IterIndices = CharIndices<'a>;

    #[inline]
    fn len(&self) -> usize {
        self.value.len()
    }

    #[inline]
    fn split_to(&mut self, at: usize) -> Self {
        let (first, last) = self.value.split_at(at);

        self.value = last;
        let offset = self.offset;
        self.offset += at;

        TokenStream {
            offset,
            value: first,
        }
    }

    #[inline]
    fn split_off(&mut self, at: usize) -> Self {
        let (first, last) = self.value.split_at(at);

        self.value = first;

        TokenStream {
            offset: self.offset + at,
            value: last,
        }
    }

    #[inline]
    fn iter(&self) -> Self::Iter {
        self.value.chars()
    }

    #[inline]
    fn iter_indices(&self) -> Self::IterIndices {
        self.value.char_indices()
    }

    #[inline]
    fn start(&self) -> usize {
        self.offset
    }

    #[inline]
    fn end(&self) -> usize {
        self.offset + self.value.len()
    }
}

impl<'a> AsBytes for TokenStream<'a> {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }
}

impl<'a> AsStr for TokenStream<'a> {
    #[inline]
    fn as_str(&self) -> &str {
        self.value
    }
}

impl<'a> StartWith<&str> for TokenStream<'a> {
    #[inline]
    fn starts_with(&self, needle: &str) -> Option<usize> {
        if self.as_bytes().starts_with(needle.as_bytes()) {
            Some(needle.len())
        } else {
            None
        }
    }
}

impl<'a> StartWith<&[u8]> for TokenStream<'a> {
    #[inline]
    fn starts_with(&self, needle: &[u8]) -> Option<usize> {
        if self.as_bytes().starts_with(needle) {
            Some(needle.len())
        } else {
            None
        }
    }
}

impl<'a, const N: usize> StartWith<&[u8; N]> for TokenStream<'a> {
    #[inline]
    fn starts_with(&self, needle: &[u8; N]) -> Option<usize> {
        if self.as_bytes().starts_with(needle) {
            Some(needle.len())
        } else {
            None
        }
    }
}

impl<'a> Find<&str> for TokenStream<'a> {
    #[inline]
    fn find(&self, needle: &str) -> Option<usize> {
        memmem::find(self.as_bytes(), needle.as_bytes())
    }
}

impl<'a> Find<&[u8]> for TokenStream<'a> {
    #[inline]
    fn find(&self, needle: &[u8]) -> Option<usize> {
        memmem::find(self.as_bytes(), needle)
    }
}

impl<'a, const N: usize> Find<&[u8; N]> for TokenStream<'a> {
    #[inline]
    fn find(&self, needle: &[u8; N]) -> Option<usize> {
        memmem::find(self.as_bytes(), needle)
    }
}

impl<'a> Find<TokenStream<'a>> for TokenStream<'a> {
    #[inline]
    fn find(&self, needle: TokenStream<'a>) -> Option<usize> {
        memmem::find(self.as_bytes(), needle.value.as_bytes())
    }
}

impl<'a> PatternInput for TokenStream<'a> {}
