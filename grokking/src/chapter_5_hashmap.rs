use crate::chapter_4_stack::Stack;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem;

struct HashMapEntry<K, V>
where
    K: Hash + PartialEq,
{
    key: K,
    value: V,
}

impl<K, V> HashMapEntry<K, V>
where
    K: Hash + PartialEq,
{
    fn new(key: K, value: V) -> Self {
        HashMapEntry { key, value }
    }
}

struct HashMap<K, V>
where
    K: Hash + PartialEq,
{
    hash_table: Vec<Option<Stack<HashMapEntry<K, V>>>>,
    number_of_values: u32,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + PartialEq,
{
    fn hash_key(&self, key: &K) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        (s.finish() % self.hash_table.len() as u64) as usize
    }

    fn create_vector_of_nones(size: usize) -> Vec<Option<Stack<HashMapEntry<K, V>>>> {
        let mut hash_table = Vec::with_capacity(size);
        for _ in 0..size {
            hash_table.push(None)
        }
        hash_table
    }

    fn maybe_resize(&mut self) {
        let should_resize = 4 * self.number_of_values > 3 * self.hash_table.len() as u32;
        if !should_resize {
            return;
        }
        let preserved_number_of_values = self.number_of_values;
        let preserved_old_hash_table = mem::replace(
            &mut self.hash_table,
            HashMap::create_vector_of_nones((preserved_number_of_values * 2) as usize),
        );
        preserved_old_hash_table
            .into_iter()
            .flatten()
            .for_each(|stack| {
                stack
                    .drain()
                    .for_each(|table_stack| self.insert(table_stack.key, table_stack.value))
            });
        self.number_of_values = preserved_number_of_values;
    }

    pub fn new() -> Self {
        HashMap {
            hash_table: HashMap::create_vector_of_nones(16),
            number_of_values: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let hash = self.hash_key(&key);
        let entry = HashMapEntry::new(key, value);
        match &mut self.hash_table[hash] {
            None => {
                self.hash_table[hash] = {
                    self.number_of_values += 1;
                    Some(Stack::new(entry))
                }
            }
            Some(shelf) => {
                if let Some(el) = shelf.iter_mut().find(|v| v.key == entry.key) {
                    el.value = entry.value
                } else {
                    shelf.push_head(entry);
                    self.number_of_values += 1;
                }
            }
        }
    }

    pub fn get(&self, key_ref: &K) -> Option<&V> {
        let hash = self.hash_key(key_ref);
        match &self.hash_table[hash] {
            None => None,
            Some(stack) => stack
                .iter()
                .find(|&it| &it.key == key_ref)
                .map(|entry| &entry.value),
        }
    }

    pub fn remove(&mut self, key_ref: &K) -> Option<V> {
        let hash = self.hash_key(key_ref);
        match &mut self.hash_table[hash] {
            None => None,
            Some(stack) => stack.remove_by(|it| &it.key == key_ref).map(|entry| {
                self.number_of_values -= 1;
                entry.value
            }),
        }
    }

    pub fn size(&self) -> u32 {
        self.number_of_values
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_5_hashmap::HashMap;

    #[test]
    fn should_insert_and_get() {
        let mut map = HashMap::new();

        map.insert("key", 1);
        map.insert("key", 2);

        assert_eq!(map.get(&"key"), Some(&2));
    }

    #[test]
    fn should_remove() {
        let mut map = HashMap::new();

        map.insert("key", 1);
        map.insert("key", 2);

        assert_eq!(map.remove(&"key"), Some(2));
        assert_eq!(map.remove(&"key"), None);
    }

    #[test]
    fn should_insert_and_get_thousand_elements() {
        let mut map = HashMap::new();

        for i in 0..1000 {
            map.insert(format!("key{}", i), i);
        }

        for i in 0..1000 {
            assert_eq!(map.get(&format!("key{}", i)), Some(&i));
        }
        assert_eq!(map.size(), 1000);
    }
}
