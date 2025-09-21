//! Linking non-empty and possibly empty iterators together.

use core::iter;

use crate::non_empty::{IntoNonEmptyIterator, NonEmptyIterator};

/// Converts the given arguments to iterators and links them together.
///
/// The first argument must be [`IntoNonEmptyIterator`], while the second one can simply
/// implement [`IntoIterator`] yielding the same item type.
pub fn chain<I: IntoNonEmptyIterator, J: IntoIterator<Item = I::Item>>(
    non_empty: I,
    maybe_empty: J,
) -> Chain<I::IntoNonEmptyIter, J::IntoIter> {
    Chain::new(non_empty.into_non_empty_iter(), maybe_empty.into_iter())
}

/// Represents non-empty iterators that link two iterators together.
///
/// The first iterator must be [`NonEmptyIterator`], while the second one can simply
/// implement [`Iterator`] yielding the same item type.
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Chain<I: NonEmptyIterator, J: Iterator<Item = I::Item>> {
    non_empty: I,
    maybe_empty: J,
}

impl<I: NonEmptyIterator, J: Iterator<Item = I::Item>> Chain<I, J> {
    /// Constructs [`Self`].
    pub const fn new(non_empty: I, maybe_empty: J) -> Self {
        Self {
            non_empty,
            maybe_empty,
        }
    }
}

impl<I: NonEmptyIterator, J: Iterator<Item = I::Item>> IntoIterator for Chain<I, J> {
    type Item = I::Item;

    type IntoIter = iter::Chain<I::IntoIter, J>;

    fn into_iter(self) -> Self::IntoIter {
        self.non_empty.into_iter().chain(self.maybe_empty)
    }
}

unsafe impl<I: NonEmptyIterator, J: Iterator<Item = I::Item>> NonEmptyIterator for Chain<I, J> {}
