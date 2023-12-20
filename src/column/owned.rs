use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct HashTableColumnOwned<K, V> {
    pub(crate) key: K,
    pub(crate) values: Vec<V>,
}

impl<K, V> HashTableColumnOwned<K, V> {
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn into_values(self) -> Vec<V> {
        self.values
    }

    pub fn into_key(self) -> K {
        self.key
    }

    pub fn into_pair(self) -> (K, Vec<V>) {
        (self.key, self.values)
    }
}

impl<K, V> Deref for HashTableColumnOwned<K, V> {
    type Target = Vec<V>;

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
