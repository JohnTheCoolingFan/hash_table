use std::{borrow::Borrow, collections::VecDeque, hash::Hash};

use crate::{
    column::{borrowed::HashTableColumnBorrowed, owned::HashTableColumnOwned},
    row::{borrowed::HashTableRowBorrowed, owned::HashTableRowOwned},
    HashMap,
};

pub mod iter;

#[derive(Debug, Default)]
pub struct HashTable<K, V> {
    pub(crate) indices_table: HashMap<K, usize>,
    pub(crate) values_vector: Vec<V>,
}

impl<K, V> HashTable<K, V> {
    #[inline(always)]
    pub fn columns_len(&self) -> usize {
        self.indices_table.len()
    }

    pub fn rows_len(&self) -> usize {
        self.values_vector.len() / self.columns_len()
    }

    pub fn with_capacity(columns: usize, rows: usize) -> Self {
        Self {
            indices_table: HashMap::with_capacity(columns),
            values_vector: Vec::with_capacity(columns * rows),
        }
    }
}

impl<K, V> HashTable<K, V>
where
    K: Hash + Eq,
{
    pub fn with_columns(columns: impl IntoIterator<Item = K>) -> Self {
        let indices_table = Self::indices_table_from_iterator(columns);
        Self {
            indices_table,
            values_vector: Vec::new(),
        }
    }

    pub fn with_columns_and_capacity(columns: impl IntoIterator<Item = K>, rows: usize) -> Self {
        let indices_table = Self::indices_table_from_iterator(columns);
        let columns_count = indices_table.len();
        Self {
            indices_table,
            values_vector: Vec::with_capacity(columns_count * rows),
        }
    }

    fn indices_table_from_iterator(columns: impl IntoIterator<Item = K>) -> HashMap<K, usize> {
        columns.into_iter().zip(0_usize..).collect()
    }

    pub fn get<Q>(&self, column: &Q, row: usize) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let column_index = self.indices_table.get(column)?;
        self.values_vector
            .get(self.columns_len() * row + column_index)
    }

    pub fn get_row(&self, row: usize) -> Option<HashTableRowBorrowed<'_, K, V>> {
        if row < self.values_vector.len() / self.columns_len() {
            Some(HashTableRowBorrowed {
                parent_table: self,
                row_idx: row,
            })
        } else {
            None
        }
    }

    pub fn get_column<'t, 'k, Q>(
        &'t self,
        column: &'k Q,
    ) -> Option<HashTableColumnBorrowed<'t, 'k, K, Q, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        if self.indices_table.contains_key(column) {
            Some(HashTableColumnBorrowed {
                parent_table: self,
                column,
            })
        } else {
            None
        }
    }

    pub fn add_row(&mut self, row: impl IntoIterator<Item = (K, V)>) {
        let mut pairs: Vec<(K, V)> = row.into_iter().collect();
        pairs.sort_by_key(|(k, _)| self.indices_table.get(k));
        self.values_vector.extend(pairs.into_iter().map(|(_, v)| v));
    }

    pub fn remove_row(&mut self, row: usize) -> Option<HashTableRowOwned<'_, K, V>> {
        if row >= self.rows_len() {
            return None;
        }
        let row_start = row * self.rows_len();
        let row_end = row_start + self.rows_len();
        let values = self.values_vector.drain(row_start..row_end);
        let mut keys = self
            .indices_table
            .iter()
            .map(|(k, v)| (k, *v))
            .collect::<Vec<_>>();
        keys.sort_by_key(|(_, i)| *i);
        Some(HashTableRowOwned {
            inner: keys.into_iter().map(|(k, _)| k).zip(values).collect(),
        })
    }

    pub fn add_column(
        &mut self,
        column: K,
        mut values: impl FnMut(HashTableRowBorrowed<'_, K, V>) -> V,
    ) {
        let rows = self.rows_len();
        let mut additional_values = VecDeque::with_capacity(rows);
        for i in 0..rows {
            let row = self.get_row(i).unwrap();
            let value = values(row);
            additional_values.push_back(value);
        }
        let new_column_index = self.indices_table.values().max().unwrap() + 1;
        self.indices_table.insert(column, new_column_index);
        for i in 0..rows {
            let new_elem_index = i * self.columns_len() + new_column_index;
            self.values_vector
                .insert(new_elem_index, additional_values.pop_front().unwrap());
        }
    }

    pub fn remove_column<Q>(&mut self, column: &Q) -> Option<HashTableColumnOwned<K, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.indices_table.contains_key(column) {
            let rows = self.rows_len();
            let (key, column_index) = self.indices_table.remove_entry(column).unwrap();
            for v in self.indices_table.values_mut() {
                if *v > column_index {
                    *v -= 1;
                }
            }
            let mut buf = Vec::with_capacity(self.rows_len());
            for i in 0..rows {
                let index = i * self.columns_len() + column_index;
                buf.push(self.values_vector.remove(index));
            }
            Some(HashTableColumnOwned { key, values: buf })
        } else {
            None
        }
    }
}
