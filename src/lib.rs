//! Non-empty iterators.

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod non_empty;

pub mod adapter;
pub mod chain;
pub mod cloned;
pub mod copied;
pub mod cycle;
pub mod enumerate;
pub mod flat_map;
pub mod flatten;
pub mod fuse;
pub mod inspect;
pub mod map;
pub mod once;
pub mod peeked;
pub mod repeat;
pub mod rev;
pub mod step_by;
pub mod successors;
pub mod take;
pub mod zip;

#[doc(inline)]
pub use non_empty::{
    FromNonEmptyIterator, IntoNonEmptyIterator, NonEmptyIterator, TryIntoNonEmptyIterator,
};

#[doc(inline)]
pub use adapter::NonEmptyAdapter;
#[doc(inline)]
pub use chain::{Chain, chain};
#[doc(inline)]
pub use cloned::Cloned;
#[doc(inline)]
pub use cycle::Cycle;
#[doc(inline)]
pub use enumerate::Enumerate;
#[doc(inline)]
pub use flat_map::FlatMap;
#[doc(inline)]
pub use flatten::Flatten;
#[doc(inline)]
pub use fuse::Fuse;
#[doc(inline)]
pub use inspect::Inspect;
#[doc(inline)]
pub use map::Map;
#[doc(inline)]
pub use once::{Once, OnceWith, once, once_with};
#[doc(inline)]
pub use peeked::Peeked;
#[doc(inline)]
pub use repeat::{Repeat, RepeatN, RepeatWith, repeat, repeat_n, repeat_with};
#[doc(inline)]
pub use rev::Rev;
#[doc(inline)]
pub use step_by::StepBy;
#[doc(inline)]
pub use successors::{Successors, successors};
#[doc(inline)]
pub use take::Take;
#[doc(inline)]
pub use zip::{Zip, zip};
