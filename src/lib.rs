#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod map;
mod tree;

pub use self::map::Map;
pub use self::tree::Node;
