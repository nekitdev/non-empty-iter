//! Mapping items of non-empty iterators.

use core::iter;

use crate::non_empty::NonEmptyIterator;

/// Represents non-empty iterators that map the items of the non-empty iterator with the function.
///
/// This `struct` is created by the [`map`] method on [`NonEmptyIterator`].
/// See its documentation for more.
///
/// [`map`]: NonEmptyIterator::map
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Map<I: NonEmptyIterator, F> {
    non_empty: I,
    function: F,
}

impl<I: NonEmptyIterator, F> Map<I, F> {
    /// Constructs [`Self`].
    pub const fn new(non_empty: I, function: F) -> Self {
        Self {
            non_empty,
            function,
        }
    }
}

impl<U, I: NonEmptyIterator, F: FnMut(I::Item) -> U> IntoIterator for Map<I, F> {
    type Item = U;

    type IntoIter = iter::Map<I::IntoIter, F>;

    fn into_iter(self) -> Self::IntoIter {
        self.non_empty.into_iter().map(self.function)
    }
}

unsafe impl<U, I: NonEmptyIterator, F: FnMut(I::Item) -> U> NonEmptyIterator for Map<I, F> {}
