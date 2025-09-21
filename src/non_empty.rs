//! The non-empty iterator core traits.

use core::{
    cmp::Ordering,
    iter::{self, Product, Sum},
};

use non_zero_size::Size;

use crate::{
    adapter::NonEmptyAdapter, chain::Chain, cloned::Cloned, copied::Copied, cycle::Cycle,
    enumerate::Enumerate, flat_map::FlatMap, flatten::Flatten, fuse::Fuse, inspect::Inspect,
    map::Map, peeked::Peeked, rev::Rev, step_by::StepBy, take::Take, zip::Zip,
};

/// Represents [`Iterator`] that is guaranteed to be non-empty
/// (equivalently, having at least one item).
///
/// Note that this trait is `unsafe` to implement, as the implementor must guarantee non-emptiness.
///
/// Moreover, [`NonEmptyIterator`] has [`IntoIterator`] and [`Sized`] bounds, as it is always
/// consumed by value and converted into the underlying iterator.
///
/// One can use the `for` loop with non-empty iterators, and downgrade to the regular [`Iterator`]
/// by calling [`into_iter`] or [`consume`].
///
/// # Safety
///
/// By implementing [`NonEmptyIterator`] the implementor is responsible
/// for ensuring that non-emptiness holds. Violating this invariant causes
/// *Undefined Behavior*!
///
/// [`into_iter`]: IntoIterator::into_iter
/// [`consume`]: NonEmptyIterator::consume
pub unsafe trait NonEmptyIterator: IntoIterator + Sized {
    /// Consumes the non-empty iterator, returning the next item
    /// along with the possibly empty iterator.
    #[must_use]
    fn consume(self) -> (Self::Item, Self::IntoIter) {
        let mut iterator = self.into_iter();

        // SAFETY: the implementor guarantees the iterator is non-empty
        let item = unsafe { iterator.next().unwrap_unchecked() };

        (item, iterator)
    }

    /// Consumes the non-empty iterator, returning the item count.
    ///
    /// See also [`count`] on [`Iterator`].
    ///
    /// # Non-zero
    ///
    /// The returned count is guaranteed to be non-zero.
    ///
    /// [`count`]: Iterator::count
    #[must_use]
    fn count(self) -> Size {
        let count = self.into_iter().count();

        // SAFETY: the implementor guarantees the iterator is non-empty
        // therefore, `count` is non-zero
        unsafe { Size::new_unchecked(count) }
    }

    /// Creates non-empty iterators that yield the current count and the item during iteration.
    ///
    /// See also [`enumerate`] on [`Iterator`].
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`enumerate`]: Iterator::enumerate
    fn enumerate(self) -> Enumerate<Self> {
        Enumerate::new(self)
    }

    /// Peeks at the next item of the non-empty iterator, returning it along
    /// with the possibly empty iterator.
    ///
    /// This is equivalent to calling [`consume`] and wrapping the output.
    ///
    /// See also [`peekable`] on [`Iterator`].
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`consume`]: NonEmptyIterator::consume
    /// [`peekable`]: Iterator::peekable
    fn peeked(self) -> Peeked<Self::IntoIter> {
        let (item, rest) = self.consume();

        Peeked::new(item, rest)
    }

    /// Links the non-empty iterator with the provided possibly empty iterator.
    ///
    /// See also [`chain`] on [`Iterator`].
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`chain`]: Iterator::chain
    fn chain<I: IntoIterator<Item = Self::Item>>(self, other: I) -> Chain<Self, I::IntoIter> {
        Chain::new(self, other.into_iter())
    }

    /// Creates non-empty iterators that clone the items of the underlying non-empty iterator.
    ///
    /// See also [`cloned`] on [`Iterator`].
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`cloned`]: Iterator::cloned
    fn cloned<'a, T>(self) -> Cloned<Self>
    where
        Self: NonEmptyIterator<Item = &'a T>,
        T: Clone + 'a,
    {
        Cloned::new(self)
    }

    /// Creates non-empty iterators that copy the items of the underlying non-empty iterator.
    ///
    /// See also [`copied`] on [`Iterator`].
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`copied`]: Iterator::copied
    fn copied<'a, T>(self) -> Copied<Self>
    where
        Self: NonEmptyIterator<Item = &'a T>,
        T: Copy + 'a,
    {
        Copied::new(self)
    }

    /// Zips the non-empty iterator with the provided non-empty iterator.
    ///
    /// This allows to iterate over the items of both iterators simultaneously.
    ///
    /// See also [`zip`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that the argument is required to be [`IntoNonEmptyIterator`].
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`zip`]: Iterator::zip
    fn zip<I: IntoNonEmptyIterator>(self, other: I) -> Zip<Self, I::IntoNonEmptyIter> {
        Zip::new(self, other.into_non_empty_iter())
    }

    /// Sums the items of the non-empty iterator together.
    ///
    /// See also [`sum`] on [`Iterator`].
    ///
    /// [`sum`]: Iterator::sum
    #[must_use]
    fn sum<S: Sum<Self::Item>>(self) -> S {
        self.into_iter().sum()
    }

    /// Multiplies the items of the non-empty iterator together.
    ///
    /// See also [`product`] on [`Iterator`].
    ///
    /// [`product`]: Iterator::product
    #[must_use]
    fn product<P: Product<Self::Item>>(self) -> P {
        self.into_iter().product()
    }

    /// Tests whether all items of the non-empty iterator match the predicate.
    ///
    /// See also [`all`] on [`Iterator`].
    ///
    /// [`all`]: Iterator::all
    fn all<P: FnMut(Self::Item) -> bool>(self, predicate: P) -> bool {
        self.into_iter().all(predicate)
    }

    /// Tests whether any items of the non-empty iterator match the predicate.
    ///
    /// See also [`any`] on [`Iterator`].
    ///
    /// [`any`]: Iterator::any
    fn any<P: FnMut(Self::Item) -> bool>(self, predicate: P) -> bool {
        self.into_iter().any(predicate)
    }

    /// Tests whether no items of the non-empty iterator match the predicate.
    ///
    /// This is equivalent to negating the output of [`any`].
    ///
    /// [`any`]: NonEmptyIterator::any
    fn none<P: FnMut(Self::Item) -> bool>(self, predicate: P) -> bool {
        !self.any(predicate)
    }

    /// Reduces the items of the non-empty iterator into the single one
    /// by repeatedly applying the given function.
    ///
    /// See also [`reduce`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that this function always returns some value, as the iterator is non-empty.
    ///
    /// [`reduce`]: Iterator::reduce
    #[must_use]
    fn reduce<F>(self, function: F) -> Self::Item
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        let output = self.into_iter().reduce(function);

        // SAFETY: the implementor guarantees the iterator is non-empty
        // therefore, `output` has to contain some value
        unsafe { output.unwrap_unchecked() }
    }

    /// Converts the non-empty iterator of pairs into the pair of collections.
    ///
    /// See also [`unzip`] on [`Iterator`].
    ///
    /// [`unzip`]: Iterator::unzip
    fn unzip<T, U, C: Default + Extend<T>, D: Default + Extend<U>>(self) -> (C, D)
    where
        Self: NonEmptyIterator<Item = (T, U)>,
    {
        self.into_iter().unzip()
    }

    /// Collects the items of the non-empty iterator into the collection.
    ///
    /// See also [`collect`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that the collection is required to be [`FromNonEmptyIterator`],
    /// though anything that is [`FromIterator`] is also [`FromNonEmptyIterator`].
    ///
    /// [`collect`]: Iterator::collect
    fn collect<C: FromNonEmptyIterator<Self::Item>>(self) -> C {
        C::from_non_empty_iter(self)
    }

    /// Similar to [`map`], but flattens produced non-empty iterators.
    ///
    /// See also [`flat_map`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that the function is required to return [`IntoNonEmptyIterator`].
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`map`]: NonEmptyIterator::map
    /// [`flat_map`]: Iterator::flat_map
    fn flat_map<J: IntoNonEmptyIterator, F: FnMut(Self::Item) -> J>(
        self,
        function: F,
    ) -> FlatMap<Self, J, F> {
        FlatMap::new(self, function)
    }

    /// Flattens one level of nesting in `self` non-empty iterator.
    ///
    /// See also [`flatten`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that the items are required to be [`IntoNonEmptyIterator`].
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`flatten`]: Iterator::flatten
    fn flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoNonEmptyIterator,
    {
        Flatten::new(self)
    }

    /// Equivalent to [`filter`] on [`Iterator`].
    ///
    /// Note that the returned iterator can be empty, depending on the predicate.
    ///
    /// [`filter`]: Iterator::filter
    fn filter<P: FnMut(&Self::Item) -> bool>(
        self,
        predicate: P,
    ) -> iter::Filter<Self::IntoIter, P> {
        self.into_iter().filter(predicate)
    }

    /// Equivalent to [`find`] on [`Iterator`].
    ///
    /// [`find`]: Iterator::find
    fn find<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> Option<Self::Item> {
        self.into_iter().find(predicate)
    }

    /// Equivalent to [`filter_map`] on [`Iterator`].
    ///
    /// Note that the returned iterator can be empty, depending on the function.
    ///
    /// [`filter_map`]: Iterator::filter_map
    fn filter_map<T, F: FnMut(Self::Item) -> Option<T>>(
        self,
        function: F,
    ) -> iter::FilterMap<Self::IntoIter, F> {
        self.into_iter().filter_map(function)
    }

    /// Equivalent to [`fold`] on [`Iterator`].
    ///
    /// [`fold`]: Iterator::fold
    #[must_use]
    fn fold<A, F: FnMut(A, Self::Item) -> A>(self, initial: A, function: F) -> A {
        self.into_iter().fold(initial, function)
    }

    /// Creates non-empty iterators that map the items of the non-empty iterator with the function.
    ///
    /// See also [`map`] on [`Iterator`].
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`map`]: Iterator::map
    fn map<U, F: FnMut(Self::Item) -> U>(self, function: F) -> Map<Self, F> {
        Map::new(self, function)
    }

    /// Returns the maximum item of the non-empty iterator.
    ///
    /// See also [`max`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that this function always returns some value, as the iterator is non-empty.
    ///
    /// [`max`]: Iterator::max
    #[must_use]
    fn max(self) -> Self::Item
    where
        Self::Item: Ord,
    {
        let max = self.into_iter().max();

        // SAFETY: the implementor guarantees the iterator is non-empty
        // therefore, `max` has to contain some value
        unsafe { max.unwrap_unchecked() }
    }

    /// Returns the maximum item of the non-empty iterator with respect to the comparison function.
    ///
    /// See also [`max_by`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that this function always returns some value, as the iterator is non-empty.
    ///
    /// [`max_by`]: Iterator::max_by
    #[must_use]
    fn max_by<F: FnMut(&Self::Item, &Self::Item) -> Ordering>(self, function: F) -> Self::Item {
        let max = self.into_iter().max_by(function);

        // SAFETY: the implementor guarantees the iterator is non-empty
        // therefore, `max` has to contain some value
        unsafe { max.unwrap_unchecked() }
    }

    /// Returns the maximum item of the non-empty iterator with respect to the key function.
    ///
    /// See also [`max_by_key`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that this function always returns some value, as the iterator is non-empty.
    ///
    /// [`max_by_key`]: Iterator::max_by_key
    #[must_use]
    fn max_by_key<K: Ord, F: FnMut(&Self::Item) -> K>(self, function: F) -> Self::Item {
        let max = self.into_iter().max_by_key(function);

        // SAFETY: the implementor guarantees the iterator is non-empty
        // therefore, `max` has to contain some value
        unsafe { max.unwrap_unchecked() }
    }

    /// Returns the minimum item of the non-empty iterator.
    ///
    /// See also [`min`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that this function always returns some value, as the iterator is non-empty.
    ///
    /// [`min`]: Iterator::min
    #[must_use]
    fn min(self) -> Self::Item
    where
        Self::Item: Ord,
    {
        let min = self.into_iter().min();

        // SAFETY: the implementor guarantees the iterator is non-empty
        // therefore, `min` has to contain some value
        unsafe { min.unwrap_unchecked() }
    }

    /// Returns the minimum item of the non-empty iterator with respect to the comparison function.
    ///
    /// See also [`min_by`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that this function always returns some value, as the iterator is non-empty.
    ///
    /// [`min_by`]: Iterator::min_by
    #[must_use]
    fn min_by<F: FnMut(&Self::Item, &Self::Item) -> Ordering>(self, function: F) -> Self::Item {
        let min = self.into_iter().min_by(function);

        // SAFETY: the implementor guarantees the iterator is non-empty
        // therefore, `min` has to contain some value
        unsafe { min.unwrap_unchecked() }
    }

    /// Returns the minimum item of the non-empty iterator with respect to the key function.
    ///
    /// See also [`min_by_key`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that this function always returns some value, as the iterator is non-empty.
    ///
    /// [`min_by_key`]: Iterator::min_by_key
    #[must_use]
    fn min_by_key<K: Ord, F: FnMut(&Self::Item) -> K>(self, function: F) -> Self::Item {
        let min = self.into_iter().min_by_key(function);

        // SAFETY: the implementor guarantees the iterator is non-empty
        // therefore, `min` has to contain some value
        unsafe { min.unwrap_unchecked() }
    }

    /// Returns the `n`-th item of the non-empty iterator.
    ///
    /// See also [`nth`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that this function expects non-zero `n`, as the first item can be obtained
    /// via [`consume`].
    ///
    /// [`nth`]: Iterator::nth
    fn nth(self, n: Size) -> Option<Self::Item> {
        self.into_iter().nth(n.get())
    }

    /// Skips the first given number of items in the non-empty iterator.
    ///
    /// See also [`skip`] on [`Iterator`].
    ///
    /// The returned iterator can be empty, depending on the count.
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that this function expects non-zero count.
    ///
    /// [`skip`]: Iterator::skip
    fn skip(self, count: Size) -> iter::Skip<Self::IntoIter> {
        self.into_iter().skip(count.get())
    }

    /// Takes only the first given number of items from the non-empty iterator.
    ///
    /// See also [`take`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that this function expects non-zero count in order to guarantee non-emptiness.
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`take`]: Iterator::take
    fn take(self, count: Size) -> Take<Self> {
        Take::new(self, count)
    }

    /// Returns the last item of the non-empty iterator.
    ///
    /// See also [`last`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that this function always returns some value, as the iterator is non-empty.
    ///
    /// [`last`]: Iterator::last
    #[must_use]
    fn last(self) -> Self::Item {
        let last = self.into_iter().last();

        // SAFETY: the implementor guarantees the iterator is non-empty
        // therefore, `last` has to contain some value
        unsafe { last.unwrap_unchecked() }
    }

    /// Steps the non-empty iterator by the given custom amount.
    ///
    /// See also [`step_by`] on [`Iterator`].
    ///
    /// # Difference from [`Iterator`]
    ///
    /// Note that this function expects non-zero step as the [`step_by`] on [`Iterator`]
    /// panics on zero.
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`step_by`]: Iterator::step_by
    fn step_by(self, step: Size) -> StepBy<Self> {
        StepBy::new(self, step)
    }

    /// Equivalent to [`for_each`] on [`Iterator`].
    ///
    /// [`for_each`]: Iterator::for_each
    fn for_each<F: FnMut(Self::Item)>(self, function: F) {
        self.into_iter().for_each(function);
    }

    /// Consumes the iterator, exhausting it by dropping all of its items.
    ///
    /// This is equivalent to calling [`for_each`] with [`drop`].
    ///
    /// [`for_each`]: NonEmptyIterator::for_each
    fn exhaust(self) {
        self.for_each(drop);
    }

    /// Equivalent to [`skip_while`] on [`Iterator`].
    ///
    /// Note that the returned iterator can be empty, depending on the predicate.
    ///
    /// [`skip_while`]: Iterator::skip_while
    fn skip_while<P: FnMut(&Self::Item) -> bool>(
        self,
        predicate: P,
    ) -> iter::SkipWhile<Self::IntoIter, P> {
        self.into_iter().skip_while(predicate)
    }

    /// Equivalent to [`take_while`] on [`Iterator`].
    ///
    /// Note that the returned iterator can be empty, depending on the predicate.
    ///
    /// [`take_while`]: Iterator::take_while
    fn take_while<P: FnMut(&Self::Item) -> bool>(
        self,
        predicate: P,
    ) -> iter::TakeWhile<Self::IntoIter, P> {
        self.into_iter().take_while(predicate)
    }

    /// Equivalent to [`map_while`] on [`Iterator`].
    ///
    /// Note that the returned iterator can be empty, depending on the predicate.
    ///
    /// [`map_while`]: Iterator::map_while
    fn map_while<T, P: FnMut(Self::Item) -> Option<T>>(
        self,
        predicate: P,
    ) -> iter::MapWhile<Self::IntoIter, P> {
        self.into_iter().map_while(predicate)
    }

    /// Equivalent to [`scan`] on [`Iterator`].
    ///
    /// Note that the returned iterator can be empty, depending on the function.
    ///
    /// [`scan`]: Iterator::scan
    fn scan<S, T, F: FnMut(&mut S, Self::Item) -> Option<T>>(
        self,
        initial: S,
        function: F,
    ) -> iter::Scan<Self::IntoIter, S, F> {
        self.into_iter().scan(initial, function)
    }

    /// Creates non-empty iterators that call the provided function with references to each item.
    ///
    /// See also [`inspect`] on [`Iterator`].
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`inspect`]: Iterator::inspect
    fn inspect<F: FnMut(&Self::Item)>(self, function: F) -> Inspect<Self, F> {
        Inspect::new(self, function)
    }

    /// Equivalent to [`partition`] on [`Iterator`].
    ///
    /// [`partition`]: Iterator::partition
    fn partition<C: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool>(
        self,
        function: F,
    ) -> (C, C) {
        self.into_iter().partition(function)
    }

    /// Equivalent to [`position`] on [`Iterator`].
    ///
    /// [`position`]: Iterator::position
    fn position<P: FnMut(Self::Item) -> bool>(self, predicate: P) -> Option<usize> {
        self.into_iter().position(predicate)
    }

    /// Equivalent to [`cmp`] on [`Iterator`].
    ///
    /// [`cmp`]: Iterator::cmp
    fn cmp<I: IntoIterator<Item = Self::Item>>(self, other: I) -> Ordering
    where
        Self::Item: Ord,
    {
        self.into_iter().cmp(other)
    }

    /// Equivalent to [`partial_cmp`] on [`Iterator`].
    ///
    /// [`partial_cmp`]: Iterator::partial_cmp
    fn partial_cmp<I: IntoIterator>(self, other: I) -> Option<Ordering>
    where
        Self::Item: PartialOrd<I::Item>,
    {
        self.into_iter().partial_cmp(other)
    }

    /// Equivalent to [`eq`] on [`Iterator`].
    ///
    /// [`eq`]: Iterator::eq
    fn eq<I: IntoIterator>(self, other: I) -> bool
    where
        Self::Item: PartialEq<I::Item>,
    {
        self.into_iter().eq(other)
    }

    /// Equivalent to [`ne`] on [`Iterator`].
    ///
    /// [`ne`]: Iterator::ne
    fn ne<I: IntoIterator>(self, other: I) -> bool
    where
        Self::Item: PartialEq<I::Item>,
    {
        self.into_iter().ne(other)
    }

    /// Equivalent to [`lt`] on [`Iterator`].
    ///
    /// [`lt`]: Iterator::lt
    fn lt<I: IntoIterator>(self, other: I) -> bool
    where
        Self::Item: PartialOrd<I::Item>,
    {
        self.into_iter().lt(other)
    }

    /// Equivalent to [`le`] on [`Iterator`].
    ///
    /// [`le`]: Iterator::le
    fn le<I: IntoIterator>(self, other: I) -> bool
    where
        Self::Item: PartialOrd<I::Item>,
    {
        self.into_iter().le(other)
    }

    /// Equivalent to [`gt`] on [`Iterator`].
    ///
    /// [`gt`]: Iterator::gt
    fn gt<I: IntoIterator>(self, other: I) -> bool
    where
        Self::Item: PartialOrd<I::Item>,
    {
        self.into_iter().gt(other)
    }

    /// Equivalent to [`ge`] on [`Iterator`].
    fn ge<I: IntoIterator>(self, other: I) -> bool
    where
        Self::Item: PartialOrd<I::Item>,
    {
        self.into_iter().ge(other)
    }

    /// Equivalent to [`is_sorted`] on [`Iterator`].
    ///
    /// [`is_sorted`]: Iterator::is_sorted
    #[allow(clippy::wrong_self_convention)]
    fn is_sorted(self) -> bool
    where
        Self::Item: PartialOrd,
    {
        self.into_iter().is_sorted()
    }

    /// Equivalent to [`is_sorted_by`] on [`Iterator`].
    ///
    /// [`is_sorted_by`]: Iterator::is_sorted_by
    #[allow(clippy::wrong_self_convention)]
    fn is_sorted_by<F: FnMut(&Self::Item, &Self::Item) -> bool>(self, function: F) -> bool {
        self.into_iter().is_sorted_by(function)
    }

    /// Equivalent to [`is_sorted_by_key`] on [`Iterator`].
    ///
    /// [`is_sorted_by_key`]: Iterator::is_sorted_by_key
    #[allow(clippy::wrong_self_convention)]
    fn is_sorted_by_key<K: PartialOrd, F: FnMut(Self::Item) -> K>(self, function: F) -> bool {
        self.into_iter().is_sorted_by_key(function)
    }

    /// Similar to [`collect`], but extends the provided collection instead of creating new ones.
    ///
    /// Returns the provided collection after extending.
    ///
    /// [`collect`]: NonEmptyIterator::collect
    fn collect_into<C: Extend<Self::Item>>(self, collection: &mut C) -> &mut C {
        collection.extend(self);

        collection
    }

    /// Equivalent to [`find_map`] on [`Iterator`].
    ///
    /// [`find_map`]: Iterator::find_map
    fn find_map<T, F: FnMut(Self::Item) -> Option<T>>(self, function: F) -> Option<T> {
        self.into_iter().find_map(function)
    }

    /// Fuses the non-empty iterator, ensuring that once it returns [`None`],
    /// it will return [`None`] forever afterwards.
    ///
    /// See also [`fuse`] on [`Iterator`].
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`fuse`]: Iterator::fuse
    fn fuse(self) -> Fuse<Self> {
        Fuse::new(self)
    }

    /// Reverses the iteration in non-empty iterators.
    ///
    /// See also [`rev`] on [`Iterator`].
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`rev`]: Iterator::rev
    fn rev(self) -> Rev<Self>
    where
        Self::IntoIter: DoubleEndedIterator,
    {
        Rev::new(self)
    }

    /// Repeats the non-empty iterator endlessly.
    ///
    /// See also [`cycle`] on [`Iterator`].
    ///
    /// # Non-empty
    ///
    /// The returned iterator is guaranteed to be non-empty.
    ///
    /// [`cycle`]: Iterator::cycle
    fn cycle(self) -> Cycle<Self>
    where
        Self::IntoIter: Clone,
    {
        Cycle::new(self)
    }
}

/// Represents types that can be created from non-empty iterators.
///
/// This is similar to [`FromIterator`], but specifically for non-empty iterators,
/// though anything that is [`FromIterator`] is also [`FromNonEmptyIterator`].
pub trait FromNonEmptyIterator<T>: Sized {
    /// Creates [`Self`] from the provided non-empty iterator.
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = T>>(iterable: I) -> Self;
}

impl<T, C: FromIterator<T>> FromNonEmptyIterator<T> for C {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = T>>(iterable: I) -> Self {
        iterable.into_non_empty_iter().into_iter().collect()
    }
}

/// Represents types that can be converted into non-empty iterators.
///
/// This is similar to [`IntoIterator`], but specifically for non-empty iterators.
///
/// Any [`NonEmptyIterator`] is also [`IntoNonEmptyIterator`], akin to how any [`Iterator`]
/// is also [`IntoIterator`].
pub trait IntoNonEmptyIterator: IntoIterator {
    /// What kind of [`NonEmptyIterator`] are we turning this into?
    type IntoNonEmptyIter: NonEmptyIterator<Item = Self::Item>;

    /// Converts `self` into [`NonEmptyIterator`].
    fn into_non_empty_iter(self) -> Self::IntoNonEmptyIter;
}

impl<I: NonEmptyIterator> IntoNonEmptyIterator for I {
    type IntoNonEmptyIter = Self;

    fn into_non_empty_iter(self) -> Self::IntoNonEmptyIter {
        self
    }
}

mod sealed {
    pub trait Sealed {}
}

/// Convenience trait implemented for any type that is [`IntoIterator`],
/// allowing to convert iterables into non-empty ones.
pub trait TryIntoNonEmptyIterator: sealed::Sealed {
    /// The type of the items being iterated over.
    type Item;

    /// Which kind of [`NonEmptyIterator`] are we turning this into?
    type IntoNonEmptyIter: NonEmptyIterator<Item = Self::Item>;

    /// Tries to convert `self` into [`NonEmptyIterator`].
    ///
    /// Returns [`None`] if `self` is empty and therefore can not be converted.
    fn try_into_non_empty_iter(self) -> Option<Self::IntoNonEmptyIter>;
}

impl<I: IntoIterator> sealed::Sealed for I {}

impl<I: IntoIterator> TryIntoNonEmptyIterator for I {
    type Item = I::Item;

    type IntoNonEmptyIter = NonEmptyAdapter<iter::Peekable<I::IntoIter>>;

    fn try_into_non_empty_iter(self) -> Option<Self::IntoNonEmptyIter> {
        let mut peekable = self.into_iter().peekable();

        peekable.peek()?;

        // SAFETY: `peekable` is non-empty if we reached here
        Some(unsafe { NonEmptyAdapter::new(peekable) })
    }
}
