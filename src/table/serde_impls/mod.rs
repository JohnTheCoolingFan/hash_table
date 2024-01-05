pub mod de;
pub mod ser;

/// A module to use with `#[serde(with = "...")]`
pub mod hashtable_columns_map {
    pub use super::{
        de::deserialize_hashtable_from_map as deserialize,
        ser::serialize_hashtable_as_map as serialize,
    };
}
