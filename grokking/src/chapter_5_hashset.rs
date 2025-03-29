use crate::chapter_4_stack::Stack;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem;

struct HashSet<K>
where
    K: Hash + PartialEq,
{
    hash_table: Vec<Option<Stack<K>>>,
    number_of_values: u32,
}

impl<K> HashSet<K>
where
    K: Hash + PartialEq,
{
    fn hash_key(&self, key: &K) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        (s.finish() % self.hash_table.len() as u64) as usize
    }

    fn create_vector_of_nones(size: usize) -> Vec<Option<Stack<K>>> {
        let mut hash_table = Vec::with_capacity(size);
        for _ in 0..size {
            hash_table.push(None)
        }
        hash_table
    }

    fn maybe_resize_table(&mut self) {
        let old_hash_table_len = self.hash_table.len();

        let should_resize = 4 * self.number_of_values > 3 * old_hash_table_len as u32;
        if !should_resize {
            return;
        }

        let preserved_old_hash_table = mem::replace(
            &mut self.hash_table,
            HashSet::create_vector_of_nones(old_hash_table_len * 2),
        );
        self.number_of_values = 0;
        preserved_old_hash_table
            .into_iter()
            .flatten()
            .flat_map(Stack::drain)
            .for_each(|entry| self.insert_hash_table_value(entry));
    }

    pub fn new() -> Self {
        HashSet {
            hash_table: HashSet::create_vector_of_nones(16),
            number_of_values: 0,
        }
    }

    fn insert_hash_table_value(&mut self, value: K) {
        let hash = self.hash_key(&value);
        match &mut self.hash_table[hash] {
            None => {
                self.hash_table[hash] = {
                    self.number_of_values += 1;
                    Some(Stack::new(value))
                }
            }
            Some(shelf) => {
                if !shelf.iter().any(|value_in_shelf| *value_in_shelf == value) {
                    shelf.push_head(value);
                    self.number_of_values += 1;
                }
            }
        }
    }

    pub fn insert(&mut self, key: K) {
        self.insert_hash_table_value(key);
        self.maybe_resize_table();
    }

    pub fn contains(&self, key_ref: &K) -> bool {
        let hash = self.hash_key(key_ref);
        match &self.hash_table[hash] {
            None => false,
            Some(stack) => stack.iter().any(|it| it == key_ref),
        }
    }

    pub fn remove(&mut self, lookup_value: &K) -> bool {
        let hash = self.hash_key(lookup_value);
        match &mut self.hash_table[hash] {
            None => false,
            Some(stack) => stack
                .remove_by(|value_in_shelf| value_in_shelf == lookup_value)
                .map(|_| {
                    self.number_of_values -= 1;
                    true
                })
                .is_some(),
        }
    }

    pub fn size(&self) -> u32 {
        self.number_of_values
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_5_hashset::HashSet;

    #[test]
    fn should_insert_and_verify() {
        let mut map = HashSet::new();

        map.insert("key");

        assert!(map.contains(&"key"));
    }

    #[test]
    fn should_remove() {
        let mut map = HashSet::new();

        map.insert("key1");
        map.insert("key2");
        map.remove(&"key1");

        assert!(!map.contains(&"key1"));
        assert!(map.contains(&"key2"));
    }

    #[test]
    fn should_insert_and_verify_thousand_elements() {
        let mut map = HashSet::new();

        for i in 0..2000 {
            map.insert(format!("key{}", i % 1000));
        }

        for i in 0..1000 {
            assert!(map.contains(&format!("key{}", i)));
        }
        assert_eq!(map.size(), 1000);
    }


    #[test]
    fn should_not_break_when_removing_item_that_doesnt_exist() {
        let mut map = HashSet::new();

        assert!(!map.remove(&"key".to_string()));
    }

    #[test]
    fn should_return_none() {
        let map= HashSet::new();

        assert!(!map.contains(&"key".to_string()));
    }
}
