use crate::chapter_5_hashset::HashSet;
use std::cell::{Ref, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;

pub struct GraphNode<T> {
    id: u64,
    item: T,
    legs: Vec<Leg<T>>,
}

type Link<T> = Rc<RefCell<GraphNode<T>>>;

struct Leg<T> {
    node: Link<T>,
    weight: u64,
}

impl<T> Leg<T> {
    fn new_weighted(node: Link<T>, weight: u64) -> Self {
        Leg { node, weight }
    }
}

impl<T> GraphNode<T> {
    fn attach(&mut self, other: &Link<T>) {
        self.attach_weighted(other, 1)
    }

    fn attach_weighted(&mut self, other: &Link<T>, weight: u64) {
        self.legs.push(Leg::new_weighted(Rc::clone(other), weight))
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
        let node = Rc::new(RefCell::new(GraphNode {
            id: self.id,
            item,
            legs: vec![],
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
            for ptr in next.borrow_mut().legs.drain(0..) {
                self.put_to_queue(&ptr.node);
            }
        } else {
            for ptr in next.borrow().legs.iter() {
                self.put_to_queue(&ptr.node);
            }
        };

        Some(next)
    }
}

fn destroy_all<T>(from_node: &Link<T>) {
    GraphNode::breath_search_drain(from_node).count();
}

impl<T> Drop for GraphNode<T> {
    fn drop(&mut self) {
        self.legs.iter().map(|v| &v.node).for_each(destroy_all);
        self.legs.clear();
    }
}

fn extract_item<T>(node: &Link<T>) -> Ref<'_, T> {
    Ref::map(node.borrow(), |b| &b.item)
}

fn extract_item_optional<T>(node: &Option<Link<T>>) -> Option<Ref<'_, T>> {
    node.as_ref().map(extract_item)
}

struct GraphItemIterator<T> {
    graph_node_iterator: GraphNodeIterator<T>,
    cursor: Option<Link<T>>,
}

impl<T> GraphItemIterator<T> {
    fn new(graph_node_iterator: GraphNodeIterator<T>) -> Self {
        GraphItemIterator {
            cursor: None,
            graph_node_iterator,
        }
    }

    fn fetch(&mut self) -> Option<Ref<T>> {
        self.cursor = self.graph_node_iterator.next();
        let Some(ptr) = &self.cursor else { return None };
        Some(Ref::map(ptr.borrow(), |b| &b.item))
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_6_graph::{
        GraphItemIterator, GraphNode, NodeFactory, destroy_all, extract_item,
    };
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
                .legs
                .iter()
                .map(|v| v.node.borrow().item.clone())
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

        let mut iter = GraphNode::breath_search_iterator(&vini);

        assert_eq!(iter.next().unwrap().borrow().item, "Vinícius");
        assert_eq!(iter.next().unwrap().borrow().item, "Bianca");
        assert!(iter.next().is_none());

        let mut iter = GraphNode::breath_search_iterator(&vini);

        assert_eq!(iter.next().unwrap().borrow().item, "Vinícius");
        assert!(
            &iter
                .next()
                .as_ref()
                .map(extract_item)
                .unwrap()
                .contains("Bianca")
        );
        assert!(iter.next().is_none());

        for node in GraphNode::breath_search_iterator(&vini) {
            let item = &node.borrow().item;
            println!("{}", item)
        }

        let mut iter_util = GraphItemIterator::new(GraphNode::breath_search_iterator(&vini));
        assert!(iter_util.fetch().unwrap().contains("Vinícius"));
        assert!(&iter_util.fetch().unwrap().contains("Bianca"));
        assert!(iter.next().is_none());
    }
}
