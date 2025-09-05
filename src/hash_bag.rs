use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct HashBag<K: Hash + Eq, V>(HashMap<K, Vec<V>>);

impl<K: Hash + Eq, V> HashBag<K, V> {
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(v) = self.0.get_mut(&key) {
            v.push(value);
        } else {
            self.0.insert(key, vec![value]);
        }
    }

    pub fn get(&self, key: &K) -> Option<&Vec<V>> {
        self.0.get(key)
    }
}

impl<K: Hash + Eq, V> Default for HashBag<K, V> {
    fn default() -> Self {
        HashBag(HashMap::new())
    }
}