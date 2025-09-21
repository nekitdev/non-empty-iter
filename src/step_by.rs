//! Stepping non-empty iterators by the custom amount.

use core::iter;

use non_zero_size::Size;

use crate::non_empty::NonEmptyIterator;

/// Represents non-empty iterators for stepping non-empty iterators by the custom amount.
///
/// This `struct` is created by the [`step_by`] method on [`NonEmptyIterator`].
/// See its documentation for more.
///
/// [`step_by`]: NonEmptyIterator::step_by
#[derive(Debug, Clone)]
#[must_use = "non-empty iterators are lazy and do nothing unless consumed"]
pub struct StepBy<I: NonEmptyIterator> {
    non_empty: I,
    step: Size,
}

impl<I: NonEmptyIterator> StepBy<I> {
    /// Constructs [`Self`].
    pub const fn new(non_empty: I, step: Size) -> Self {
        Self { non_empty, step }
    }
}

impl<I: NonEmptyIterator> IntoIterator for StepBy<I> {
    type Item = I::Item;

    type IntoIter = iter::StepBy<I::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.non_empty.into_iter().step_by(self.step.get())
    }
}

unsafe impl<I: NonEmptyIterator> NonEmptyIterator for StepBy<I> {}
