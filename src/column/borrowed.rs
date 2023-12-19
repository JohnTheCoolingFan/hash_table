use std::borrow::Borrow;

use crate::*;

#[derive(Debug)]
pub struct HashTableColumnBorrowed<'t, 'k, K, Q, V> {
    pub(crate) parent_table: &'t HashTable<K, V>,
    pub(crate) column: &'k Q,
}

impl<K, Q, V> Clone for HashTableColumnBorrowed<'_, '_, K, Q, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K, Q, V> Copy for HashTableColumnBorrowed<'_, '_, K, Q, V> {}

impl<'t, 'k, K, Q, V> HashTableColumnBorrowed<'t, 'k, K, Q, V> {
    pub fn column_key(&self) -> &'k Q {
        self.column
    }
}

impl<'t, 'k, K, Q, V> HashTableColumnBorrowed<'t, 'k, K, Q, V>
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
pub struct BorrowedColumnIter<'t, 'k, K, Q, V> {
    column: HashTableColumnBorrowed<'t, 'k, K, Q, V>,
    row_idx: usize,
}

impl<K, Q, V> Clone for BorrowedColumnIter<'_, '_, K, Q, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K, Q, V> Copy for BorrowedColumnIter<'_, '_, K, Q, V> {}

impl<'t, 'k, K, Q, V> Iterator for BorrowedColumnIter<'t, 'k, K, Q, V>
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

impl<'t, 'k, K, Q, V> IntoIterator for HashTableColumnBorrowed<'t, 'k, K, Q, V>
where
    K: Hash + Eq,
    K: Borrow<Q>,
    Q: Hash + Eq,
{
    type Item = &'t V;
    type IntoIter = BorrowedColumnIter<'t, 'k, K, Q, V>;

    fn into_iter(self) -> Self::IntoIter {
        BorrowedColumnIter {
            column: self,
            row_idx: 0,
        }
    }
}
