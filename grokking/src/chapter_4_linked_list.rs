use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node<T> {
    item: Option<T>,
    prev: Option<NodeWeakRef<T>>,
    next: Option<NodeStrongRef<T>>,
}

type NodeStrongRef<T> = Rc<RefCell<Node<T>>>;
type NodeWeakRef<T> = Weak<RefCell<Node<T>>>;

impl<T> Into<NodeStrongRef<T>> for Node<T> {
    fn into(self) -> NodeStrongRef<T> {
        Rc::new(RefCell::new(self))
    }
}

impl<T> Node<T> {
    fn new(item: T) -> Node<T> {
        Node {
            item: Some(item),
            prev: None,
            next: None,
        }
    }

    fn pop(&mut self) -> (Option<NodeWeakRef<T>>, Option<T>, Option<NodeStrongRef<T>>) {
        let Node { item, prev, next } = self;

        let prev = if let Some(prev) = prev.take() {
            prev.upgrade()
        } else {
            None
        };

        match (&prev, &next) {
            (Some(prev_up), Some(next)) => {
                prev_up.borrow_mut().next = Some(Rc::clone(next));
                next.borrow_mut().prev = Some(Rc::downgrade(prev_up));
            }
            (Some(prev), None) => prev.borrow_mut().next = None,
            (None, Some(next)) => next.borrow_mut().prev = None,
            (None, None) => {}
        }

        (
            prev.map(|prev| Rc::downgrade(&prev)),
            item.take(),
            next.take(),
        )
    }

    fn append(self_ref: NodeStrongRef<T>, item: T) -> NodeWeakRef<T> {
        let mut self_ref_mut = self_ref.borrow_mut();
        let old_next = self_ref_mut.next.take();
        let new_cell = Node {
            item: Some(item),
            prev: Some(Rc::downgrade(&self_ref)),
            next: old_next,
        }
        .into();
        self_ref_mut.next = Some(Rc::clone(&new_cell));
        Rc::downgrade(&new_cell)
    }

    fn prepend(self_ref: NodeStrongRef<T>, item: T) -> NodeStrongRef<T> {
        let mut self_ref_mut = self_ref.borrow_mut();
        let old_prev = self_ref_mut.prev.take();
        let new_cell = Node {
            item: Some(item),
            next: Some(Rc::clone(&self_ref)),
            prev: old_prev,
        }
        .into();
        self_ref_mut.prev = Some(Rc::downgrade(&new_cell));
        new_cell
    }
}

pub struct LinkedList<T> {
    len: u64,
    first: Option<NodeStrongRef<T>>,
    last: Option<NodeWeakRef<T>>,
}

pub struct Drain<T>(LinkedList<T>);

impl<T> Drain<T> {
    fn new(list: LinkedList<T>) -> Self {
        Drain(list)
    }
}

impl<T> Iterator for Drain<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_first()
    }
}

impl<T> LinkedList<T> {
    pub fn empty() -> LinkedList<T> {
        LinkedList {
            len: 0,
            first: None,
            last: None,
        }
    }

    pub fn new(item: T) -> LinkedList<T> {
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
                let cell = Node::new(new_first).into();
                self.last = Some(Rc::downgrade(&cell));
                cell
            }
        });
        self.len += 1;
    }

    pub fn pop_first(&mut self) -> Option<T> {
        let (_, item, next) = self.first.take()?.borrow_mut().pop();
        self.first = next;
        self.len -= 1;
        item
    }

    pub fn add_last(&mut self, new_last: T) {
        self.last = Some(match self.last.take().and_then(|v| v.upgrade()) {
            Some(last) => Node::append(last, new_last),
            None => {
                let cell = Node::new(new_last).into();
                self.first = Some(Rc::clone(&cell));
                Rc::downgrade(&cell)
            }
        });
        self.len += 1;
    }

    pub fn pop_last(&mut self) -> Option<T> {
        let (prev, item, _) = self.last.take()?.upgrade()?.borrow_mut().pop();
        self.last = prev;
        self.len -= 1;
        item
    }

    pub fn len(&self) -> u64 {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_4_linked_list::LinkedList;

    #[test]
    fn should_add_first_and_pop_first() {
        let mut list = LinkedList::empty();

        list.add_first(2);
        list.add_first(1);

        assert_eq!(list.pop_first().unwrap(), 1);
        assert_eq!(list.pop_first().unwrap(), 2)
    }

    #[test]
    fn should_add_first_and_pop_last() {
        let mut list = LinkedList::empty();

        list.add_first(2);
        list.add_first(1);

        assert_eq!(list.pop_last().unwrap(), 2);
        assert_eq!(list.pop_last().unwrap(), 1)
    }

    #[test]
    fn should_add_last_and_pop_first() {
        let mut list = LinkedList::empty();

        list.add_last(1);
        list.add_last(2);

        assert_eq!(list.pop_first().unwrap(), 1);
        assert_eq!(list.pop_first().unwrap(), 2)
    }

    #[test]
    fn should_add_last_and_pop_last() {
        let mut list = LinkedList::empty();

        list.add_last(1);
        list.add_last(2);

        assert_eq!(list.pop_last().unwrap(), 2);
        assert_eq!(list.pop_last().unwrap(), 1)
    }

    #[test]
    fn should_count_len_after_adding() {
        let mut list = LinkedList::empty();

        list.add_last(1);
        list.add_last(2);

        assert_eq!(list.len(), 2);
    }

    #[test]
    fn should_count_len_after_popping() {
        let mut list = LinkedList::empty();

        list.add_last(1);
        list.add_last(2);
        list.pop_first();
        list.pop_last();

        assert_eq!(list.len(), 0);
    }

    #[test]
    fn should_drain() {
        let mut list = LinkedList::empty();

        list.add_last(1);
        list.add_last(2);

        let mut iter = list.drain();

        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next(), None);
    }
}
