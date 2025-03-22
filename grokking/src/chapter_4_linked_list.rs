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
        match self.first.take() {
            None => None,
            Some(mut old_first) => {
                self.first = match old_first.next.take() {
                    None => None,
                    Some(old_second) => Some(*old_second),
                };
                Some(old_first.item)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_4_linked_list::LinkedList;

    #[test]
    fn should_add_first_item() {
        let mut list = LinkedList::empty();

        list.add_first(1);
        list.add_first(2);

        assert_eq!(list.pop_first().unwrap(), 2);
        assert_eq!(list.pop_first().unwrap(), 1)
    }
}
