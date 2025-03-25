struct SinglyLinkedNode<T> {
    item: T,
    next: Link<T>,
}

type Link<T> = Option<Box<SinglyLinkedNode<T>>>;

pub struct SinglyLinkedList<T> {
    head: Link<T>,
}

impl<T> SinglyLinkedList<T> {
    pub fn empty() -> SinglyLinkedList<T> {
        SinglyLinkedList { head: None }
    }

    pub fn new(item: T) -> SinglyLinkedList<T> {
        let mut list = Self::empty();
        list.push_head(item);
        list
    }

    fn pop_head_node(&mut self) -> Link<T> {
        match self.head.take() {
            None => None,
            Some(mut head) => {
                self.head = head.next.take();
                Some(head)
            }
        }
    }

    fn peek_head(&self) -> Option<&T> {
        self.head.as_ref().map(|v| &v.item)
    }

    pub fn push_head(&mut self, item: T) {
        self.head = Some(Box::new(SinglyLinkedNode {
            item,
            next: self.pop_head_node(),
        }));
    }

    pub fn pop_head(&mut self) -> Option<T> {
        self.pop_head_node().take().map(|v| v.item)
    }
}

impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_head_node().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_4_singly_linked_list::SinglyLinkedList;

    #[test]
    fn should_add_first_and_pop_first() {
        let mut list = SinglyLinkedList::empty();

        list.push_head(2);
        list.push_head(1);

        assert_eq!(list.peek_head().unwrap(), &1);
        assert_eq!(list.pop_head().unwrap(), 1);
        assert_eq!(list.pop_head().unwrap(), 2)
    }
}
