use std::cell::RefCell;
use std::rc::Rc;

struct LinkedItem<T> {
    item: Option<T>,
    prev: Option<LinkedItemRef<T>>,
    next: Option<LinkedItemRef<T>>,
}

type LinkedItemRef<T> = Rc<RefCell<LinkedItem<T>>>;

impl<T> Into<LinkedItemRef<T>> for LinkedItem<T> {
    fn into(self) -> LinkedItemRef<T> {
        Rc::new(RefCell::new(self))
    }
}

impl<T> LinkedItem<T> {
    fn pop(
        &mut self,
    ) -> (
        Option<LinkedItemRef<T>>,
        Option<T>,
        Option<LinkedItemRef<T>>,
    ) {
        let LinkedItem { item, prev, next } = self;

        match (&prev, &next) {
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next = Some(Rc::clone(next));
                next.borrow_mut().prev = Some(Rc::clone(prev));
            }
            (Some(prev), None) => prev.borrow_mut().next = None,
            (None, Some(next)) => next.borrow_mut().prev = None,
            (None, None) => {}
        }

        (prev.take(), item.take(), next.take())
    }
}

pub struct LinkedList<T> {
    len: u64,
    first: Option<LinkedItemRef<T>>,
    last: Option<LinkedItemRef<T>>,
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
        let old_first_now_second = self.first.take();

        let new_first = LinkedItem {
            item: Some(new_first),
            next: match &old_first_now_second {
                None => None,
                Some(linked_item_ref) => Some(Rc::clone(linked_item_ref)),
            },
            prev: None,
        }
        .into();

        if let Some(linked_item_ref) = old_first_now_second {
            linked_item_ref.borrow_mut().prev = Some(Rc::clone(&new_first));
        } else {
            self.last = Some(Rc::clone(&new_first))
        }

        self.first = Some(Rc::clone(&new_first));
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
        let old_last = self.last.take();

        let new_last = LinkedItem {
            item: Some(new_last),
            next: None,
            prev: match &old_last {
                None => None,
                Some(linked_item_ref) => Some(Rc::clone(linked_item_ref)),
            },
        }
        .into();

        if let Some(linked_item_ref) = old_last {
            linked_item_ref.borrow_mut().next = Some(Rc::clone(&new_last));
        } else {
            self.first = Some(Rc::clone(&new_last));
        }

        self.last = Some(Rc::clone(&new_last));
        self.len += 1;
    }

    pub fn pop_last(&mut self) -> Option<T> {
        let last = self.last.take()?;
        let (prev, item, _) = last.borrow_mut().pop();
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
