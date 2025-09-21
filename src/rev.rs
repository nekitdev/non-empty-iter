//! Reversing the non-empty iteration direction.

use core::iter;

use crate::non_empty::NonEmptyIterator;

/// Represents non-empty iterators with the direction reversed.
///
/// This `struct` is created by the [`rev`] method on [`NonEmptyIterator`].
/// See its documentation for more.
///
/// [`rev`]: NonEmptyIterator::rev
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Rev<I: NonEmptyIterator>
where
    I::IntoIter: DoubleEndedIterator,
{
    non_empty: I,
}

impl<I: NonEmptyIterator> Rev<I>
where
    I::IntoIter: DoubleEndedIterator,
{
    /// Constructs [`Self`].
    pub const fn new(non_empty: I) -> Self {
        Self { non_empty }
    }
}

impl<I: NonEmptyIterator> IntoIterator for Rev<I>
where
    I::IntoIter: DoubleEndedIterator,
{
    type Item = I::Item;

    type IntoIter = iter::Rev<I::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.non_empty.into_iter().rev()
    }
}

unsafe impl<I: NonEmptyIterator> NonEmptyIterator for Rev<I> where I::IntoIter: DoubleEndedIterator {}
