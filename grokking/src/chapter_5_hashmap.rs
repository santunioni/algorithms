use crate::chapter_4_stack::Stack;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem;

struct HashTableEntry<K, V>
where
    K: Hash + PartialEq,
{
    key: K,
    value: V,
}

struct HashMap<K, V>
where
    K: Hash + PartialEq,
{
    hash_table: Vec<Stack<HashTableEntry<K, V>>>,
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

    fn maybe_grow_table(&mut self) {
        let old_hash_table_len = self.hash_table.len();

        let should_grow = 4 * self.number_of_values > 3 * old_hash_table_len as u32;
        if !should_grow {
            return;
        }

        let preserved_old_hash_table = mem::replace(
            &mut self.hash_table,
            Stack::create_many(old_hash_table_len * 2),
        );
        self.number_of_values = 0;
        preserved_old_hash_table
            .into_iter()
            .flat_map(Stack::drain)
            .for_each(|entry| self.insert_hash_table_entry(entry));
    }

    pub fn new() -> Self {
        HashMap {
            hash_table: Stack::create_many(16),
            number_of_values: 0,
        }
    }

    fn insert_hash_table_entry(&mut self, new_entry: HashTableEntry<K, V>) {
        let hash = self.hash_key(&new_entry.key);
        let shelf = &mut self.hash_table[hash];
        if let Some(existing_entry) = shelf
            .iter_mut()
            .find(|existing_entry| existing_entry.key == new_entry.key)
        {
            existing_entry.value = new_entry.value
        } else {
            shelf.prepend(new_entry);
            self.number_of_values += 1;
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.insert_hash_table_entry(HashTableEntry { key, value });
        self.maybe_grow_table();
    }

    pub fn get(&self, key_ref: &K) -> Option<&V> {
        let hash = self.hash_key(key_ref);
        self.hash_table[hash]
            .iter()
            .find(|&it| &it.key == key_ref)
            .map(|entry| &entry.value)
    }

    pub fn remove(&mut self, key_ref: &K) -> Option<V> {
        let hash = self.hash_key(key_ref);
        self.hash_table[hash]
            .remove_by(|it| &it.key == key_ref)
            .map(|entry| {
                self.number_of_values -= 1;
                entry.value
            })
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
            map.insert(format!("key{i}"), i);
        }

        for i in 0..1000 {
            assert_eq!(map.get(&format!("key{i}")), Some(&i));
        }
        assert_eq!(map.size(), 1000);
    }

    #[test]
    fn should_not_break_when_removing_item_that_doesnt_exist() {
        let mut map: HashMap<String, i32> = HashMap::new();

        assert_eq!(map.remove(&"key".to_string()), None);
    }

    #[test]
    fn should_return_none() {
        let map: HashMap<String, i32> = HashMap::new();

        assert_eq!(map.get(&"key".to_string()), None);
    }
}
