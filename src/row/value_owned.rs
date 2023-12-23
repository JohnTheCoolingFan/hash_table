//! Value-owned row access

use std::{borrow::Borrow, iter::FusedIterator};

use crate::typedefs::*;

/// `HashTable` row that takes ownership over the row's values.
///
/// If you want the keys to be owned too, you can do so by iterating over this row and cloning the
/// values and then collecting into a hashmap, which is done in the implkementation of the [`From`]
/// trait
#[derive(Debug)]
pub struct HashTableRowValueOwned<'t, K, V> {
    pub(crate) parent_indices_table: &'t HashMap<K, usize>,
    pub(crate) values: Vec<V>,
}

impl<'t, K, V: Clone> Clone for HashTableRowValueOwned<'t, K, V> {
    fn clone(&self) -> Self {
        Self {
            parent_indices_table: self.parent_indices_table,
            values: self.values.clone(),
        }
    }
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

impl<'t, K, V> IntoIterator for HashTableRowValueOwned<'t, K, V> {
    type Item = (&'t K, V);
    type IntoIter = HashTableRowValueOwnedIntoIter<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        HashTableRowValueOwnedIntoIter {
            values: self.values.into_iter().map(Option::Some).collect(),
            indices_table_iter: self.parent_indices_table.iter(),
        }
    }
}

#[derive(Debug)]
pub struct HashTableRowValueOwnedIntoIter<'t, K, V> {
    values: Vec<Option<V>>,
    indices_table_iter: <&'t HashMap<K, usize> as IntoIterator>::IntoIter,
}

impl<'t, K, V: Clone> Clone for HashTableRowValueOwnedIntoIter<'t, K, V> {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            indices_table_iter: self.indices_table_iter.clone(),
        }
    }
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices_table_iter.size_hint()
    }
}

impl<'t, K, V> FusedIterator for HashTableRowValueOwnedIntoIter<'t, K, V> {}

impl<'t, K, V> ExactSizeIterator for HashTableRowValueOwnedIntoIter<'t, K, V> {
    fn len(&self) -> usize {
        self.indices_table_iter.len()
    }
}

impl<'t, K, V, OwnedK> From<HashTableRowValueOwned<'t, K, V>> for HashMap<OwnedK, V>
where
    K: ToOwned<Owned = OwnedK>,
    OwnedK: Hash + Eq,
{
    fn from(row: HashTableRowValueOwned<'t, K, V>) -> Self {
        row.into_iter().map(|(k, v)| (k.to_owned(), v)).collect()
    }
}
