use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node<T> {
    item: T,
    prev: Option<NodeWeakRef<T>>,
    next: Option<NodeStrongRef<T>>,
}

type NodeStrongRef<T> = Rc<RefCell<Node<T>>>;
type NodeWeakRef<T> = Weak<RefCell<Node<T>>>;

impl<T> Node<T> {
    fn new(item: T) -> NodeStrongRef<T> {
        Rc::new(RefCell::new(Node {
            item,
            prev: None,
            next: None,
        }))
    }

    fn append(self_ref: NodeStrongRef<T>, item: T) -> NodeWeakRef<T> {
        let mut self_ref_mut = self_ref.borrow_mut();
        let old_next = self_ref_mut.next.take();

        let new_cell = Node::new(item);
        let mut new_cell_borrow = new_cell.borrow_mut();
        new_cell_borrow.prev = Some(Rc::downgrade(&self_ref));
        new_cell_borrow.next = old_next;

        self_ref_mut.next = Some(Rc::clone(&new_cell));
        Rc::downgrade(&new_cell)
    }

    fn prepend(self_ref: NodeStrongRef<T>, item: T) -> NodeStrongRef<T> {
        let mut self_ref_mut = self_ref.borrow_mut();
        let old_prev = self_ref_mut.prev.take();

        let new_cell = Node::new(item);
        let mut new_cell_borrow = new_cell.borrow_mut();
        new_cell_borrow.next = Some(Rc::clone(&self_ref));
        new_cell_borrow.prev = old_prev;

        self_ref_mut.prev = Some(Rc::downgrade(&new_cell));
        Rc::clone(&new_cell)
    }
}

pub struct Deque<T> {
    first: Option<NodeStrongRef<T>>,
    last: Option<NodeWeakRef<T>>,
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while self.pop_first().is_some() {}
    }
}

pub struct Drain<T>(Deque<T>);

impl<T> Drain<T> {
    fn new(list: Deque<T>) -> Self {
        Drain(list)
    }
}

impl<T> Iterator for Drain<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_first()
    }
}

impl<T> DoubleEndedIterator for Drain<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_last()
    }
}

impl<T> Deque<T> {
    pub fn empty() -> Deque<T> {
        Deque {
            first: None,
            last: None,
        }
    }

    pub fn new(item: T) -> Deque<T> {
        let mut list = Self::empty();
        list.add_first(item);
        list
    }

    pub fn drain(self) -> Drain<T> {
        Drain::new(self)
    }

    pub fn add_first(&mut self, new_first: T) {
        self.first = Some(match self.first.take() {
            Some(first) => Node::prepend(first, new_first),
            None => {
                let cell = Node::new(new_first);
                self.last = Some(Rc::downgrade(&cell));
                cell
            }
        });
    }

    pub fn pop_first(&mut self) -> Option<T> {
        match self.first.take() {
            None => None,
            Some(node_strong_ref) => {
                self.first = node_strong_ref.borrow_mut().next.take();
                Some(Deque::extract_strong_ref_item(node_strong_ref))
            }
        }
    }

    pub fn add_last(&mut self, new_last: T) {
        self.last = Some(match self.last.take().and_then(|v| v.upgrade()) {
            Some(last) => Node::append(last, new_last),
            None => {
                let cell = Node::new(new_last);
                self.first = Some(Rc::clone(&cell));
                Rc::downgrade(&cell)
            }
        });
    }

    pub fn pop_last(&mut self) -> Option<T> {
        self.last.take().map(|node_weak_ref| {
            let node_strong_ref = Weak::upgrade(&node_weak_ref).unwrap();
            let previous = node_strong_ref.borrow_mut().prev.take();

            if let Some(previous_weak_ref) = previous {
                if let Some(previous_strong_ref) = Weak::upgrade(&previous_weak_ref) {
                    previous_strong_ref.borrow_mut().next.take();
                    self.last = Some(Rc::downgrade(&previous_strong_ref));
                    return Deque::extract_strong_ref_item(node_strong_ref)
                }
            }

            drop(node_strong_ref);
            self
                .first
                .take()
                .map(Deque::extract_strong_ref_item)
                .expect("Should have found head to be the last, because there arent previous references to last")
        })
    }

    fn extract_strong_ref_item(strong_ref: NodeStrongRef<T>) -> T {
        Rc::into_inner(strong_ref)
            .expect("Tried to extract value from Rc with multiple strong pointers")
            .into_inner()
            .item
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_4_deque::Deque;

    #[test]
    fn should_add_first_and_pop_first() {
        let mut list = Deque::empty();

        list.add_first(2);
        list.add_first(1);

        assert_eq!(list.pop_first().unwrap(), 1);
        assert_eq!(list.pop_first().unwrap(), 2)
    }

    #[test]
    fn should_add_first_and_pop_last() {
        let mut list = Deque::empty();

        list.add_first(2);
        list.add_first(1);

        assert_eq!(list.pop_last().unwrap(), 2);
        assert_eq!(list.pop_last().unwrap(), 1)
    }

    #[test]
    fn should_add_last_and_pop_first() {
        let mut list = Deque::empty();

        list.add_last(1);
        list.add_last(2);

        assert_eq!(list.pop_first().unwrap(), 1);
        assert_eq!(list.pop_first().unwrap(), 2)
    }

    #[test]
    fn should_add_last_and_pop_last() {
        let mut list = Deque::empty();

        list.add_last(1);
        list.add_last(2);

        assert_eq!(list.pop_last().unwrap(), 2);
        assert_eq!(list.pop_last().unwrap(), 1)
    }

    #[test]
    fn should_drain() {
        let mut list = Deque::empty();

        list.add_last(1);
        list.add_last(2);

        let mut iter = list.drain();

        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 2);
        assert!(iter.next().is_none());
    }

    #[test]
    fn should_drain_rev() {
        let mut list = Deque::empty();

        list.add_last(1);
        list.add_last(2);
        list.add_last(3);

        let mut iter = list.drain();
        assert_eq!(iter.next().unwrap(), 1);

        let mut iter = iter.rev();
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next().unwrap(), 2);

        assert!(iter.next().is_none());
    }
}
