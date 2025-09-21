//! Non-empty adapter.

use crate::non_empty::NonEmptyIterator;

/// Adapts [`IntoIterator`] values that are known to be non-empty to implement [`NonEmptyIterator`].
///
/// This adapter is primarily used in the [`TryIntoNonEmptyIterator`] trait implementation.
///
/// [`TryIntoNonEmptyIterator`]: crate::non_empty::TryIntoNonEmptyIterator
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct NonEmptyAdapter<I: IntoIterator> {
    iterable: I,
}

impl<I: IntoIterator> NonEmptyAdapter<I> {
    /// Constructs [`Self`].
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the provided iterable is non-empty.
    pub const unsafe fn new(iterable: I) -> Self {
        Self { iterable }
    }
}

impl<I: IntoIterator> IntoIterator for NonEmptyAdapter<I> {
    type Item = I::Item;

    type IntoIter = I::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iterable.into_iter()
    }
}

unsafe impl<I: IntoIterator> NonEmptyIterator for NonEmptyAdapter<I> {}
