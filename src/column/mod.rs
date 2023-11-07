use crate::HashTable;
use std::{borrow::Borrow, hash::Hash};

#[derive(Debug)]
pub struct HashTableColumn<'t, 'k, K, Q, V> {
    pub(crate) parent_table: &'t HashTable<K, V>,
    pub(crate) column: &'k Q,
}

impl<K, Q, V> Clone for HashTableColumn<'_, '_, K, Q, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K, Q, V> Copy for HashTableColumn<'_, '_, K, Q, V> {}

impl<'t, 'k, K, Q, V> HashTableColumn<'t, 'k, K, Q, V> {
    pub fn column_key(&self) -> &'k Q {
        self.column
    }
}

impl<'t, 'k, K, Q, V> HashTableColumn<'t, 'k, K, Q, V>
where
    K: Hash + Eq,
    K: Borrow<Q>,
    Q: Hash + Eq,
{
    pub fn get(&self, row: usize) -> Option<&'t V> {
        self.parent_table.get(self.column, row)
    }
}

#[derive(Debug)]
pub struct ColumnIter<'t, 'k, K, Q, V> {
    column: HashTableColumn<'t, 'k, K, Q, V>,
    row_idx: usize,
}

impl<K, Q, V> Clone for ColumnIter<'_, '_, K, Q, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K, Q, V> Copy for ColumnIter<'_, '_, K, Q, V> {}

impl<'t, 'k, K, Q, V> Iterator for ColumnIter<'t, 'k, K, Q, V>
where
    K: Hash + Eq,
    K: Borrow<Q>,
    Q: Hash + Eq,
{
    type Item = &'t V;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.column.get(self.row_idx);
        self.row_idx += 1;
        val
    }
}

impl<'t, 'k, K, Q, V> IntoIterator for HashTableColumn<'t, 'k, K, Q, V>
where
    K: Hash + Eq,
    K: Borrow<Q>,
    Q: Hash + Eq,
{
    type Item = &'t V;
    type IntoIter = ColumnIter<'t, 'k, K, Q, V>;

    fn into_iter(self) -> Self::IntoIter {
        ColumnIter {
            column: self,
            row_idx: 0,
        }
    }
}
