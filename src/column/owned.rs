//! Owned column access

use std::ops::{Deref, DerefMut};

/// A column of a table.
///
/// Takes ownership over the key of the column and its values
#[derive(Debug)]
pub struct HashTableColumnOwned<K, V> {
    pub(crate) key: K,
    pub(crate) values: Vec<V>,
}

impl<K, V> HashTableColumnOwned<K, V> {
    /// Get the key of the column
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Take the values of the column and drop the key
    pub fn into_values(self) -> Vec<V> {
        self.values
    }

    /// Take the key of the column and drop the values
    pub fn into_key(self) -> K {
        self.key
    }

    /// Take the inner key and values
    pub fn into_pair(self) -> (K, Vec<V>) {
        (self.key, self.values)
    }
}

impl<K, V, VV> From<(K, VV)> for HashTableColumnOwned<K, V>
where
    VV: Into<Vec<V>>,
{
    fn from(value: (K, VV)) -> Self {
        Self {
            key: value.0,
            values: value.1.into(),
        }
    }
}

impl<K, V> Deref for HashTableColumnOwned<K, V> {
    type Target = Vec<V>;

    /// This [`Deref`] implementation allows using this type as a regular [`Vec`]
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<K, V> DerefMut for HashTableColumnOwned<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

impl<K, V> IntoIterator for HashTableColumnOwned<K, V> {
    type Item = <Vec<V> as IntoIterator>::Item;
    type IntoIter = <Vec<V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
