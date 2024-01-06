#![doc = include_str!("../README.md")]

pub mod column;
pub mod row;
pub mod table;
#[cfg(test)]
mod tests;
pub mod typedefs;
pub use table::HashTable;
#[doc(hidden)]
pub use typedefs::*;
