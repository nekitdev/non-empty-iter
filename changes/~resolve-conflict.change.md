Resolve possible conflicting implementations of `FromNonEmptyIterator<T>`
by removing the generic implementation for all `C: FromIterator<T>` types.
