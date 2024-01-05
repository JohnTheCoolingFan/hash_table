use std::marker::PhantomData;

use serde::{de::Visitor, Deserialize, Deserializer};

use crate::{
    typedefs::{Hash, HashMap},
    HashTable,
};

impl<'de, K, V> Deserialize<'de> for HashTable<K, V>
where
    K: Hash + Eq,
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    /// Deserializes a [`HashTable`] from sequence of key-value maps
    ///
    /// Will fall back to [`deserialize_hashtable_from_map`] if deserializer decides to provide a
    /// map
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(HashTableVisitor(PhantomData))
    }
}

struct HashTableVisitor<K, V>(PhantomData<(K, V)>);

impl<'de, K, V> Visitor<'de> for HashTableVisitor<K, V>
where
    K: Hash + Eq,
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    type Value = HashTable<K, V>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a sequence of table rows")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut res = match seq.size_hint() {
            None => HashTable::default(),
            Some(len) => HashTable::with_capacity(0, len),
        };

        while let Some(row) = seq.next_element::<HashMap<K, V>>()? {
            res.push_row(row);
        }

        Ok(res)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        HashTableColumnVisitor(PhantomData).visit_map(map)
    }
}

/// A function to use in `#[serde(deserialize_with = "...")]`
///
/// Will fall back to the row-wise deserialization if the deserializer decides to deserialize a
/// sequence
pub fn deserialize_hashtable_from_map<'de, K, V, D>(des: D) -> Result<HashTable<K, V>, D::Error>
where
    D: Deserializer<'de>,
    K: Hash + Eq,
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    des.deserialize_map(HashTableColumnVisitor(PhantomData))
}

struct HashTableColumnVisitor<K, V>(PhantomData<(K, V)>);

impl<'de, K, V> Visitor<'de> for HashTableColumnVisitor<K, V>
where
    K: Hash + Eq,
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    type Value = HashTable<K, V>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a map of column key to sequence of values")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut res = match map.size_hint() {
            None => HashTable::default(),
            Some(len) => HashTable::with_capacity(len, 0),
        };

        while let Some((key, values)) = map.next_entry::<K, Vec<V>>()? {
            res.insert_column(key, values);
        }

        Ok(res)
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        HashTableVisitor(PhantomData).visit_seq(seq)
    }
}
