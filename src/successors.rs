//! Non-empty iterators that compute each successive item from the preceding one.

use core::iter;

use crate::non_empty::NonEmptyIterator;

/// Creates [`Successors<T, S>`] non-empty iterator which, starting from the initial item,
/// computes each successive item from the preceding one.
pub const fn successors<T, S: FnMut(&T) -> Option<T>>(
    initial: T,
    successor: S,
) -> Successors<T, S> {
    Successors::new(initial, successor)
}

/// Represents non-empty iterators which, starting from the initial item,
/// compute each successive item from preceding one.
///
/// This `struct` is created by the [`successors`] function. See its documentation for more.
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Successors<T, S: FnMut(&T) -> Option<T>> {
    initial: T,
    successor: S,
}

impl<T, S: FnMut(&T) -> Option<T>> Successors<T, S> {
    /// Constructs [`Self`].
    pub const fn new(initial: T, successor: S) -> Self {
        Self { initial, successor }
    }
}

impl<T, S: FnMut(&T) -> Option<T>> IntoIterator for Successors<T, S> {
    type Item = T;

    type IntoIter = iter::Successors<T, S>;

    fn into_iter(self) -> Self::IntoIter {
        iter::successors(Some(self.initial), self.successor)
    }
}

unsafe impl<T, S: FnMut(&T) -> Option<T>> NonEmptyIterator for Successors<T, S> {}
