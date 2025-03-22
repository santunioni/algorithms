struct LinkedItem<T> {
    item: T,
    next: Option<Box<LinkedItem<T>>>,
}

struct LinkedList<T> {
    len: usize,
    first: Option<LinkedItem<T>>,
}

impl<T> LinkedList<T> {
    fn empty() -> LinkedList<T> {
        LinkedList {
            len: 0,
            first: None,
        }
    }

    fn add_first(&mut self, new_first: T) {
        self.first = Some(LinkedItem {
            item: new_first,
            next: self.first.take().map(Box::new),
        });
    }

    fn pop_first(&mut self) -> Option<T> {
        let mut first = self.first.take()?;
        if let Some(second) = first.next.take() {
            self.first = Some(*second);
        }
        Some(first.item)
    }

    fn add_last(&mut self, new_last: T) {}

    fn pop_last(&mut self) -> Option<T> {
        None
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
}
