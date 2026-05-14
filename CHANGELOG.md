# Changelog

<!-- changelogging: start -->

## [0.3.1](https://github.com/nekitdev/non-empty-iter/tree/v0.3.1) (2026-05-14)

### Internal

- Update `doc_auto_cfg -> doc_cfg` for [`docs.rs`](https://docs.rs/).

## [0.3.0](https://github.com/nekitdev/non-empty-iter/tree/v0.3.0) (2026-05-13)

### Changes

- `rust-version` is now set to `1.95` (latest stable).

## [0.2.0](https://github.com/nekitdev/non-empty-iter/tree/v0.2.0) (2025-09-21)

### Changes

- Resolve possible conflicting implementations of `FromNonEmptyIterator<T>`
  by removing the generic implementation for all `C: FromIterator<T>` types.

## [0.1.0](https://github.com/nekitdev/non-empty-iter/tree/v0.1.0) (2025-09-21)

Initial release.
