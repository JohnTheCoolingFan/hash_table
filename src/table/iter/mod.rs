//! Implementation of various ways to iterate over a hashtable

use std::iter::FusedIterator;

use crate::{
    column::{borrowed::HashTableColumnBorrowed, owned::HashTableColumnOwned},
    row::borrowed::HashTableRowBorrowed,
    *,
};

impl<K, V> IntoIterator for HashTable<K, V>
where
    K: Clone,
    K: Hash + Eq,
{
    type Item = HashMap<K, V>;
    type IntoIter = HashTableIntoIter<K, V>;

    /// Row-wise iterator that takes ownership of both keys and values, cloning the keys each time and
    /// allocating a new hashmap.
    fn into_iter(self) -> Self::IntoIter {
        HashTableIntoIter { inner: self }
    }
}

/// Row-wise iterator with ownership over the [`HashTable`]
///
/// Returned by [`HashTable::into_iter`]
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
                .map(|row| row.into_iter().map(|(k, v)| (k.clone(), v)).collect())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.inner.rows_len();
        (len, Some(len))
    }
}

impl<K, V> HashTable<K, V> {
    /// Row-wise iterator that borrows the table
    pub fn iter(&self) -> HashTableBorrowedIter<'_, K, V> {
        HashTableBorrowedIter {
            row: 0,
            table: self,
        }
    }

    /*
    pub fn iter_mut(&mut self) -> HashTableMutIter<'_, K, V> {
        HashTableMutIter {
            row: (0..self.rows_len()),
            table: self,
        }
    }
    */

    /// Column-wise iterator that takes ownership of the keys and values
    pub fn into_iter_columns(self) -> HashTableOwnedIntoIterColumn<K, V> {
        HashTableOwnedIntoIterColumn {
            row_len: self.columns_len(),
            indices_iter: self.indices_table.into_iter(),
            values: self.values_vector.into_iter().map(Option::Some).collect(),
        }
    }

    /// Column-wise iterator that borrows the values from the table
    pub fn iter_columns(&self) -> HashTableBorrowedIterColumn<'_, K, V> {
        HashTableBorrowedIterColumn {
            row_len: self.columns_len(),
            indices_iter: self.indices_table.iter(),
            values: &self.values_vector,
        }
    }
}

/// Row-wise iterator that borrows the table
///
/// Returned by [`HashTable::iter`]
#[derive(Debug)]
pub struct HashTableBorrowedIter<'t, K, V> {
    row: usize,
    table: &'t HashTable<K, V>,
}

impl<'t, K, V> Iterator for HashTableBorrowedIter<'t, K, V> {
    type Item = HashTableRowBorrowed<'t, K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.table.get_row(self.row)?;
        self.row += 1;
        Some(val)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.table.rows_len();
        let remainder = len - self.row;
        (remainder, Some(remainder))
    }
}

/*
#[derive(Debug)]
pub struct HashTableMutIter<'t, K, V> {
    row: Range<usize>,
    table: &'t mut HashTable<K, V>,
}

impl<'t, K, V> Iterator for HashTableMutIter<'t, K, V> {
    type Item = HashTableMutableBorrowedRow<'t, K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.table.get_row_mut(self.row.next()?)
    }
}
*/

/// Column-wise iterator with ownership over the keys and values of a table
///
/// Returned by [`HashTable::into_iter_columns`]
#[derive(Debug)]
pub struct HashTableOwnedIntoIterColumn<K, V> {
    indices_iter: <HashMap<K, usize> as IntoIterator>::IntoIter,
    values: Vec<Option<V>>,
    row_len: usize,
}

impl<K, V> Iterator for HashTableOwnedIntoIterColumn<K, V> {
    type Item = HashTableColumnOwned<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        let (key, idx) = self.indices_iter.next()?;
        let values = self
            .values
            .chunks_exact_mut(self.row_len)
            .map(|chunk| {
                chunk[idx]
                    .take()
                    .expect("Each column is accessed only once")
            })
            .collect();
        Some(HashTableColumnOwned { key, values })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices_iter.size_hint()
    }
}

impl<K, V> FusedIterator for HashTableOwnedIntoIterColumn<K, V> {}

impl<K, V> ExactSizeIterator for HashTableOwnedIntoIterColumn<K, V> {
    fn len(&self) -> usize {
        self.indices_iter.len()
    }
}

/// Column-wise iterator that borrows the table
///
/// Returned by [`HashTable::iter_columns`]
#[derive(Debug)]
pub struct HashTableBorrowedIterColumn<'t, K, V> {
    indices_iter: <&'t HashMap<K, usize> as IntoIterator>::IntoIter,
    values: &'t [V],
    row_len: usize,
}

impl<'t, K, V> Clone for HashTableBorrowedIterColumn<'t, K, V> {
    fn clone(&self) -> Self {
        Self {
            indices_iter: self.indices_iter.clone(),
            values: self.values,
            row_len: self.row_len,
        }
    }
}

impl<'t, K, V> Iterator for HashTableBorrowedIterColumn<'t, K, V> {
    type Item = HashTableColumnBorrowed<'t, 't, K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        let (key, idx) = self.indices_iter.next()?;
        let values = self
            .values
            .chunks_exact(self.row_len)
            .map(|chunk| &chunk[*idx])
            .collect();
        Some(HashTableColumnBorrowed {
            column: key,
            values,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices_iter.size_hint()
    }
}

impl<'t, K, V> FusedIterator for HashTableBorrowedIterColumn<'t, K, V> {}

impl<'t, K, V> ExactSizeIterator for HashTableBorrowedIterColumn<'t, K, V> {
    fn len(&self) -> usize {
        self.indices_iter.len()
    }
}
