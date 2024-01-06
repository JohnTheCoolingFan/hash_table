//! # Hash Table (data structure)
//!
//! This crate provides a 2d data structure where columns are indexed by hashable keys and rows are indexed using an unsigned integer (`usize`).
//!
//! ## Features
//! | Name        | Description                             | Enabled by default? |
//! |-------------|-----------------------------------------|---------------------|
//! | `hashbrown` | Uses `hashbrown` instead of std hashmap | No                  |
//! | `serde`     | Serde trait implementations             | Yes                 |

pub mod column;
pub mod row;
pub mod table;
#[cfg(test)]
mod tests;
pub mod typedefs;
pub use table::HashTable;
#[doc(hidden)]
pub use typedefs::*;
