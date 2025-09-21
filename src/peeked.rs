//! Non-empty peeked iterators.

use core::iter;

use crate::non_empty::NonEmptyIterator;

/// Represents non-empty peeked iterators.
///
/// This `struct` is created by the [`peeked`] method on [`NonEmptyIterator`].
/// See its documentation for more.
///
/// [`peeked`]: NonEmptyIterator::peeked
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Peeked<I: Iterator> {
    item: I::Item,
    rest: I,
}

impl<I: Iterator> Peeked<I> {
    /// Constructs [`Self`].
    pub const fn new(item: I::Item, rest: I) -> Self {
        Self { item, rest }
    }

    /// Returns the immutable reference to the peeked item.
    pub const fn peek(&self) -> &I::Item {
        &self.item
    }

    /// Returns the mutable reference to the peeked item.
    pub const fn peek_mut(&mut self) -> &mut I::Item {
        &mut self.item
    }

    /// Returns the peeked item and the underlying iterator.
    pub fn get(self) -> (I::Item, I) {
        (self.item, self.rest)
    }
}

impl<I: Iterator> IntoIterator for Peeked<I> {
    type Item = I::Item;

    type IntoIter = iter::Chain<iter::Once<I::Item>, I>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(self.item).chain(self.rest)
    }
}

unsafe impl<I: Iterator> NonEmptyIterator for Peeked<I> {}
