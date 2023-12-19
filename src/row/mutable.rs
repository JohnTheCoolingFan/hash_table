use std::borrow::Borrow;

use crate::*;

#[derive(Debug)]
pub struct HashTableMutableBorrowedRow<'t, K, V> {
    pub(crate) indices_table: &'t HashMap<K, usize>,
    pub(crate) values: &'t mut [V],
}

impl<'t, 'r: 't, K, V> HashTableMutableBorrowedRow<'t, K, V> {
    pub fn get<Q>(&'r mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        K: Hash + Eq,
        Q: Hash + Eq + ?Sized,
    {
        self.indices_table.get(key).map(|i| &mut self.values[*i])
    }
}

impl<'t, K, V> IntoIterator for HashTableMutableBorrowedRow<'t, K, V> {
    type Item = (&'t K, &'t mut V);
    type IntoIter = HashTableMutableBorrowedRowIntoIter<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

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
