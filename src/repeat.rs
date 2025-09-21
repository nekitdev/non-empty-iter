//! Non-empty iterators that repeat items.

use core::iter;

use non_zero_size::Size;

use crate::non_empty::NonEmptyIterator;

/// Creates [`Repeat<T>`] non-empty iterator that repeats the given item endlessly.
pub const fn repeat<T: Clone>(item: T) -> Repeat<T> {
    Repeat::new(item)
}

/// Represents non-empty iterators that repeat the given item endlessly.
///
/// This `struct` is created by the [`repeat`] function. See its documentation for more.
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct Repeat<T: Clone> {
    item: T,
}

impl<T: Clone> Repeat<T> {
    /// Constructs [`Self`].
    pub const fn new(item: T) -> Self {
        Self { item }
    }
}

impl<T: Clone> IntoIterator for Repeat<T> {
    type Item = T;

    type IntoIter = iter::Repeat<T>;

    fn into_iter(self) -> Self::IntoIter {
        iter::repeat(self.item)
    }
}

unsafe impl<T: Clone> NonEmptyIterator for Repeat<T> {}

/// Creates [`RepeatWith<F>`] non-empty iterator that repeats items
/// computed from the given function endlessly.
pub const fn repeat_with<T, F: FnMut() -> T>(function: F) -> RepeatWith<F> {
    RepeatWith::new(function)
}

/// Represents non-empty iterators that repeat items computed from the given function endlessly.
///
/// This `struct` is created by the [`repeat_with`] function. See its documentation for more.
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct RepeatWith<F> {
    function: F,
}

impl<F> RepeatWith<F> {
    /// Constructs [`Self`].
    pub const fn new(function: F) -> Self {
        Self { function }
    }
}

impl<T, F: FnMut() -> T> IntoIterator for RepeatWith<F> {
    type Item = T;

    type IntoIter = iter::RepeatWith<F>;

    fn into_iter(self) -> Self::IntoIter {
        iter::repeat_with(self.function)
    }
}

unsafe impl<T, F: FnMut() -> T> NonEmptyIterator for RepeatWith<F> {}

/// Creates [`RepeatN<T>`] non-empty iterator that repeats the given item the given number of times.
pub const fn repeat_n<T: Clone>(item: T, count: Size) -> RepeatN<T> {
    RepeatN::new(item, count)
}

/// Represents non-empty iterators that repeat the given item exactly the given number of times.
///
/// This `struct` is created by the [`repeat_n`] function. See its documentation for more.
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct RepeatN<T: Clone> {
    item: T,
    count: Size,
}

impl<T: Clone> RepeatN<T> {
    /// Constructs [`Self`].
    pub const fn new(item: T, count: Size) -> Self {
        Self { item, count }
    }
}

impl<T: Clone> IntoIterator for RepeatN<T> {
    type Item = T;

    type IntoIter = iter::RepeatN<T>;

    fn into_iter(self) -> Self::IntoIter {
        iter::repeat_n(self.item, self.count.get())
    }
}

unsafe impl<T: Clone> NonEmptyIterator for RepeatN<T> {}
