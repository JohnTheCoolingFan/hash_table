#![doc = include_str!("../README.md")]

#[cfg(all(
    feature = "serde",
    feature = "hashbrown",
    not(feature = "hashbrown-serde")
))]
compile_error!("Due to how rust features work, you need to enable the `hashbrown-serde` feature to use both hashbrown and serde");

pub mod column;
pub mod row;
pub mod table;
#[cfg(test)]
mod tests;
pub mod typedefs;
pub use table::HashTable;
#[doc(hidden)]
pub use typedefs::*;
