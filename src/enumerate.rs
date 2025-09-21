//! Enumerating items in non-empty iterators.

use core::iter;

use crate::non_empty::NonEmptyIterator;

/// Represents non-empty iterators that yield the current count and the item during iteration.
///
/// This `struct` is created by the [`enumerate`] method on [`NonEmptyIterator`].
/// See its documentation for more.
///
/// [`enumerate`]: NonEmptyIterator::enumerate
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Enumerate<I: NonEmptyIterator> {
    non_empty: I,
}

impl<I: NonEmptyIterator> Enumerate<I> {
    /// Constructs [`Self`].
    pub const fn new(non_empty: I) -> Self {
        Self { non_empty }
    }
}

impl<I: NonEmptyIterator> IntoIterator for Enumerate<I> {
    type Item = (usize, I::Item);

    type IntoIter = iter::Enumerate<I::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.non_empty.into_iter().enumerate()
    }
}

unsafe impl<I: NonEmptyIterator> NonEmptyIterator for Enumerate<I> {}
