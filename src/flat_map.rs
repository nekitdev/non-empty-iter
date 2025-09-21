//! Mapping non-empty iterators' items to non-empty iterators that have their items yielded.

use core::iter;

use crate::non_empty::{IntoNonEmptyIterator, NonEmptyIterator};

/// Represents non-empty iterators which map items to non-empty iterators that have
/// their items yielded.
///
/// This `struct` is created by the [`flat_map`] method on [`NonEmptyIterator`].
/// See its documentation for more.
///
/// [`flat_map`]: NonEmptyIterator::flat_map
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct FlatMap<I: NonEmptyIterator, J: IntoNonEmptyIterator, F: FnMut(I::Item) -> J> {
    non_empty: I,
    function: F,
}

impl<I: NonEmptyIterator, J: IntoNonEmptyIterator, F: FnMut(I::Item) -> J> FlatMap<I, J, F> {
    /// Constructs [`Self`].
    pub const fn new(non_empty: I, function: F) -> Self {
        Self {
            non_empty,
            function,
        }
    }
}

impl<I: NonEmptyIterator, J: IntoNonEmptyIterator, F: FnMut(I::Item) -> J> IntoIterator
    for FlatMap<I, J, F>
{
    type Item = J::Item;

    type IntoIter = iter::FlatMap<I::IntoIter, J, F>;

    fn into_iter(self) -> Self::IntoIter {
        self.non_empty.into_iter().flat_map(self.function)
    }
}

unsafe impl<I: NonEmptyIterator, J: IntoNonEmptyIterator, F: FnMut(I::Item) -> J> NonEmptyIterator
    for FlatMap<I, J, F>
{
}
