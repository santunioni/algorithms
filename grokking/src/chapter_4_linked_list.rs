use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Cell<T> {
    item: Option<T>,
    prev: Option<CellWeakRef<T>>,
    next: Option<CellStrongRef<T>>,
}

type CellStrongRef<T> = Rc<RefCell<Cell<T>>>;
type CellWeakRef<T> = Weak<RefCell<Cell<T>>>;

impl<T> Into<CellStrongRef<T>> for Cell<T> {
    fn into(self) -> CellStrongRef<T> {
        Rc::new(RefCell::new(self))
    }
}

impl<T> Cell<T> {
    fn new(item: T) -> Cell<T> {
        Cell {
            item: Some(item),
            prev: None,
            next: None,
        }
    }

    fn pop(&mut self) -> (Option<CellWeakRef<T>>, Option<T>, Option<CellStrongRef<T>>) {
        let Cell { item, prev, next } = self;

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

    fn append(self_ref: CellStrongRef<T>, item: T) -> CellStrongRef<T> {
        let mut self_ref_mut = self_ref.borrow_mut();
        let old_next = self_ref_mut.next.take();
        let new_cell = Cell {
            item: Some(item),
            prev: Some(Rc::downgrade(&self_ref)),
            next: old_next,
        }
        .into();
        self_ref_mut.next = Some(Rc::clone(&new_cell));
        new_cell
    }

    fn prepend(self_ref: CellStrongRef<T>, item: T) -> CellStrongRef<T> {
        let mut self_ref_mut = self_ref.borrow_mut();
        let old_prev = self_ref_mut.prev.take();
        let new_cell = Cell {
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
    first: Option<CellStrongRef<T>>,
    last: Option<CellWeakRef<T>>,
}

pub struct Drain<T>(LinkedList<T>);

impl<T> Drain<T> {
    fn new(list: LinkedList<T>) -> Self {
        Drain(list)
    }

    fn rev(self) -> DrainRev<T> {
        DrainRev::new(self.0)
    }
}

impl<T> Iterator for Drain<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_first()
    }
}

pub struct DrainRev<T>(LinkedList<T>);

impl<T> DrainRev<T> {
    fn new(list: LinkedList<T>) -> Self {
        DrainRev(list)
    }

    fn rev(self) -> Drain<T> {
        Drain::new(self.0)
    }
}

impl<T> Iterator for DrainRev<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_last()
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

    pub fn drain(self) -> Drain<T> {
        Drain::new(self)
    }

    pub fn add_first(&mut self, new_first: T) {
        self.first = if let Some(first) = self.first.take() {
            Some(Cell::prepend(first, new_first))
        } else {
            let cell = Cell::new(new_first).into();
            self.last = Some(Rc::downgrade(&cell));
            Some(cell)
        };
        self.len += 1;
    }

    pub fn pop_first(&mut self) -> Option<T> {
        let first = self.first.take()?;
        let (_, item, next) = first.borrow_mut().pop();
        self.first = next;
        self.len -= 1;
        item
    }

    pub fn add_last(&mut self, new_last: T) {
        self.last = if let Some(last) = self.last.take().and_then(|v| v.upgrade()) {
            Some(Rc::downgrade(&Cell::append(last, new_last)))
        } else {
            let cell = Cell::new(new_last).into();
            self.first = Some(Rc::clone(&cell));
            Some(Rc::downgrade(&cell))
        };
        self.len += 1;
    }

    pub fn pop_last(&mut self) -> Option<T> {
        let last = self.last.take()?;
        let (prev, item, _) = last.upgrade()?.borrow_mut().pop();
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

    #[test]
    fn should_drain_rev() {
        let mut list = LinkedList::empty();

        list.add_last(1);
        list.add_last(2);

        let mut iter = list.drain().rev();

        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next(), None);
    }
}
