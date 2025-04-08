use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
struct Node<T> {
    item: T,
    balance_factor: i8,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

enum Side {
    Left,
    Right,
}

enum Rotation {
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

    fn rotate_pivot(pivot: &mut Option<Box<Node<T>>>, orientation: Rotation) {
        match orientation {
            Rotation::Clock => {
                let Some(mut taken) = pivot.take() else { return; };
                let Some(mut left) = taken.left.take() else { return; };
                taken.left = left.right.take();
                taken.balance_factor += 2;
                left.balance_factor += 1;
                left.right = Some(taken);
                pivot.replace(left);
            },
            Rotation::Counter => {
                let Some(mut taken) = pivot.take() else { return; };
                let Some(mut right) = taken.right.take() else { return; };
                taken.right = right.left.take();
                taken.balance_factor -= 2;
                right.balance_factor -= 1;
                right.left = Some(taken);
                pivot.replace(right);
            }
            Rotation::None => {}
        }
    }

    fn add_neighbor(&mut self, neighbor: Box<Node<T>>) -> Rotation {
        if neighbor.item >= self.item {
            match &mut self.right {
                Some(self_right) => {
                    let rotation = self_right.add_neighbor(neighbor);
                    Self::rotate_pivot(&mut self.right, rotation)
                }
                None => self.right = Some(neighbor),
            }
            self.balance_factor += 1;
        } else {
            match &mut self.left {
                Some(self_left) => {
                    let rotation = self_left.add_neighbor(neighbor);
                    Self::rotate_pivot(&mut self.left, rotation)
                }
                None => self.left = Some(neighbor),
            }
            self.balance_factor -= 1;
        }

        if self.balance_factor < -1 {
            Rotation::Clock
        } else if self.balance_factor > 1 {
            Rotation::Counter
        } else { Rotation::None }
    }

}

struct AVLTree<T: PartialOrd> {
    root: Option<Box<Node<T>>>,
}

impl<T: PartialOrd> AVLTree<T> {
    fn empty() -> Self {
        AVLTree {
            root: None,
        }
    }
}

impl<T: PartialOrd> AVLTree<T> {
    fn add(&mut self, item: T) {
        match &mut self.root {
            None => self.root = Some(Node::new(item)),
            Some(root) => {
                let neighbor = Node::new(item);
                let rotation = root.add_neighbor(neighbor);
                Node::rotate_pivot(&mut self.root, rotation)
            }
        };
    }

    fn contains(&self, item: &T) -> bool {
        match &self.root {
            None => false,
            Some(root) => root.node_contains_deep(item)
        }
    }

    fn is_balanced(&self) -> bool {
        match &self.root {
            None => true,
            Some(root) => root.is_deep_balanced()
        }
    }
}

impl<T: PartialOrd + Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.item)?;
        if let Some(left) = &self.left {
            write!(f, "{:p}::Left: ", self)?;
            Node::fmt(left, f)?;
        }
        if let Some(right) = &self.right {
            write!(f, "{:p}::Rght: ", self)?;
            Node::fmt(right, f)?;
        }
        Ok(())
    }
}

impl<T: PartialOrd + Display> Display for AVLTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(root) = &self.root {
            write!(f, "{:p}::Root: ", root)?;
            root.fmt(f)?
        };
        Ok(())
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

        println!("{}", &tree)
    }

    #[test]
    fn should_balance_tree_inserting_right() {
        let mut tree = AVLTree::empty();

        tree.add(0);
        tree.add(1);
        tree.add(2);
        tree.add(3);

        assert!(tree.contains(&0));
        assert!(tree.contains(&1));
        assert!(tree.contains(&2));
        assert!(tree.contains(&3));

        assert!(tree.is_balanced());
        println!("{}", &tree);
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
        assert!(!tree.contains(&101));
        println!("{}", &tree);
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
        assert!(!tree.contains(&101));
        println!("{}", &tree);
    }

    #[test]
    fn should_print() {
        let mut tree = AVLTree::empty();

        for item in (0..100).rev() {
            tree.add(item);
        }

        println!("{}", &tree.root.clone().unwrap().item);

        println!("{}", &tree.root.clone().unwrap().right.unwrap().item);

        println!("{}", &tree.root.clone().unwrap().right.unwrap().left.unwrap().item);
        println!("{}", &tree.root.clone().unwrap().right.unwrap().right.unwrap().item);

        println!("{}", &tree.root.clone().unwrap().left.unwrap().item);

        println!("{}", &tree.root.clone().unwrap().left.unwrap().left.unwrap().item);
        println!("{}", &tree.root.clone().unwrap().left.unwrap().right.unwrap().item);
    }
}
