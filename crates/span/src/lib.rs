//！ `span` is a region of source code
#![cfg_attr(docsrs, feature(doc_cfg))]

use std::{
    cmp,
    ops::{Range, RangeFrom, RangeTo},
};

/// A region of source code.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Span<Idx> {
    /// No value.
    None,
    /// A (half-open) range bounded inclusively below and exclusively above (start..end in a future edition).
    Range(Range<Idx>),
    /// A range only bounded inclusively below (start..).
    RangeFrom(RangeFrom<Idx>),
    /// A range only bounded exclusively above (..end).
    RangeTo(RangeTo<Idx>),
    /// Match the whole source code.
    RangeFull,
}

impl<Idx> Span<Idx>
where
    Idx: PartialOrd + Ord + Copy,
{
    /// Create a range between two `span`s.
    ///
    /// If two spans intersect, returns `Span::None`.
    #[inline]
    pub fn between(&self, other: &Self) -> Span<Idx> {
        match (self, other) {
            (Span::None, Span::None) => Span::None,
            (Span::None, Span::Range(_)) => Span::None,
            (Span::None, Span::RangeFrom(_)) => Span::None,
            (Span::None, Span::RangeTo(_)) => Span::None,
            (Span::Range(_), Span::None) => Span::None,
            (Span::Range(range), Span::Range(other_range)) => {
                if range.end < other_range.start {
                    Span::Range(range.end..other_range.start)
                } else if other_range.end < range.start {
                    Span::Range(other_range.end..range.start)
                } else {
                    Span::None
                }
            }
            (Span::Range(range), Span::RangeFrom(range_from)) => {
                if range.end < range_from.start {
                    Span::Range(range.end..range_from.start)
                } else {
                    Span::None
                }
            }
            (Span::Range(range), Span::RangeTo(range_to)) => {
                if range_to.end < range.start {
                    Span::Range(range_to.end..range.start)
                } else {
                    Span::None
                }
            }
            (Span::RangeFrom(_), Span::None) => Span::None,
            (Span::RangeFrom(range_from), Span::Range(range)) => {
                if range.end < range_from.start {
                    Span::Range(range.end..range_from.start)
                } else {
                    Span::None
                }
            }
            (Span::RangeFrom(_), Span::RangeFrom(_)) => Span::None,
            (Span::RangeFrom(range_from), Span::RangeTo(range_to)) => {
                if range_to.end < range_from.start {
                    Span::Range(range_to.end..range_from.start)
                } else {
                    Span::None
                }
            }
            (Span::RangeTo(_), Span::None) => Span::None,
            (Span::RangeTo(range_to), Span::Range(range)) => {
                if range_to.end < range.start {
                    Span::Range(range_to.end..range.start)
                } else {
                    Span::None
                }
            }
            (Span::RangeTo(range_to), Span::RangeFrom(range_from)) => {
                if range_to.end < range_from.start {
                    Span::Range(range_to.end..range_from.start)
                } else {
                    Span::None
                }
            }
            (Span::RangeTo(_), Span::RangeTo(_)) => Span::None,
            (Span::None, Span::RangeFull) => Span::None,
            (Span::Range(_), Span::RangeFull) => Span::None,
            (Span::RangeFrom(_), Span::RangeFull) => Span::None,
            (Span::RangeTo(_), Span::RangeFull) => Span::None,
            (Span::RangeFull, Span::None) => Span::None,
            (Span::RangeFull, Span::Range(_)) => Span::None,
            (Span::RangeFull, Span::RangeFrom(_)) => Span::None,
            (Span::RangeFull, Span::RangeTo(_)) => Span::None,
            (Span::RangeFull, Span::RangeFull) => Span::None,
        }
    }

    /// Union two range.
    #[inline]
    pub fn union(&self, other: &Self) -> Self {
        match (self, other) {
            (Span::None, Span::None) => Span::None,
            (Span::None, Span::Range(_)) => other.clone(),
            (Span::None, Span::RangeFrom(_)) => other.clone(),
            (Span::None, Span::RangeTo(_)) => other.clone(),
            (Span::Range(_), Span::None) => self.clone(),
            (Span::Range(range), Span::Range(other_range)) => {
                let start = cmp::min(range.start, other_range.start);
                let end = cmp::max(range.end, other_range.end);

                Span::Range(start..end)
            }
            (Span::Range(range), Span::RangeFrom(range_from)) => {
                let start = cmp::min(range.start, range_from.start);

                Span::RangeFrom(start..)
            }
            (Span::Range(range), Span::RangeTo(range_to)) => {
                let end = cmp::max(range.end, range_to.end);

                Span::RangeTo(..end)
            }
            (Span::RangeFrom(_), Span::None) => self.clone(),
            (Span::RangeFrom(range_from), Span::Range(range)) => {
                if range_from.start > range.end {
                    Span::None
                } else {
                    Span::Range(range_from.start..range.end)
                }
            }
            (Span::RangeFrom(range_from), Span::RangeFrom(other_range_from)) => {
                let start = cmp::min(range_from.start, other_range_from.start);
                Span::RangeFrom(start..)
            }
            (Span::RangeFrom(_), Span::RangeTo(_)) => Span::RangeFull,
            (Span::RangeTo(_), Span::None) => self.clone(),
            (Span::RangeTo(range_to), Span::Range(range)) => {
                let end = cmp::max(range_to.end, range.end);

                Span::RangeTo(..end)
            }
            (Span::RangeTo(_), Span::RangeFrom(_)) => Span::RangeFull,
            (Span::RangeTo(range_to), Span::RangeTo(other_range_to)) => {
                let end = cmp::max(range_to.end, other_range_to.end);

                Span::RangeTo(..end)
            }
            (Span::None, Span::RangeFull) => Span::RangeFull,
            (Span::Range(_), Span::RangeFull) => Span::RangeFull,
            (Span::RangeFrom(_), Span::RangeFull) => Span::RangeFull,
            (Span::RangeTo(_), Span::RangeFull) => Span::RangeFull,
            (Span::RangeFull, Span::None) => Span::RangeFull,
            (Span::RangeFull, Span::Range(_)) => Span::RangeFull,
            (Span::RangeFull, Span::RangeFrom(_)) => Span::RangeFull,
            (Span::RangeFull, Span::RangeTo(_)) => Span::RangeFull,
            (Span::RangeFull, Span::RangeFull) => Span::RangeFull,
        }
    }
}
