use crate::{column::owned::HashTableColumnOwned, row::borrowed::HashTableRowBorrowed, *};

impl<K, V> IntoIterator for HashTable<K, V>
where
    K: Clone,
    K: Hash + Eq,
{
    type Item = HashMap<K, V>;
    type IntoIter = HashTableIntoIter<K, V>;

    /// Iterator taht takes ownership of both keys and values, cloning the keys each time and
    /// allocating a new hashmap
    fn into_iter(self) -> Self::IntoIter {
        HashTableIntoIter { inner: self }
    }
}

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
}

impl<K, V> HashTable<K, V> {
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

    pub fn into_iter_columns(self) -> HashTableOwnedIntoIterColumn<K, V> {
        HashTableOwnedIntoIterColumn {
            row_len: self.columns_len(),
            indices_iter: self.indices_table.into_iter(),
            values: self.values_vector.into_iter().map(Option::Some).collect(),
        }
    }
}

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

#[derive(Debug)]
pub struct HashTableOwnedIntoIterColumn<K, V> {
    indices_iter: <HashMap<K, usize> as IntoIterator>::IntoIter,
    values: Vec<Option<V>>,
    row_len: usize,
}

impl<K, V> Iterator for HashTableOwnedIntoIterColumn<K, V>
where
    K: Hash + Eq,
{
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
}
