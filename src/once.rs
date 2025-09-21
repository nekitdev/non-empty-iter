//! Yielding items exactly once.

use core::iter;

use crate::non_empty::NonEmptyIterator;

/// Creates [`Once<T>`], non-empty iterator that yields the given value exactly once.
pub const fn once<T>(value: T) -> Once<T> {
    Once::new(value)
}

/// Creates [`OnceWith<T>`] non-empty iterator that yields the value
/// computed from the given function exactly once.
pub const fn once_with<T, F: FnOnce() -> T>(function: F) -> OnceWith<F> {
    OnceWith::new(function)
}

/// Represents non-empty iterators that yield the given value exactly once.
///
/// This `struct` is created by the [`once`] function. See its documentation for more.
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Once<T> {
    value: T,
}

impl<T> Once<T> {
    /// Constructs [`Self`].
    pub const fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> IntoIterator for Once<T> {
    type Item = T;

    type IntoIter = iter::Once<T>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(self.value)
    }
}

unsafe impl<T> NonEmptyIterator for Once<T> {}

/// Represents non-empty iterators that yield the value computed from
/// the given function exactly once.
///
/// This `struct` is created by the [`once_with`] function. See its documentation for more.
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct OnceWith<F> {
    function: F,
}

impl<F> OnceWith<F> {
    /// Constructs [`Self`].
    pub const fn new(function: F) -> Self {
        Self { function }
    }
}

impl<T, F: FnOnce() -> T> IntoIterator for OnceWith<F> {
    type Item = T;

    type IntoIter = iter::OnceWith<F>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once_with(self.function)
    }
}

unsafe impl<T, F: FnOnce() -> T> NonEmptyIterator for OnceWith<F> {}
