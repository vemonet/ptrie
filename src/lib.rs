#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

#[doc(hidden)]
pub mod error;
#[doc(hidden)]
pub mod trie;
#[doc(hidden)]
pub mod trie_node;

pub use error::TrieError;

#[doc(inline)]
pub use trie::{Trie, TrieIterator};
