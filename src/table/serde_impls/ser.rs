use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize, Serializer,
};

use crate::{row::borrowed::HashTableRowBorrowed, HashTable};

impl<'t, K, V> Serialize for HashTableRowBorrowed<'t, K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_map(Some(self.columns_len()))?;
        for (k, v) in *self {
            state.serialize_entry(k, v)?;
        }
        state.end()
    }
}

impl<K, V> Serialize for HashTable<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_seq(Some(self.rows_len()))?;
        for row in self {
            state.serialize_element(&row)?;
        }
        state.end()
    }
}

/// A function to use in `#[serde(serialize_with = "...")]`
///
/// Serializes the table as a map of column keys to column values
pub fn serialize_hashtable_as_map<S, K, V>(
    table: &HashTable<K, V>,
    ser: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    K: Serialize,
    V: Serialize,
{
    let mut state = ser.serialize_map(Some(table.columns_len()))?;

    for column in table.iter_columns() {
        state.serialize_entry(column.column_key(), &column.values)?;
    }

    state.end()
}
