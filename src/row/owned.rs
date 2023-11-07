use std::{hash::Hash, ops::Deref};

use crate::typedefs::HashMap;

/// `HashTable` row that takes ownership over the row's values. If you want teh keys to be owned too,
/// use the `Into::into` implementation to convert to a `HashMap<K, V>`
#[derive(Debug)]
pub struct HashTableRowOwned<'t, K, V> {
    pub(crate) inner: HashMap<&'t K, V>,
}

impl<'t, K, OwnedK, V> From<HashTableRowOwned<'t, K, V>> for HashMap<OwnedK, V>
where
    K: ToOwned<Owned = OwnedK>,
    OwnedK: Hash + Eq,
{
    fn from(value: HashTableRowOwned<'t, K, V>) -> Self {
        value
            .inner
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v))
            .collect()
    }
}

impl<'t, K, V> IntoIterator for HashTableRowOwned<'t, K, V> {
    type Item = <HashMap<&'t K, V> as IntoIterator>::Item;
    type IntoIter = <HashMap<&'t K, V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'t, K, V> Deref for HashTableRowOwned<'t, K, V> {
    type Target = HashMap<&'t K, V>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
