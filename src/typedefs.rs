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
