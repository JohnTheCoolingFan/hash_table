use std::{hash::Hash, marker::PhantomData};

use super::directions::*;
use crate::{column::owned::HashTableColumnOwned, HashMap, HashTable};

#[derive(Debug)]
pub struct TableOwnedIterWrapper<K, V, D> {
    pub table: HashTable<K, V>,
    dir_phantom: PhantomData<D>,
}

impl<K, V, D> TableOwnedIterWrapper<K, V, D>
where
    D: IterDirection,
{
    pub fn new(table: HashTable<K, V>) -> Self {
        Self {
            table,
            dir_phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct TableRowWiseOwnedIter<K, V> {
    table: HashTable<K, V>,
}

impl<K, V> Iterator for TableRowWiseOwnedIter<K, V>
where
    K: Clone + Hash + Eq,
{
    type Item = HashMap<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.table.remove_row(0).map(Into::into)
    }
}

#[derive(Debug)]
pub struct TableColumnWiseOwnedIter<K, V> {
    table: HashTable<K, V>,
}

impl<K, V> Iterator for TableColumnWiseOwnedIter<K, V>
where
    K: Hash + Eq + Clone,
{
    type Item = HashTableColumnOwned<K, V>;

    /// Iteration order depends on what column key will be returned first by the underlying hashmap
    fn next(&mut self) -> Option<Self::Item> {
        let key = self.table.indices_table.keys().next()?.clone();
        self.table.remove_column(&key)
    }
}

#[derive(Debug)]
pub struct TableElementWiseReverseOwnedIter<K, V> {
    table: HashTable<K, V>,
}

impl<K, V> Iterator for TableElementWiseReverseOwnedIter<K, V>
where
    K: Hash + Eq + Clone,
{
    type Item = ((K, usize), V);

    /// This implementation goes in reverse order. Last row to first, last key to first.
    fn next(&mut self) -> Option<Self::Item> {
        if self.table.values_vector.is_empty() {
            None
        } else {
            let columns = self.table.columns_len();
            let row_idx = if self.table.values_vector.len() == columns * self.table.rows_len() {
                self.table.rows_len() - 1
            } else {
                self.table.rows_len()
            };
            let val = self.table.values_vector.pop()?;
            let col_idx = self.table.values_vector.len() % columns;
            let col_key = self
                .table
                .indices_table
                .iter()
                .find_map(|(k, i)| (*i == col_idx).then_some(k.clone()))?;
            Some(((col_key, row_idx), val))
        }
    }
}

impl<K, V> IntoIterator for TableOwnedIterWrapper<K, V, Row>
where
    K: Clone + Hash + Eq,
{
    type Item = HashMap<K, V>;
    type IntoIter = TableRowWiseOwnedIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        TableRowWiseOwnedIter { table: self.table }
    }
}

impl<K, V> IntoIterator for TableOwnedIterWrapper<K, V, Column>
where
    K: Hash + Eq + Clone,
{
    type Item = HashTableColumnOwned<K, V>;
    type IntoIter = TableColumnWiseOwnedIter<K, V>;

    /// Iteration order depends on what column key will be returned first by the underlying hashmap
    fn into_iter(self) -> Self::IntoIter {
        TableColumnWiseOwnedIter { table: self.table }
    }
}

impl<K, V> IntoIterator for TableOwnedIterWrapper<K, V, ElementsReverse>
where
    K: Clone + Hash + Eq,
{
    type Item = ((K, usize), V);
    type IntoIter = TableElementWiseReverseOwnedIter<K, V>;

    /// This implementation goes in reverse order. Last row to first, last key to first.
    fn into_iter(self) -> Self::IntoIter {
        TableElementWiseReverseOwnedIter { table: self.table }
    }
}
