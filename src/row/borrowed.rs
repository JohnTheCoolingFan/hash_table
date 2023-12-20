use std::borrow::Borrow;

use crate::*;

#[derive(Debug)]
pub struct HashTableRowBorrowed<'t, K, V> {
    pub(crate) indices_table: &'t HashMap<K, usize>,
    pub(crate) row_values: &'t [V],
}

impl<K, V> Clone for HashTableRowBorrowed<'_, K, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K, V> Copy for HashTableRowBorrowed<'_, K, V> {}

impl<'t, K, V> HashTableRowBorrowed<'t, K, V>
where
    K: Hash + Eq,
{
    pub fn get<Q>(&self, column: &Q) -> Option<&'t V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.indices_table.get(column).map(|i| &self.row_values[*i])
    }

    pub fn columns_keys(&self) -> Keys<'t, K, usize> {
        self.indices_table.keys()
    }

    pub fn columns_len(&self) -> usize {
        self.indices_table.len()
    }
}

impl<'t, K, V> IntoIterator for HashTableRowBorrowed<'t, K, V>
where
    K: Hash + Eq,
{
    type Item = (&'t K, &'t V);
    type IntoIter = BorrowedRowIter<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        BorrowedRowIter {
            columns_iter: self.indices_table.iter(),
            values: self.row_values,
        }
    }
}

#[derive(Debug)]
pub struct BorrowedRowIter<'t, K, V> {
    columns_iter: <&'t HashMap<K, usize> as IntoIterator>::IntoIter,
    values: &'t [V],
}

impl<'t, K, V> Iterator for BorrowedRowIter<'t, K, V>
where
    K: Hash + Eq,
{
    type Item = (&'t K, &'t V);

    fn next(&mut self) -> Option<Self::Item> {
        let (key, idx) = self.columns_iter.next()?;
        let val = &self.values[*idx];
        Some((key, val))
    }
}
