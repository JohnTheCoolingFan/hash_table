//! Borrowed row access

use std::{borrow::Borrow, iter::FusedIterator};

use crate::*;

/// A row of a hash table that gives a borrowed access to its values
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
    /// Get an element of the row in the requested `column`
    pub fn get<Q>(&self, column: &Q) -> Option<&'t V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.indices_table.get(column).map(|i| &self.row_values[*i])
    }

    /// Return an iterator over the keys of the columns of the table
    pub fn columns_keys(&self) -> Keys<'t, K, usize> {
        self.indices_table.keys()
    }

    /// Return an amount of columns in the row
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

/// Iterator over a row of a borrowed table
///
/// Returned by [`HashTableRowBorrowed::into_iter`]
#[derive(Debug)]
pub struct BorrowedRowIter<'t, K, V> {
    columns_iter: <&'t HashMap<K, usize> as IntoIterator>::IntoIter,
    values: &'t [V],
}

impl<'t, K, V> Clone for BorrowedRowIter<'t, K, V> {
    fn clone(&self) -> Self {
        Self {
            columns_iter: self.columns_iter.clone(),
            values: self.values,
        }
    }
}

impl<'t, K, V> FusedIterator for BorrowedRowIter<'t, K, V> {}

impl<'t, K, V> ExactSizeIterator for BorrowedRowIter<'t, K, V> {
    fn len(&self) -> usize {
        self.columns_iter.len()
    }
}

impl<'t, K, V> Iterator for BorrowedRowIter<'t, K, V> {
    type Item = (&'t K, &'t V);

    fn next(&mut self) -> Option<Self::Item> {
        let (key, idx) = self.columns_iter.next()?;
        let val = &self.values[*idx];
        Some((key, val))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.columns_iter.size_hint()
    }
}
