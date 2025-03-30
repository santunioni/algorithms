use crate::chapter_5_hashset::HashSet;
use std::cell::{Ref, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;

struct Node<T> {
    id: u64,
    item: T,
    pointers: Vec<Link<T>>,
}

type Link<T> = Rc<RefCell<Node<T>>>;

impl<T> Node<T> {
    fn attach(&mut self, other: &Link<T>) {
        self.pointers.push(Rc::clone(other))
    }

    fn breath_search_iterator(node: &Link<T>) -> GraphNodeIterator<T> {
        GraphNodeIterator::new(node, false, Mode::Breath)
    }

    fn breath_search_drain(node: &Link<T>) -> GraphNodeIterator<T> {
        GraphNodeIterator::new(node, true, Mode::Breath)
    }

    fn depth_search_iterator(node: &Link<T>) -> GraphNodeIterator<T> {
        GraphNodeIterator::new(node, false, Mode::Depth)
    }

    fn depth_search_drain(node: &Link<T>) -> GraphNodeIterator<T> {
        GraphNodeIterator::new(node, true, Mode::Depth)
    }
}

struct NodeFactory {
    id: u64,
}

impl NodeFactory {
    fn new() -> Self {
        NodeFactory { id: 0 }
    }

    fn create_node<T>(&mut self, item: T) -> Link<T> {
        let node = Rc::new(RefCell::new(Node {
            id: self.id,
            item,
            pointers: vec![],
        }));
        self.id += 1;
        node
    }
}

enum Mode {
    Depth,
    Breath,
}

struct GraphNodeIterator<T> {
    queue: VecDeque<Link<T>>,
    visited: HashSet<u64>,
    drain: bool,
    mode: Mode,
}

impl<T> GraphNodeIterator<T> {
    fn new(node: &Link<T>, drain: bool, mode: Mode) -> Self {
        let mut it = GraphNodeIterator {
            queue: VecDeque::new(),
            visited: HashSet::new(),
            drain,
            mode,
        };
        it.put_to_queue(node);
        it
    }

    fn put_to_queue(&mut self, node: &Link<T>) {
        let id = node.borrow().id;
        if !self.visited.contains(&id) {
            self.visited.insert(id);
            match self.mode {
                Mode::Breath => self.queue.push_back(Rc::clone(node)),
                Mode::Depth => self.queue.push_front(Rc::clone(node)),
            }
        }
    }
}

impl<T> Iterator for GraphNodeIterator<T> {
    type Item = Link<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.queue.pop_front()?;

        if self.drain {
            for ptr in next.borrow_mut().pointers.drain(0..) {
                self.put_to_queue(&ptr);
            }
        } else {
            for ptr in next.borrow().pointers.iter() {
                self.put_to_queue(ptr);
            }
        };

        Some(next)
    }
}


fn destroy_all<T>(from_node: &Link<T>) {
    Node::breath_search_drain(from_node).count();
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        self.pointers.iter().for_each(destroy_all);
        self.pointers.clear();
    }
}

fn extract_item<T>(node: &Link<T>) -> Ref<'_, T> {
    Ref::map(node.borrow(), |b| &b.item)
}

fn extract_item_optional<T>(node: &Option<Link<T>>) -> Option<Ref<'_, T>> {
    node.as_ref().map(extract_item)
}

#[cfg(test)]
mod tests {
    use crate::chapter_6_graph::{destroy_all, extract_item, Node, NodeFactory};
    use std::rc::Rc;

    #[test]
    fn should_build_node() {
        let mut node_factory = NodeFactory::new();
        let node = node_factory.create_node(1);

        assert_eq!(node.borrow().item, 1);
        assert_eq!(Rc::strong_count(&node), 1);
    }

    #[test]
    fn should_build_graph() {
        let mut node_factory = NodeFactory::new();
        let vini = node_factory.create_node("Vinícius".to_string());
        let bibi = node_factory.create_node("Bianca".to_string());

        vini.borrow_mut().attach(&bibi);
        bibi.borrow_mut().attach(&vini);

        assert_eq!(
            vini.borrow()
                .pointers
                .iter()
                .map(|v| v.borrow().item.clone())
                .collect::<Vec<_>>(),
            vec!["Bianca"]
        );

        destroy_all(&vini);
    }

    #[test]
    fn should_destroy_all_graph() {
        let mut node_factory = NodeFactory::new();
        let vini = node_factory.create_node("Vinícius".to_string());
        let bibi = node_factory.create_node("Bianca".to_string());

        vini.borrow_mut().attach(&bibi);
        bibi.borrow_mut().attach(&vini);

        assert_eq!(Rc::strong_count(&vini), 2);
        assert_eq!(Rc::strong_count(&bibi), 2);

        destroy_all(&vini);

        assert_eq!(Rc::strong_count(&vini), 1);
        assert_eq!(Rc::strong_count(&bibi), 1);
    }

    #[test]
    fn should_iter_on_nodes() {
        let mut node_factory = NodeFactory::new();
        let vini = node_factory.create_node("Vinícius".to_string());
        let bibi = node_factory.create_node("Bianca".to_string());

        vini.borrow_mut().attach(&bibi);
        bibi.borrow_mut().attach(&vini);

        let mut iter = Node::breath_search_iterator(&vini);

        assert_eq!(iter.next().unwrap().borrow().item, "Vinícius");
        assert_eq!(iter.next().unwrap().borrow().item, "Bianca");
        assert!(iter.next().is_none());

        let mut iter = Node::breath_search_iterator(&vini);

        assert_eq!(iter.next().unwrap().borrow().item, "Vinícius");
        assert!(&iter.next().as_ref().map(extract_item).unwrap().contains("Bianca"));
        assert!(iter.next().is_none());
    }
}
