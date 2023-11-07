use std::{borrow::Borrow, hash::Hash};

use crate::{HashTable, Keys};

#[derive(Debug)]
pub struct HashTableRowBorrowed<'a, K, V> {
    pub(crate) parent_table: &'a HashTable<K, V>,
    pub(crate) row_idx: usize,
}

impl<K, V> Clone for HashTableRowBorrowed<'_, K, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K, V> Copy for HashTableRowBorrowed<'_, K, V> {}

impl<'a, K, V> HashTableRowBorrowed<'a, K, V>
where
    K: Hash + Eq,
{
    pub fn get<Q>(&self, column: &Q) -> Option<&'a V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.parent_table.get(column, self.row_idx)
    }

    pub fn columns_len(&self) -> Keys<'a, K, usize> {
        self.parent_table.indices_table.keys()
    }
}

#[derive(Debug)]
pub struct BorrowedRowIter<'a, K, V> {
    row: HashTableRowBorrowed<'a, K, V>,
    columns_iter: Keys<'a, K, usize>,
}

impl<'a, K, V> Iterator for BorrowedRowIter<'a, K, V>
where
    K: Hash + Eq,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let key = self.columns_iter.next()?;
        let val = self.row.get(key)?;
        Some((key, val))
    }
}

impl<'a, K, V> IntoIterator for HashTableRowBorrowed<'a, K, V>
where
    K: Hash + Eq,
{
    type Item = (&'a K, &'a V);
    type IntoIter = BorrowedRowIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        let keys_iter = self.columns_len();
        BorrowedRowIter {
            row: self,
            columns_iter: keys_iter,
        }
    }
}
