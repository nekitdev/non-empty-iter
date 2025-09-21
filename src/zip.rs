//! Zipping two non-empty iterators together.

use core::iter;

use crate::non_empty::{IntoNonEmptyIterator, NonEmptyIterator};

/// Converts the given arguments to non-empty iterators and zips them.
///
/// See the documentation of [`NonEmptyIterator::zip`] for more.
pub fn zip<I: IntoNonEmptyIterator, J: IntoNonEmptyIterator>(
    first: I,
    second: J,
) -> Zip<I::IntoNonEmptyIter, J::IntoNonEmptyIter> {
    Zip::new(first.into_non_empty_iter(), second.into_non_empty_iter())
}

/// Represents non-empty iterators that iterate over two other non-empty iterators simultaneously.
///
/// This struct is created by the [`zip`] function and [`NonEmptyIterator::zip`] method.
/// See their documentation for more.
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Zip<I: NonEmptyIterator, J: NonEmptyIterator> {
    first: I,
    second: J,
}

impl<I: NonEmptyIterator, J: NonEmptyIterator> Zip<I, J> {
    /// Constructs [`Self`].
    pub const fn new(first: I, second: J) -> Self {
        Self { first, second }
    }
}

impl<I: NonEmptyIterator, J: NonEmptyIterator> IntoIterator for Zip<I, J> {
    type Item = (I::Item, J::Item);

    type IntoIter = iter::Zip<I::IntoIter, J::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        iter::zip(self.first, self.second)
    }
}

unsafe impl<I: NonEmptyIterator, J: NonEmptyIterator> NonEmptyIterator for Zip<I, J> {}
