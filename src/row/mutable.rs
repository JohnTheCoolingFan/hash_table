//! Mutable borrow column access

use std::borrow::Borrow;

use crate::*;

/// Row of a table with mutable access to the values
#[derive(Debug)]
pub struct HashTableMutableBorrowedRow<'t, K, V> {
    pub(crate) indices_table: &'t HashMap<K, usize>,
    pub(crate) values: &'t mut [V],
}

impl<'t, 'r: 't, K, V> HashTableMutableBorrowedRow<'t, K, V> {
    /// Get an element of this row in the requested `column`.
    pub fn get<Q>(&'r mut self, column: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        K: Hash + Eq,
        Q: Hash + Eq + ?Sized,
    {
        self.indices_table.get(column).map(|i| &mut self.values[*i])
    }
}

impl<'t, K, V> IntoIterator for HashTableMutableBorrowedRow<'t, K, V> {
    type Item = (&'t K, &'t mut V);
    type IntoIter = HashTableMutableBorrowedRowIntoIter<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        HashTableMutableBorrowedRowIntoIter {
            indices_table_iter: self.indices_table.iter(),
            values: self.values.iter_mut().map(Option::Some).collect(),
        }
    }
}

/// Iterator over mutable references to values of a table row.
///
/// Returned by [`HashTableMutableBorrowedRow::into_iter`].
#[derive(Debug)]
pub struct HashTableMutableBorrowedRowIntoIter<'t, K, V> {
    indices_table_iter: <&'t HashMap<K, usize> as IntoIterator>::IntoIter,
    values: Vec<Option<&'t mut V>>,
}

impl<'t, K, V> Iterator for HashTableMutableBorrowedRowIntoIter<'t, K, V> {
    type Item = (&'t K, &'t mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.indices_table_iter
            .next()
            .map(|(k, i)| (k, self.values[*i].take().expect("Indexes do not repeat")))
    }
}
