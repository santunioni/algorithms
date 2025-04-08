use std::cmp::Ordering;

#[derive(Clone)]
struct Node<T> {
    item: T,
    height: u32,
    balance_factor: i8,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
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

impl<T: PartialOrd> Node<T> {
    fn new(item: T) -> Box<Self> {
        Box::new(Node {
            item,
            left: None,
            right: None,
            height: 0,
            balance_factor: 0,
        })
    }

    fn refresh_returning_required_rotation(&mut self) -> RequiredRotation {
        let right_height = if let Some(right) = &self.right {
            right.height as i32
        } else {
            -1
        };
        let left_height = if let Some(left) = &self.left {
            left.height as i32
        } else {
            -1
        };
        self.balance_factor = (right_height - left_height) as i8;
        self.height = (left_height.max(right_height) + 1) as u32;

        if self.balance_factor < -1 {
            RequiredRotation::Clock
        } else if self.balance_factor > 1 {
            RequiredRotation::Counter
        } else {
            RequiredRotation::None
        }
    }

    fn is_balanced(&self) -> bool {
        (-1..=1).contains(&self.balance_factor)
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
        match item.partial_cmp(&self.item) {
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

                taken.refresh_returning_required_rotation();
                left.right = Some(taken);

                left.refresh_returning_required_rotation();
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

                taken.refresh_returning_required_rotation();
                right.left = Some(taken);

                right.refresh_returning_required_rotation();
                pivot.replace(right);
            }
            RequiredRotation::None => {}
        }
    }

    fn add_neighbor(&mut self, neighbor: Box<Node<T>>) -> RequiredRotation {
        if neighbor.item >= self.item {
            match &mut self.right {
                Some(self_right) => {
                    let rotation = self_right.add_neighbor(neighbor);
                    Self::maybe_rotate(&mut self.right, rotation)
                }
                None => self.right = Some(neighbor),
            }
        } else {
            match &mut self.left {
                Some(self_left) => {
                    let rotation = self_left.add_neighbor(neighbor);
                    Self::maybe_rotate(&mut self.left, rotation)
                }
                None => self.left = Some(neighbor),
            }
        }

        self.refresh_returning_required_rotation()
    }
}

struct AVLTree<T: PartialOrd> {
    root: Option<Box<Node<T>>>,
}

impl<T: PartialOrd> AVLTree<T> {
    fn empty() -> Self {
        AVLTree { root: None }
    }
}

impl<T: PartialOrd> AVLTree<T> {
    fn add(&mut self, item: T) {
        match &mut self.root {
            None => self.root = Some(Node::new(item)),
            Some(root) => {
                let neighbor = Node::new(item);
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

    fn height(&self) -> u32 {
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

        assert_eq!(tree.height(), 6);
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

        assert_eq!(tree.height(), 6);
    }
}
