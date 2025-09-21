//! Cloning the items of non-empty iterators.

use core::iter;

use crate::non_empty::NonEmptyIterator;

/// Represents non-empty iterators that clone the items of the underlying non-empty iterator.
///
/// This `struct` is created by the [`cloned`] method on [`NonEmptyIterator`].
/// See its documentation for more.
///
/// [`cloned`]: NonEmptyIterator::cloned
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Cloned<I: NonEmptyIterator> {
    non_empty: I,
}

impl<I: NonEmptyIterator> Cloned<I> {
    /// Constructs [`Self`].
    pub const fn new(non_empty: I) -> Self {
        Self { non_empty }
    }
}

impl<'a, I, T> IntoIterator for Cloned<I>
where
    I: NonEmptyIterator<Item = &'a T>,
    T: Clone + 'a,
{
    type Item = T;

    type IntoIter = iter::Cloned<I::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.non_empty.into_iter().cloned()
    }
}

unsafe impl<'a, I, T> NonEmptyIterator for Cloned<I>
where
    I: NonEmptyIterator<Item = &'a T>,
    T: Clone + 'a,
{
}
