pub mod directions;
pub mod owned;

use crate::*;

impl<K, V> IntoIterator for HashTable<K, V>
where
    K: Clone,
    K: Hash + Eq,
{
    type Item = HashMap<K, V>;
    type IntoIter = HashTableIntoIter<K, V>;

    /// Iterator taht takes ownership of both keys and values, cloning the keys each time and
    /// allocating a new hashmap
    fn into_iter(self) -> Self::IntoIter {
        HashTableIntoIter { inner: self }
    }
}

#[derive(Debug)]
pub struct HashTableIntoIter<K, V> {
    inner: HashTable<K, V>,
}

impl<K, V> Iterator for HashTableIntoIter<K, V>
where
    K: Clone,
    K: Hash + Eq,
{
    type Item = HashMap<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.rows_len() == 0 {
            None
        } else {
            self.inner
                .remove_row(self.inner.rows_len() - 1)
                .map(Into::into)
        }
    }
}
