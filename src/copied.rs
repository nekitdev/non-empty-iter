//! Copying the items of non-empty iterators.

use core::iter;

use crate::non_empty::NonEmptyIterator;

/// Represents non-empty iterators that copy the items of the underlying non-empty iterator.
///
/// This `struct` is created by the [`copied`] method on [`NonEmptyIterator`].
/// See its documentation for more.
///
/// [`copied`]: NonEmptyIterator::copied
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Copied<I: NonEmptyIterator> {
    non_empty: I,
}

impl<I: NonEmptyIterator> Copied<I> {
    /// Constructs [`Self`].
    pub const fn new(non_empty: I) -> Self {
        Self { non_empty }
    }
}

impl<'a, I, T> IntoIterator for Copied<I>
where
    I: NonEmptyIterator<Item = &'a T>,
    T: Copy + 'a,
{
    type Item = T;

    type IntoIter = iter::Copied<I::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.non_empty.into_iter().copied()
    }
}

unsafe impl<'a, I, T> NonEmptyIterator for Copied<I>
where
    I: NonEmptyIterator<Item = &'a T>,
    T: Copy + 'a,
{
}
