use std::cmp::Ordering;

type Comparator<T> = fn(&T, &T) -> Option<Ordering>;

#[derive(Clone)]
struct Node<T> {
    item: T,
    height: u16,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    comparator: Comparator<T>,
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

impl<T> Node<T> {
    fn new(item: T, comparator: Comparator<T>) -> Box<Self> {
        Box::new(Node {
            item,
            left: None,
            right: None,
            height: 0,
            comparator,
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

    fn node_contains_deep(&self, item: &T) -> bool {
        let comparator = self.comparator;
        match comparator(item, &self.item) {
            None => false,
            Some(Ordering::Equal) => true,
            Some(Ordering::Less) => match &self.left {
                None => false,
                Some(left) => left.node_contains_deep(item),
            },
            Some(Ordering::Greater) => match &self.right {
                None => false,
                Some(right) => right.node_contains_deep(item),
            },
        }
    }

    fn maybe_rotate(pivot: &mut Option<Box<Node<T>>>, orientation: RequiredRotation) {
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

    fn add_neighbor(&mut self, neighbor: Box<Node<T>>) -> RequiredRotation {
        let comparator = self.comparator;
        match comparator(&self.item, &neighbor.item) {
            Some(Ordering::Less) => match &mut self.right {
                Some(self_right) => {
                    let rotation = self_right.add_neighbor(neighbor);
                    Self::maybe_rotate(&mut self.right, rotation)
                }
                None => self.right = Some(neighbor),
            },
            _ => match &mut self.left {
                Some(self_left) => {
                    let rotation = self_left.add_neighbor(neighbor);
                    Self::maybe_rotate(&mut self.left, rotation)
                }
                None => self.left = Some(neighbor),
            },
        };

        self.refresh_metadata();
        self.get_required_rotation()
    }
}

struct AVLTree<T> {
    root: Option<Box<Node<T>>>,
    comparator: Comparator<T>,
}

impl<T: PartialOrd> AVLTree<T> {
    fn empty() -> Self {
        AVLTree {
            root: None,
            comparator: |this, that| this.partial_cmp(that),
        }
    }
}

impl<T> AVLTree<T> {
    fn new(compare: Comparator<T>) -> Self {
        AVLTree {
            root: None,
            comparator: compare,
        }
    }
}

impl<T> AVLTree<T> {
    fn add(&mut self, item: T) {
        match &mut self.root {
            None => self.root = Some(Node::new(item, self.comparator)),
            Some(root) => {
                let neighbor = Node::new(item, self.comparator);
                let rotation = root.add_neighbor(neighbor);
                Node::maybe_rotate(&mut self.root, rotation)
            }
        };
    }

    fn contains(&self, item: &T) -> bool {
        match &self.root {
            None => false,
            Some(root) => root.node_contains_deep(item),
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
    fn should_add_non_comparable_if_providing_comparator() {
        struct Person {
            name: String,
        }

        let mut tree = AVLTree::<Person>::new(|u, v| u.name.partial_cmp(&v.name));

        tree.add(Person { name: "Vinícius".to_string() });
        tree.add(Person { name: "Bianca".to_string() });
        tree.add(Person { name: "José".to_string() });

        assert!(tree.contains(&Person { name: "Vinícius".to_string() }));
        assert!(!tree.contains(&Person { name: "João".to_string() }));
    }
}
