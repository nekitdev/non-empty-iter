//! Iterating over only the first provided number of items in non-empty iterators.

use core::iter;

use non_zero_size::Size;

use crate::non_empty::NonEmptyIterator;

/// Represents non-empty iterators that only iterate over the first given number of items
/// of the underlying iterator.
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Take<I: NonEmptyIterator> {
    non_empty: I,
    count: Size,
}

impl<I: NonEmptyIterator> Take<I> {
    /// Constructs [`Self`].
    pub const fn new(non_empty: I, count: Size) -> Self {
        Self { non_empty, count }
    }
}

impl<I: NonEmptyIterator> IntoIterator for Take<I> {
    type Item = I::Item;

    type IntoIter = iter::Take<I::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.non_empty.into_iter().take(self.count.get())
    }
}

unsafe impl<I: NonEmptyIterator> NonEmptyIterator for Take<I> {}
