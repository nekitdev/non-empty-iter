//! Fusing non-empty iterators.

use core::iter;

use crate::non_empty::NonEmptyIterator;

/// Represents non-empty iterators that yield [`None`] forever after the underlying
/// iterator yields [`None`] once.
///
/// This `struct` is created by the [`fuse`] method on [`NonEmptyIterator`].
/// See its documentation for more.
///
/// [`fuse`]: NonEmptyIterator::fuse
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Fuse<I: NonEmptyIterator> {
    non_empty: I,
}

impl<I: NonEmptyIterator> Fuse<I> {
    /// Constructs [`Self`].
    pub const fn new(non_empty: I) -> Self {
        Self { non_empty }
    }
}

impl<I: NonEmptyIterator> IntoIterator for Fuse<I> {
    type Item = I::Item;

    type IntoIter = iter::Fuse<I::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.non_empty.into_iter().fuse()
    }
}

unsafe impl<I: NonEmptyIterator> NonEmptyIterator for Fuse<I> {}
