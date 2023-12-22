//! HashTable and its associated types

use std::{borrow::Borrow, hash::Hash, ops::Deref};

use crate::{
    column::{borrowed::HashTableColumnBorrowed, owned::HashTableColumnOwned},
    row::{
        borrowed::HashTableRowBorrowed, mutable::HashTableMutableBorrowedRow,
        value_owned::HashTableRowValueOwned,
    },
    HashMap,
};

pub mod iter;

/// This data structure represents a 2-dimensional grid of values. Each element is indexed by a
/// hashable key and a row index. It's also possible to access a whole row or column of the table.
#[derive(Debug, Default, Clone)]
pub struct HashTable<K, V> {
    pub(crate) indices_table: HashMap<K, usize>,
    pub(crate) values_vector: Vec<V>,
}

impl<K, V> HashTable<K, V> {
    /// Returns the number of columns in this table.
    #[inline(always)]
    pub fn columns_len(&self) -> usize {
        self.indices_table.len()
    }

    /// Returns the number of rows in this table.
    #[inline(always)]
    pub fn rows_len(&self) -> usize {
        self.values_vector.len() / self.columns_len()
    }

    /// Create new [`HashTable`] with specified amoutn of reserved capacity.
    pub fn with_capacity(columns: usize, rows: usize) -> Self {
        Self {
            indices_table: HashMap::with_capacity(columns),
            values_vector: Vec::with_capacity(columns * rows),
        }
    }

    /// `values_vector` index of where the row starts.
    #[inline]
    fn row_start(&self, row: usize) -> usize {
        self.columns_len() * row
    }

    /// Get a row of the table.
    ///
    /// Returns None if `row` is bigger than or equal to the number of rows.
    pub fn get_row(&self, row: usize) -> Option<HashTableRowBorrowed<'_, K, V>> {
        if row >= self.rows_len() {
            None
        } else {
            let start = self.row_start(row);
            let end = start + self.columns_len();
            Some(HashTableRowBorrowed {
                indices_table: &self.indices_table,
                row_values: &self.values_vector[start..end],
            })
        }
    }

    /// Get row with mutable access.
    ///
    /// Returns None if `row` is bigger than or equal to the number of row.
    pub fn get_row_mut(&mut self, row: usize) -> Option<HashTableMutableBorrowedRow<'_, K, V>> {
        if row >= self.rows_len() {
            None
        } else {
            let start = self.columns_len() * row;
            let end = start + self.columns_len();
            Some(HashTableMutableBorrowedRow {
                indices_table: &self.indices_table,
                values: &mut self.values_vector[start..end],
            })
        }
    }

    /// Remove a row and take ownership of its values.
    ///
    /// This still borrows the hashtable immutably to allow getting the values by a key. Keys can
    /// be converted to an owned variant, usually by cloning them.
    pub fn remove_row(&mut self, row: usize) -> Option<HashTableRowValueOwned<'_, K, V>> {
        if row >= self.rows_len() {
            return None;
        }
        let row_start = row * self.rows_len();
        let row_end = row_start + self.rows_len();
        let values = self.values_vector.drain(row_start..row_end);
        Some(HashTableRowValueOwned {
            parent_indices_table: &self.indices_table,
            values: values.collect(),
        })
    }
}

impl<K, V> HashTable<K, V>
where
    K: Hash + Eq,
{
    /// Create a [`HashTable`] from iterator of column keys.
    pub fn with_columns(columns: impl IntoIterator<Item = K>) -> Self {
        let indices_table = Self::indices_table_from_iterator(columns);
        Self {
            indices_table,
            values_vector: Vec::new(),
        }
    }

    /// Create a [`HashTable`] from iterator of column keys and with allocated capacity for at
    /// least the specified amount of `rows`.
    pub fn with_columns_and_capacity(columns: impl IntoIterator<Item = K>, rows: usize) -> Self {
        let indices_table = Self::indices_table_from_iterator(columns);
        let columns_count = indices_table.len();
        Self {
            indices_table,
            values_vector: Vec::with_capacity(columns_count * rows),
        }
    }

    /// Make an indices table from an iterator.
    fn indices_table_from_iterator(columns: impl IntoIterator<Item = K>) -> HashMap<K, usize> {
        columns.into_iter().zip(0_usize..).collect()
    }

    /// Index of a column.
    #[inline]
    fn column_index<Q>(&self, column: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.indices_table.get(column).copied()
    }

    /// Index of an element.
    #[inline]
    fn elem_index<Q>(&self, column: &Q, row: usize) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.column_index(column)
            .map(|col_idx| self.row_start(row) + col_idx)
    }

    /// Get an element from the table.
    ///
    /// Will return None of the `column` does not exist in teh table or `row` is out of range.
    #[inline]
    pub fn get<Q>(&self, column: &Q, row: usize) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.values_vector.get(self.elem_index(column, row)?)
    }

    /// Get an element from the table with mutable access.
    ///
    /// Will return None of the `column` does not exist in the table or `row` is out of range.
    #[inline]
    pub fn get_mut<Q>(&mut self, column: &Q, row: usize) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let idx = self.elem_index(column, row)?;
        self.values_vector.get_mut(idx)
    }

    /// Get an immutable access to a table column.
    ///
    /// Will return None if the `column` does not exist in the table.
    #[inline]
    pub fn get_column<'t, 'k, Q>(
        &'t self,
        column: &'k Q,
    ) -> Option<HashTableColumnBorrowed<'t, 'k, Q, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.indices_table
            .get(column)
            .map(|idx| HashTableColumnBorrowed {
                column,
                values: self
                    .values_vector
                    .chunks_exact(self.columns_len())
                    .map(|chunk| &chunk[*idx])
                    .collect(),
            })
    }

    /// Add a row to the table from an iterator of key-value pairs.
    pub fn add_row<I>(&mut self, row: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut pairs: Vec<(K, V)> = row.into_iter().collect();
        pairs.sort_by_key(|(k, _)| self.indices_table.get(k));
        self.values_vector.extend(pairs.into_iter().map(|(_, v)| v));
    }

    /// Add a row to the table using a generator function that returns the value from the column
    /// key.
    pub fn add_row_with<F>(&mut self, mut row_generator: F)
    where
        F: FnMut(&K) -> V,
    {
        let mut keys = self.indices_table.iter().collect::<Vec<_>>();
        keys.sort_by_key(|(_, i)| *i);
        self.values_vector
            .extend(keys.into_iter().map(|(k, _)| row_generator(k)))
    }

    /// Add a column with values provided through an iterator.
    pub fn add_column<I>(&mut self, column: K, values: I)
    where
        I: IntoIterator<Item = V>,
    {
        let mut values = values.into_iter();
        let rows = self.rows_len();
        let new_column_index = self.columns_len();
        self.indices_table.insert(column, new_column_index);
        for i in 0..rows {
            let new_elem_index = (i + 1) * new_column_index;
            self.values_vector
                .insert(new_elem_index, values.next().unwrap())
        }
    }

    /// Add a column using a generator function that returns a value based on the values of the
    /// row.
    pub fn add_column_with<F>(&mut self, column: K, mut values: F)
    where
        F: FnMut(HashTableRowBorrowed<'_, K, V>) -> V,
    {
        let rows = self.rows_len();
        self.add_column(
            column,
            (0..rows)
                .map(|i| {
                    let row = self.get_row(i).unwrap();
                    values(row)
                })
                .collect::<Vec<_>>(),
        );
    }

    /// Remove a column from the table and take ownership of the key and values.
    ///
    /// Will return None if the `column` does not exist in the table.
    pub fn remove_column<Q>(&mut self, column: &Q) -> Option<HashTableColumnOwned<K, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if !self.indices_table.contains_key(column) {
            return None;
        }
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
    }

    /// Construct HashTable from iterator of columns
    pub fn from_column_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = HashTableColumnOwned<K, V>>,
    {
        let mut indices = HashMap::new();
        let mut result_values = Vec::new();
        let mut expected_length = None;
        for (i, col) in iter.into_iter().enumerate() {
            let (key, col_values) = col.into_pair();
            let expected_length = expected_length.get_or_insert(col_values.len());
            if col_values.len() != *expected_length {
                panic!("Column {i} doesn't have the same amopunt of elements as the first column");
            }
            indices.insert(key, i);
            for (row, val) in col_values.into_iter().enumerate() {
                result_values.insert((row + 1) * (i + 1), val);
            }
        }
        Self {
            indices_table: indices,
            values_vector: result_values,
        }
    }
}

impl<K, V, R> FromIterator<R> for HashTable<K, V>
where
    K: Hash + Eq,
    R: IntoIterator<Item = (K, V)>,
{
    fn from_iter<T: IntoIterator<Item = R>>(iter: T) -> Self {
        let mut keys = HashMap::new();
        let mut values = Vec::new();
        let mut iter = iter.into_iter();
        if let Some(first_row) = iter.next() {
            for (i, (k, v)) in first_row.into_iter().enumerate() {
                keys.insert(k, i);
                values.push(v);
            }
            let mut row_buf: Vec<(usize, V)> = Vec::with_capacity(keys.len());
            for row in iter {
                for (k, v) in row {
                    let Some(idx) = keys.get(&k) else {
                        panic!("Row contains key that is not present in the first row")
                    };

                    row_buf.push((*idx, v));
                }
                row_buf.sort_by_key(|(i, _)| *i);
                row_buf.dedup_by_key(|(i, _)| *i);
                if row_buf.len() != keys.len() {
                    panic!("Row length and columns amount mismatch");
                }
                values.extend(row_buf.drain(..).map(|(_, v)| v))
            }
        }
        Self {
            indices_table: keys,
            values_vector: values,
        }
    }
}

/// Convenience struct that allows using [`FromIterator`] to build from column iterator without
/// implementation conflicting with row [`FromIterator`]
#[derive(Debug)]
pub struct HashTableFromColumns<K, V>(pub HashTable<K, V>);

impl<K, V> From<HashTableFromColumns<K, V>> for HashTable<K, V> {
    fn from(value: HashTableFromColumns<K, V>) -> Self {
        value.0
    }
}

impl<K, V> Deref for HashTableFromColumns<K, V> {
    type Target = HashTable<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> FromIterator<HashTableColumnOwned<K, V>> for HashTableFromColumns<K, V>
where
    K: Hash + Eq,
{
    #[inline]
    fn from_iter<T: IntoIterator<Item = HashTableColumnOwned<K, V>>>(iter: T) -> Self {
        Self(HashTable::from_column_iter(iter))
    }
}
