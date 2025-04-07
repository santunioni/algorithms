use std::cmp::Ordering;

struct Node<T> {
    item: T,
    balance_factor: i8,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(item: T) -> Box<Self> {
        Box::new(Node {
            item,
            left: None,
            right: None,
            balance_factor: 0,
        })
    }

    fn is_balanced(&self) -> bool {
        -1 <= self.balance_factor && self.balance_factor <= 1
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
}

struct AVLTree<T: PartialOrd> {
    root: Box<Node<T>>,
}

impl<T: PartialOrd> AVLTree<T> {
    fn new(item: T) -> Self {
        AVLTree {
            root: Node::new(item),
        }
    }
}

impl<T: PartialOrd> AVLTree<T> {
    fn add(&mut self, item: T) {
        AVLTree::add_neighbor(&mut self.root, Node::new(item));
    }

    fn add_neighbor(node: &mut Node<T>, neighbor: Box<Node<T>>) -> i8 {
        if neighbor.item >= node.item {
            match &mut node.right {
                Some(right) => {
                    AVLTree::add_neighbor(right, neighbor);
                }
                None => node.right = Some(neighbor),
            }
            node.balance_factor += 1;
        } else {
            match &mut node.left {
                Some(left) => {
                    AVLTree::add_neighbor(left, neighbor);
                }
                None => node.left = Some(neighbor),
            }
            node.balance_factor -= 1;
        }
        node.balance_factor
    }

    fn contains(&self, item: &T) -> bool {
        Self::exists_in_node(&self.root, item)
    }

    fn exists_in_node(node: &Node<T>, item: &T) -> bool {
        match item.partial_cmp(&node.item) {
            None => false,
            Some(Ordering::Equal) => true,
            Some(Ordering::Less) => match &node.left {
                None => false,
                Some(left) => Self::exists_in_node(left, item),
            },
            Some(Ordering::Greater) => match &node.right {
                None => false,
                Some(right) => Self::exists_in_node(right, item),
            },
        }
    }

    fn is_balanced(&self) -> bool {
        self.root.is_deep_balanced()
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_8_avl_binary_tree::AVLTree;

    #[test]
    fn should_add_and_check_element() {
        let mut tree = AVLTree::new(1);

        tree.add(2);
        tree.add(0);

        assert!(tree.contains(&1));
        assert!(tree.contains(&2));
        assert!(tree.contains(&0));
        assert!(!tree.contains(&3));
    }

    #[test]
    fn should_balance_tree_inserting_right() {
        let mut tree = AVLTree::new(0);

        for item in 1..1000 {
            tree.add(item);
            assert!(tree.contains(&item));
            assert!(tree.is_balanced())
        }
    }
    #[test]
    fn should_balance_tree_inserting_left() {
        let mut tree = AVLTree::new(1000);

        for item in (0..1000).rev() {
            tree.add(item);
            assert!(tree.contains(&item));
            assert!(tree.is_balanced())
        }
    }
}
