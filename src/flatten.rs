//! Flattening one level of nesting in non-empty iterators of items that can be turned
//! into non-empty iterators.

use core::iter;

use crate::non_empty::{IntoNonEmptyIterator, NonEmptyIterator};

/// Represents non-empty iterators that flatten one level of nesting in non-empty iterators
/// of items that can be turned into non-empty iterators.
///
/// This `struct` is created by the [`flatten`] method on [`NonEmptyIterator`].
/// See its documentation for more.
///
/// [`flatten`]: NonEmptyIterator::flatten
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Flatten<I: NonEmptyIterator>
where
    I::Item: IntoNonEmptyIterator,
{
    non_empty: I,
}

impl<I: NonEmptyIterator> Flatten<I>
where
    I::Item: IntoNonEmptyIterator,
{
    /// Constructs [`Self`].
    pub const fn new(non_empty: I) -> Self {
        Self { non_empty }
    }
}

impl<I: NonEmptyIterator<Item = J>, J: IntoNonEmptyIterator> IntoIterator for Flatten<I> {
    type Item = J::Item;

    type IntoIter = iter::Flatten<I::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.non_empty.into_iter().flatten()
    }
}

unsafe impl<I: NonEmptyIterator> NonEmptyIterator for Flatten<I> where I::Item: IntoNonEmptyIterator {}
