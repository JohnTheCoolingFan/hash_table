//! Re-exports of `std` or `hashbrown` types for easier usage throughout the library

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "hashbrown")] {
        pub use hashbrown::HashMap;
        pub use hashbrown::hash_map::Keys;
    } else {
        pub use std::collections::HashMap;
        pub use std::collections::hash_map::Keys;
    }
}

pub use std::hash::Hash;
