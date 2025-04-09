use crate::chapter_4_stack::Stack;
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

enum Side {
    Left,
    Right,
}

enum RequiredRotation {
    Clock,
    Counter,
    None,
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

    fn refresh_metadata(&mut self) {
        match (&self.left, &self.right) {
            (None, None) => {
                self.height = 0;
            }
            (Some(left), None) => {
                self.height = left.height + 1;
            }
            (None, Some(right)) => {
                self.height = right.height + 1;
            }
            (Some(left), Some(right)) => {
                self.height = left.height.max(right.height) + 1;
            }
        }
    }

    fn get_required_rotation(&self) -> RequiredRotation {
        match (&self.left, &self.right) {
            (None, None) => RequiredRotation::None,
            (Some(left), None) => {
                if left.height > 1 {
                    RequiredRotation::Clock
                } else {
                    RequiredRotation::None
                }
            }
            (None, Some(right)) => {
                if right.height > 1 {
                    RequiredRotation::Counter
                } else {
                    RequiredRotation::None
                }
            }
            (Some(left), Some(right)) => match right.height as i32 - left.height as i32 {
                (2..) => RequiredRotation::Counter,
                (..=-2) => RequiredRotation::Clock,
                _ => RequiredRotation::None,
            },
        }
    }

    fn is_balanced(&self) -> bool {
        matches!(self.get_required_rotation(), RequiredRotation::None)
    }

    fn is_deep_balanced(&self) -> bool {
        if !self.is_balanced() {
            return false;
        }
        if let Some(left) = &self.left {
            if !Self::is_deep_balanced(left) {
                return false;
            }
        };
        if let Some(right) = &self.right {
            if !right.is_deep_balanced() {
                return false;
            }
        };
        true
    }

    fn find_deep(&self, lookup_key: &K) -> Option<&T> {
        let extract_key = self.extract_key;
        let self_key = extract_key(&self.item);
        match self_key.cmp(lookup_key) {
            Ordering::Equal => Some(&self.item),
            Ordering::Less => match &self.right {
                None => None,
                Some(right) => right.find_deep(lookup_key),
            },
            Ordering::Greater => match &self.left {
                None => None,
                Some(left) => left.find_deep(lookup_key),
            },
        }
    }

    fn maybe_rotate(pivot: &mut Option<Box<Self>>, orientation: RequiredRotation) {
        match orientation {
            RequiredRotation::Clock => {
                let Some(mut taken) = pivot.take() else {
                    return;
                };
                let Some(mut left) = taken.left.take() else {
                    return;
                };
                taken.left = left.right.take();

                taken.refresh_metadata();
                left.right = Some(taken);

                left.refresh_metadata();
                pivot.replace(left);
            }
            RequiredRotation::Counter => {
                let Some(mut taken) = pivot.take() else {
                    return;
                };
                let Some(mut right) = taken.right.take() else {
                    return;
                };
                taken.right = right.left.take();

                taken.refresh_metadata();
                right.left = Some(taken);

                right.refresh_metadata();
                pivot.replace(right);
            }
            RequiredRotation::None => {}
        }
    }

    fn add_neighbor(&mut self, neighbor: Box<Self>) -> RequiredRotation {
        let extract_key = self.extract_key;
        let self_key = extract_key(&self.item);
        let lookup_key = extract_key(&neighbor.item);

        match self_key.cmp(lookup_key) {
            Ordering::Less => match &mut self.right {
                Some(self_right) => {
                    let rotation = self_right.add_neighbor(neighbor);
                    Self::maybe_rotate(&mut self.right, rotation)
                }
                None => self.right = Some(neighbor),
            },
            Ordering::Greater | Ordering::Equal => match &mut self.left {
                Some(self_left) => {
                    let rotation = self_left.add_neighbor(neighbor);
                    Self::maybe_rotate(&mut self.left, rotation)
                }
                None => self.left = Some(neighbor),
            },
        }

        self.refresh_metadata();
        self.get_required_rotation()
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
}

impl<K: Ord, T> AVLTree<K, T> {
    fn add(&mut self, item: T) {
        match &mut self.root {
            None => self.root = Some(Node::new(item, self.extract_key)),
            Some(root) => {
                let neighbor = Node::new(item, self.extract_key);
                let rotation = root.add_neighbor(neighbor);
                Node::maybe_rotate(&mut self.root, rotation)
            }
        };
    }

    fn contains(&self, key: &K) -> bool {
        self.find(key).is_some()
    }

    fn find(&self, key: &K) -> Option<&T> {
        match &self.root {
            None => None,
            Some(root) => root.find_deep(key),
        }
    }

    fn is_balanced(&self) -> bool {
        match &self.root {
            None => true,
            Some(root) => root.is_deep_balanced(),
        }
    }

    fn height(&self) -> u16 {
        match &self.root {
            None => 0,
            Some(root) => root.height,
        }
    }

    fn iter(&self) -> AVLTreeIterator<K, T> {
        AVLTreeIterator::new(self)
    }
}

struct AVLTreeIterator<'a, K: Ord, T> {
    stack: Stack<&'a Node<K, T>>,
    next: Option<&'a Node<K, T>>,
}

impl<'a, K: Ord, T> AVLTreeIterator<'a, K, T> {
    fn new(tree: &'a AVLTree<K, T>) -> Self {
        let mut stack = Stack::empty();
        if let Some(root) = &tree.root {
            stack.prepend(root.as_ref());
        }
        AVLTreeIterator { stack, next: None }
    }
}

impl<'a, K: Ord, T> Iterator for AVLTreeIterator<'a, K, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(next) = self.next {
                return if let Some(right) = &next.right {
                    self.stack.prepend(right);
                    self.next.take()
                } else {
                    mem::replace(&mut self.next, self.stack.pop_head())
                }
                .map(|v| &v.item);
            }

            let peeked_head = self.stack.peek_head()?;
            if let Some(left) = &peeked_head.left {
                self.stack.prepend(left);
                continue;
            }
            let to_return = self.stack.pop_head()?;

            self.next = if let Some(right) = &to_return.right {
                Some(right)
            } else {
                self.stack.pop_head()
            };

            return Some(&to_return.item);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_8_avl_binary_tree::AVLTree;

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
        assert!(tree.is_balanced());
        assert_eq!(tree.height(), 1);
    }

    #[test]
    fn should_balance_tree_inserting_right_hundred() {
        let mut tree = AVLTree::empty();

        for item in 1..100 {
            tree.add(item);
        }

        assert!(tree.is_balanced());
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

        assert!(tree.is_balanced());
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
}
