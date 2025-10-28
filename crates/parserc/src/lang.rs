//! Input types for parsing text based source codes.

use std::{fmt::Debug, iter::Enumerate, marker::PhantomData, str::Bytes};

use memchr::memmem;

use crate::{ParseError, input::*};

/// The `Input` short for compute language parsing.
pub trait LangInput:
    Input<Item = u8>
    + AsBytes
    + AsStr
    + StartWith<&'static str>
    + StartWith<&'static [u8]>
    + Find<&'static str>
    + Find<&'static [u8]>
    + Clone
    + Debug
    + PartialEq
{
}

/// `Input` for compute language parsing.
#[derive(Eq, PartialOrd, Ord, Hash)]
pub struct TokenStream<'a, E> {
    /// offset in the whole token stream.
    pub offset: usize,
    /// current segement string int the whole token stream.
    pub value: &'a str,
    /// Error type returns by this input.
    _marker: PhantomData<E>,
}

impl<'a, E> Clone for TokenStream<'a, E> {
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            value: self.value,
            _marker: Default::default(),
        }
    }
}

impl<'a, E> Debug for TokenStream<'a, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenStream")
            .field("offset", &self.offset)
            .field("value", &self.value)
            .finish()
    }
}

impl<'a, E> PartialEq for TokenStream<'a, E> {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset && self.value == other.value
    }
}

impl<'a, E> From<&'a str> for TokenStream<'a, E> {
    fn from(value: &'a str) -> Self {
        TokenStream {
            offset: 0,
            value,
            _marker: Default::default(),
        }
    }
}

impl<'a, E> From<(usize, &'a str)> for TokenStream<'a, E> {
    fn from(value: (usize, &'a str)) -> Self {
        TokenStream {
            offset: value.0,
            value: value.1,
            _marker: Default::default(),
        }
    }
}

impl<'a, E> Input for TokenStream<'a, E>
where
    E: ParseError,
{
    type Item = u8;

    type Error = E;

    type Iter = Bytes<'a>;

    type IterIndices = Enumerate<Self::Iter>;

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
            _marker: Default::default(),
        }
    }

    #[inline]
    fn split_off(&mut self, at: usize) -> Self {
        let (first, last) = self.value.split_at(at);

        self.value = first;

        TokenStream {
            offset: self.offset + at,
            value: last,
            _marker: Default::default(),
        }
    }

    #[inline]
    fn iter(&self) -> Self::Iter {
        self.value.bytes()
    }

    #[inline]
    fn iter_indices(&self) -> Self::IterIndices {
        self.iter().enumerate()
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

impl<'a, E> AsBytes for TokenStream<'a, E> {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }
}

impl<'a, E> AsStr for TokenStream<'a, E> {
    #[inline]
    fn as_str(&self) -> &str {
        self.value
    }
}

impl<'a, E> StartWith<&str> for TokenStream<'a, E> {
    #[inline]
    fn starts_with(&self, needle: &str) -> Option<usize> {
        if self.as_bytes().starts_with(needle.as_bytes()) {
            Some(needle.len())
        } else {
            None
        }
    }
}

impl<'a, E> StartWith<&[u8]> for TokenStream<'a, E> {
    #[inline]
    fn starts_with(&self, needle: &[u8]) -> Option<usize> {
        if self.as_bytes().starts_with(needle) {
            Some(needle.len())
        } else {
            None
        }
    }
}

impl<'a, const N: usize, E> StartWith<&[u8; N]> for TokenStream<'a, E> {
    #[inline]
    fn starts_with(&self, needle: &[u8; N]) -> Option<usize> {
        if self.as_bytes().starts_with(needle) {
            Some(needle.len())
        } else {
            None
        }
    }
}

impl<'a, E> Find<&str> for TokenStream<'a, E> {
    #[inline]
    fn find(&self, needle: &str) -> Option<usize> {
        memmem::find(self.as_bytes(), needle.as_bytes())
    }
}

impl<'a, E> Find<&[u8]> for TokenStream<'a, E> {
    #[inline]
    fn find(&self, needle: &[u8]) -> Option<usize> {
        memmem::find(self.as_bytes(), needle)
    }
}

impl<'a, const N: usize, E> Find<&[u8; N]> for TokenStream<'a, E> {
    #[inline]
    fn find(&self, needle: &[u8; N]) -> Option<usize> {
        memmem::find(self.as_bytes(), needle)
    }
}

impl<'a, E> LangInput for TokenStream<'a, E> where E: ParseError + Clone {}
