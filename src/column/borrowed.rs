use std::borrow::Borrow;

use crate::*;

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
    pub fn column_key(&self) -> &'k Q {
        self.column
    }
}

impl<'t, 'k, Q, V> HashTableColumnBorrowed<'t, 'k, Q, V> {
    pub fn get(&self, row: usize) -> Option<&'t V> {
        self.values.get(row).copied()
    }
}

impl<'t, 'k, Q, V> IntoIterator for HashTableColumnBorrowed<'t, 'k, Q, V> {
    type Item = &'t V;
    type IntoIter = BorrowedColumnIter<'t, 'k, Q, V>;

    fn into_iter(self) -> Self::IntoIter {
        BorrowedColumnIter {
            column: self,
            row_idx: 0,
        }
    }
}

#[derive(Debug)]
pub struct BorrowedColumnIter<'t, 'k, Q, V> {
    column: HashTableColumnBorrowed<'t, 'k, Q, V>,
    row_idx: usize,
}

impl<'t, 'k, Q, V> Iterator for BorrowedColumnIter<'t, 'k, Q, V> {
    type Item = &'t V;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.column.get(self.row_idx);
        self.row_idx += 1;
        val
    }
}
