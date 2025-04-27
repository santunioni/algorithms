use std::cmp::Ordering;
use std::mem;

type ExtractKey<K, T> = fn(&T) -> &K;

#[derive(Clone)]
struct Node<K: Ord, T> {
    item: T,
    height: u16,
    left: Option<Box<Node<K, T>>>,
    right: Option<Box<Node<K, T>>>,
    extract_key: ExtractKey<K, T>,
}

impl<K: Ord, T> Node<K, T> {
    fn new(item: T, extract_key: ExtractKey<K, T>) -> Box<Self> {
        Box::new(Node {
            item,
            left: None,
            right: None,
            height: 0,
            extract_key,
        })
    }

    fn update_height(&mut self) {
        let left_height = self.left.as_ref().map_or(0, |node| node.height);
        let right_height = self.right.as_ref().map_or(0, |node| node.height);
        self.height = 1 + left_height.max(right_height);
    }

    fn find(&self, lookup_key: &K) -> Option<&T> {
        let extract_key = self.extract_key;
        let self_key = extract_key(&self.item);

        match self_key.cmp(lookup_key) {
            Ordering::Equal => Some(&self.item),
            Ordering::Less => self.right.as_ref().and_then(|node| node.find(lookup_key)),
            Ordering::Greater => self.left.as_ref().and_then(|node| node.find(lookup_key)),
        }
    }

    fn rotate_right(&mut self) {
        let Some(mut picked) = self.left.take() else {
            return;
        };

        mem::swap(self, &mut picked);

        picked.left = self.right.take();
        picked.update_height();

        self.right = Some(picked);
    }

    fn rotate_left(&mut self) {
        let Some(mut picked) = self.right.take() else {
            return;
        };

        mem::swap(self, &mut picked);

        picked.right = self.left.take();
        picked.update_height();

        self.left = Some(picked);
    }

    fn balance(&mut self) {
        let left_height = self.left.as_ref().map_or(0, |node| node.height);
        let right_height = self.right.as_ref().map_or(0, |node| node.height);
        let balance_factor = right_height as i16 - left_height as i16;

        if balance_factor <= -2 {
            self.rotate_right();
        } else if balance_factor >= 2 {
            self.rotate_left();
        }

        self.update_height();
    }

    fn add(&mut self, neighbor: Box<Self>) {
        let extract_key = self.extract_key;
        let self_key = extract_key(&self.item);
        let lookup_key = extract_key(&neighbor.item);

        let child = match self_key.cmp(lookup_key) {
            Ordering::Less => &mut self.right,
            Ordering::Equal | Ordering::Greater => &mut self.left,
        };

        match child {
            Some(child) => child.add(neighbor),
            None => *child = Some(neighbor),
        }

        self.balance();
    }

    /// Removes a node with the specified key and returns its value
    /// Returns a tuple containing the removed item (if found) and the new subtree root
    fn pop_by_key(mut node: Box<Self>, key: &K) -> (Option<T>, Option<Box<Self>>) {
        // Extract the key from the current node
        let extract_key = node.extract_key;
        let self_key = extract_key(&node.item);

        // Compare the current node's key with the key to be removed
        match self_key.cmp(key) {
            // Found the key - remove this node
            Ordering::Equal => {
                let item = node.item;

                // Case 1: Leaf node (no children)
                if node.left.is_none() && node.right.is_none() {
                    return (Some(item), None);
                }

                // Case 2: Only right child
                if node.left.is_none() {
                    return (Some(item), node.right);
                }

                // Case 3: Only left child
                if node.right.is_none() {
                    return (Some(item), node.left);
                }

                // Case 4: Both children exist - replace with in-order successor
                let (successor, new_right) = Self::pop_min(node.right.take().unwrap());
                node.item = successor;
                node.right = new_right;
                node.balance();

                (Some(item), Some(node))
            }

            // Search left subtree
            Ordering::Greater => {
                if let Some(left) = node.left.take() {
                    let (item, new_left) = Self::pop_by_key(left, key);
                    node.left = new_left;
                    node.balance();
                    (item, Some(node))
                } else {
                    // Key not found
                    (None, Some(node))
                }
            }

            // Search right subtree
            Ordering::Less => {
                if let Some(right) = node.right.take() {
                    let (item, new_right) = Self::pop_by_key(right, key);
                    node.right = new_right;
                    node.balance();
                    (item, Some(node))
                } else {
                    // Key not found
                    (None, Some(node))
                }
            }
        }
    }

    /// Removes the minimum value node (leftmost) from the subtree
    /// Returns a tuple containing the popped item and the new subtree root
    fn pop_min(mut node: Box<Self>) -> (T, Option<Box<Self>>) {
        // If this node has no left child, it's the minimum
        if node.left.is_none() {
            let item = node.item;
            // Return the right subtree as the new root
            return (item, node.right);
        }

        // Recursively find and remove the minimum node from the left subtree
        let (item, new_left) = Self::pop_min(node.left.take().unwrap());
        node.left = new_left;

        // Rebalance the tree and update height
        node.balance();

        (item, Some(node))
    }
}

struct AVLTree<K: Ord, T> {
    root: Option<Box<Node<K, T>>>,
    extract_key: ExtractKey<K, T>,
}

impl<T: Ord> AVLTree<T, T> {
    fn empty() -> Self {
        AVLTree {
            root: None,
            extract_key: |v| v,
        }
    }
}

impl<K: Ord, T> AVLTree<K, T> {
    fn new(extract_key: ExtractKey<K, T>) -> Self {
        AVLTree {
            root: None,
            extract_key,
        }
    }

    fn add(&mut self, item: T) {
        if let Some(root) = &mut self.root {
            let neighbor = Node::new(item, self.extract_key);
            root.add(neighbor);
        } else {
            self.root = Some(Node::new(item, self.extract_key));
        }
    }

    fn contains(&self, key: &K) -> bool {
        self.find(key).is_some()
    }

    fn find(&self, key: &K) -> Option<&T> {
        self.root.as_ref().and_then(|root| root.find(key))
    }

    fn height(&self) -> u16 {
        self.root.as_ref().map_or(0, |root| root.height)
    }

    fn iter(&self) -> AVLTreeIterator<K, T> {
        AVLTreeIterator::new(self)
    }

    /// Removes and returns the item with the specified key
    /// Returns None if the key doesn't exist in the tree
    pub fn pop_by_key(&mut self, key: &K) -> Option<T> {
        if self.root.is_none() {
            return None;
        }

        let root = self.root.take().unwrap();
        let (item, new_root) = Node::pop_by_key(root, key);
        self.root = new_root;
        item
    }
}

struct AVLTreeIterator<'a, K: Ord, T> {
    stack: Vec<&'a Node<K, T>>,
}

impl<'a, K: Ord, T> AVLTreeIterator<'a, K, T> {
    fn new(tree: &'a AVLTree<K, T>) -> Self {
        let mut iterator = AVLTreeIterator {
            stack: Vec::with_capacity(tree.height() as usize),
        };

        if let Some(root) = &tree.root {
            iterator.push_left_leg(root.as_ref());
        }

        iterator
    }

    // Helper method to push all nodes along the left branch onto the stack
    fn push_left_leg(&mut self, mut node: &'a Node<K, T>) {
        self.stack.push(node);
        while let Some(left) = node.left.as_deref() {
            self.stack.push(left);
            node = left;
        }
    }
}

impl<'a, K: Ord, T> Iterator for AVLTreeIterator<'a, K, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        if let Some(right) = node.right.as_deref() {
            self.push_left_leg(right);
        }

        Some(&node.item)
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_8_avl_binary_tree::AVLTree;
    use rand::prelude::SliceRandom;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn should_add_and_check_element() {
        let mut tree = AVLTree::empty();

        tree.add(1);
        tree.add(2);
        tree.add(0);

        assert!(tree.contains(&1));
        assert!(tree.contains(&2));
        assert!(tree.contains(&0));
        assert!(!tree.contains(&3));

        assert_eq!(tree.height(), 1);
    }

    #[test]
    fn should_balance_tree_inserting_right_hundred() {
        let mut tree = AVLTree::empty();

        for item in 1..100 {
            tree.add(item);
        }

        for item in 1..100 {
            assert!(tree.contains(&item));
        }

        assert_eq!(tree.height(), 7);
    }

    #[test]
    fn should_balance_tree_inserting_left_hundred() {
        let mut tree = AVLTree::empty();

        for item in (0..100).rev() {
            tree.add(item);
        }

        for item in 1..100 {
            assert!(tree.contains(&item));
        }

        assert_eq!(tree.height(), 7);
    }

    #[test]
    fn should_sort_and_search_by_key() {
        struct Person {
            name: String,
        }

        let mut tree = AVLTree::<String, Person>::new(|u| &u.name);

        tree.add(Person {
            name: "Vinícius".to_string(),
        });
        tree.add(Person {
            name: "Bianca".to_string(),
        });
        tree.add(Person {
            name: "José".to_string(),
        });

        assert_eq!(
            tree.find(&"Vinícius".to_string()).unwrap().name,
            "Vinícius".to_string()
        );

        assert!(tree.find(&"João".to_string()).is_none());
    }

    #[test]
    fn should_iter_ordered() {
        let mut tree = AVLTree::empty();
        for item in 0..400 {
            tree.add(item);
        }

        let it = tree.iter();
        let collected_tree = it.collect::<Vec<_>>();

        let expected_vec = (0..400).collect::<Vec<i32>>();
        let expected = expected_vec.iter().collect::<Vec<&i32>>();

        assert_eq!(collected_tree, expected);
    }

    #[test]
    fn should_balance_tree_inserting_random_positions() {
        let size = 100;
        let mut rng = StdRng::seed_from_u64(42);
        let mut numbers: Vec<i32> = (0..size as i32).collect();
        numbers.shuffle(&mut rng);

        let mut tree = AVLTree::empty();
        for &num in &numbers {
            tree.add(num);
        }

        for i in 0..size as i32 {
            assert!(tree.contains(&i));
        }
        assert_eq!(tree.height(), 8);
    }

    #[test]
    fn should_pop_by_key() {
        let mut tree = AVLTree::empty();

        // Add elements
        for item in 0..10 {
            tree.add(item);
        }

        // Pop existing elements
        assert_eq!(tree.pop_by_key(&5), Some(5));
        assert!(!tree.contains(&5));

        assert_eq!(tree.pop_by_key(&0), Some(0));
        assert!(!tree.contains(&0));

        assert_eq!(tree.pop_by_key(&9), Some(9));
        assert!(!tree.contains(&9));

        assert_eq!(tree.height(), 4);

        // Try to pop non-existent elements
        assert_eq!(tree.pop_by_key(&20), None);
        assert_eq!(tree.pop_by_key(&5), None); // Already removed

        // Verify remaining elements
        for item in [1, 2, 3, 4, 6, 7, 8] {
            assert!(tree.contains(&item));
            assert_eq!(tree.pop_by_key(&item), Some(item));
            assert!(!tree.contains(&item));
        }

        // Tree should be empty now
        assert_eq!(tree.height(), 0);
    }
}
