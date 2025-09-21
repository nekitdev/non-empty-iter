//! Inspecting items of non-empty iterators.

use core::iter;

use crate::non_empty::NonEmptyIterator;

/// Represents non-empty iterators that allow inspecting each item before yielding it.
///
/// This `struct` is created by the [`inspect`] method on [`NonEmptyIterator`].
/// See its documentation for more.
///
/// [`inspect`]: NonEmptyIterator::inspect
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Inspect<I: NonEmptyIterator, F> {
    non_empty: I,
    function: F,
}

impl<I: NonEmptyIterator, F> Inspect<I, F> {
    /// Constructs [`Self`].
    pub const fn new(non_empty: I, function: F) -> Self {
        Self {
            non_empty,
            function,
        }
    }
}

impl<I: NonEmptyIterator, F: FnMut(&I::Item)> IntoIterator for Inspect<I, F> {
    type Item = I::Item;

    type IntoIter = iter::Inspect<I::IntoIter, F>;

    fn into_iter(self) -> Self::IntoIter {
        self.non_empty.into_iter().inspect(self.function)
    }
}

unsafe impl<I: NonEmptyIterator, F: FnMut(&I::Item)> NonEmptyIterator for Inspect<I, F> {}
