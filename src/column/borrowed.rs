//! Borrowed column access

use std::ops::Deref;

/// Borrowed view into a table's column
#[derive(Debug)]
pub struct HashTableColumnBorrowed<'t, 'k, Q, V> {
    pub(crate) column: &'k Q,
    pub(crate) values: Vec<&'t V>,
}

impl<Q, V> Clone for HashTableColumnBorrowed<'_, '_, Q, V> {
    fn clone(&self) -> Self {
        Self {
            column: self.column,
            values: self.values.clone(),
        }
    }
}

impl<'t, 'k, Q, V> HashTableColumnBorrowed<'t, 'k, Q, V> {
    /// Get the key of the table column
    pub fn column_key(&self) -> &'k Q {
        self.column
    }
}

impl<'t, Q, V> Deref for HashTableColumnBorrowed<'t, '_, Q, V> {
    type Target = Vec<&'t V>;

    /// This [`Deref`] implementation allows using this column as a regular [`Vec`]
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'t, Q, V> IntoIterator for HashTableColumnBorrowed<'t, '_, Q, V> {
    type Item = &'t V;
    type IntoIter = BorrowedColumnIter<'t, V>;

    fn into_iter(self) -> Self::IntoIter {
        BorrowedColumnIter {
            inner: self.values.into_iter(),
        }
    }
}

/// An iterator over borrowed values of a table column.
///
/// Returned by [`HashTableColumnBorrowed::into_iter`]
#[derive(Debug)]
pub struct BorrowedColumnIter<'t, V> {
    inner: <Vec<&'t V> as IntoIterator>::IntoIter,
}

impl<'t, V> Iterator for BorrowedColumnIter<'t, V> {
    type Item = &'t V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
