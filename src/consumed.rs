//! Consumed pairs.

/// [`Consumed<I>`] is the same as `(I::Item, I::IntoIter)` for `I` implementing [`IntoIterator`].
pub type Consumed<I> = (<I as IntoIterator>::Item, <I as IntoIterator>::IntoIter);
