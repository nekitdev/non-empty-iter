//! Repeating non-empty iterators endlessly.

use core::iter;

use crate::non_empty::NonEmptyIterator;

/// Represents non-empty iterators that repeat endlessly.
///
/// This `struct` is created by the [`cycle`] method on [`NonEmptyIterator`].
/// See its documentation for more.
///
/// [`cycle`]: NonEmptyIterator::cycle
pub struct Cycle<I: NonEmptyIterator>
where
    I::IntoIter: Clone,
{
    non_empty: I,
}

impl<I: NonEmptyIterator> Cycle<I>
where
    I::IntoIter: Clone,
{
    /// Constructs [`Self`].
    pub const fn new(non_empty: I) -> Self {
        Self { non_empty }
    }
}

impl<I: NonEmptyIterator> IntoIterator for Cycle<I>
where
    I::IntoIter: Clone,
{
    type Item = I::Item;

    type IntoIter = iter::Cycle<I::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.non_empty.into_iter().cycle()
    }
}

unsafe impl<I: NonEmptyIterator> NonEmptyIterator for Cycle<I> where I::IntoIter: Clone {}
