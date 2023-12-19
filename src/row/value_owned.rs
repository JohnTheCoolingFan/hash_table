use std::borrow::Borrow;

use crate::typedefs::*;

/// `HashTable` row that takes ownership over the row's values. If you want the keys to be owned too,
/// use the `Into::into` implementation to convert to a `HashMap<OwnedK, V>`
#[derive(Debug, Clone)]
pub struct HashTableRowValueOwned<'t, K, V> {
    pub(crate) parent_indices_table: &'t HashMap<K, usize>,
    pub(crate) values: Vec<V>,
}

impl<'t, K, V> HashTableRowValueOwned<'t, K, V> {
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        K: Hash + Eq,
        Q: Hash + Eq + ?Sized,
    {
        let idx = self.parent_indices_table.get(key)?;
        self.values.get(*idx)
    }
}

impl<'t, K, V> IntoIterator for HashTableRowValueOwned<'t, K, V>
where
    K: Hash + Eq,
{
    type Item = (&'t K, V);
    type IntoIter = HashTableRowValueOwnedIntoIter<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

#[derive(Debug)]
pub struct HashTableRowValueOwnedIntoIter<'t, K, V> {
    values: Vec<Option<V>>,
    indices_table_iter: <&'t HashMap<K, usize> as IntoIterator>::IntoIter,
}

impl<'t, K, V> Iterator for HashTableRowValueOwnedIntoIter<'t, K, V> {
    type Item = (&'t K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.indices_table_iter.next().map(|(k, i)| {
            (
                k,
                self.values[*i]
                    .take()
                    .expect("Each index is only used once"),
            )
        })
    }
}
