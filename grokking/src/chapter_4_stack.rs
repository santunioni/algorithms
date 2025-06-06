struct Node<T> {
    item: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn pop_next(&mut self) -> Link<T> {
        let to_delete = self.next.take();
        match to_delete {
            None => None,
            Some(mut to_delete) => {
                let new_next = to_delete.next.take();
                self.next = new_next;
                Some(to_delete)
            }
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Stack<T> {
    head: Link<T>,
}

impl<T> Stack<T> {
    pub fn new(with_value: T) -> Self {
        Stack {
            head: Some(Box::new(Node {
                item: with_value,
                next: None,
            })),
        }
    }

    pub fn create_many(how_many: usize) -> Vec<Stack<T>> {
        (0..how_many).map(|_| Stack::empty()).collect()
    }

    pub fn empty() -> Self {
        Stack { head: None }
    }

    fn pop_head_node(&mut self) -> Link<T> {
        self.head.take().map(|mut old_head| {
            let new_head = old_head.next.take();
            self.head = new_head;
            old_head
        })
    }

    pub(crate) fn peek_head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    fn peek_head_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.item)
    }

    pub fn prepend(&mut self, item: T) {
        self.head = Some(Box::new(Node {
            item,
            next: self.head.take(),
        }));
    }

    pub fn pop_head(&mut self) -> Option<T> {
        self.pop_head_node().map(|v| v.item)
    }

    pub fn drain(self) -> StackDrain<T> {
        StackDrain(self)
    }

    pub fn iter(&self) -> StackIter<T> {
        StackIter(self.head.as_ref().map(|box_ref| box_ref.as_ref()))
    }

    pub fn iter_mut(&mut self) -> StackIterMut<T> {
        StackIterMut(self.head.as_mut().map(|box_ref| box_ref.as_mut()))
    }

    pub fn contains<F: FnMut(&T) -> bool>(&self, check: F) -> bool {
        self.iter().any(check)
    }

    fn find_node_mut<F: Fn(&Node<T>) -> bool>(&mut self, check: F) -> Option<&mut Node<T>> {
        let mut cursor = self.head.as_mut();
        while let Some(node) = cursor {
            if check(node) {
                return Some(node);
            }
            cursor = node.next.as_mut();
        }
        None
    }

    pub fn remove_by<F: Fn(&T) -> bool>(&mut self, check: F) -> Option<T> {
        if let Some(head) = &mut self.head {
            if check(&head.item) {
                return self.pop_head();
            }
        }

        let node_previous_to_the_one = self.find_node_mut(|node| match &node.next {
            None => false,
            Some(next) => check(&next.item),
        })?;
        let mut the_one_node_to_remove = node_previous_to_the_one.next.take()?;
        node_previous_to_the_one.next = the_one_node_to_remove.next.take();
        Some(the_one_node_to_remove.item)
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        while self.pop_head_node().is_some() {}
    }
}

pub struct StackDrain<T>(Stack<T>);

impl<T> Iterator for StackDrain<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_head()
    }
}

pub struct StackIter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for StackIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|cursor| {
            let box_ref_option = cursor.next.as_ref();
            self.0 = box_ref_option.map(|box_ref| box_ref.as_ref());
            &cursor.item
        })
    }
}

pub struct StackIterMut<'a, T>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for StackIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|cursor| {
            let box_ref_option = cursor.next.as_mut();
            self.0 = box_ref_option.map(|box_ref| box_ref.as_mut());
            &mut cursor.item
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_4_stack::Stack;

    #[test]
    fn should_add_first_and_pop_first() {
        let mut stack = Stack::empty();

        stack.prepend(2);
        stack.prepend(1);

        assert_eq!(stack.peek_head().unwrap(), &1);
        assert_eq!(stack.pop_head().unwrap(), 1);
        assert_eq!(stack.pop_head().unwrap(), 2)
    }

    #[test]
    fn should_peek_head() {
        let mut stack = Stack::empty();

        stack.prepend(2);
        stack.prepend(1);

        assert_eq!(stack.peek_head().unwrap(), &1);
    }

    #[test]
    fn should_mutate_head() {
        let mut stack = Stack::empty();

        stack.prepend(2);
        stack.prepend(1);
        if let Some(ptr) = stack.peek_head_mut() {
            *ptr = 50
        }

        assert_eq!(stack.peek_head().unwrap(), &50);
    }

    #[test]
    fn should_drain_stack() {
        let mut stack = Stack::empty();

        stack.prepend(3);
        stack.prepend(2);
        stack.prepend(1);

        assert_eq!(stack.drain().collect::<Vec<i32>>(), vec![1, 2, 3])
    }

    #[test]
    fn should_iter_on_stack() {
        let mut stack = Stack::empty();

        stack.prepend(3);
        stack.prepend(2);
        stack.prepend(1);

        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn should_iter_mut_on_stack() {
        let mut stack = Stack::empty();

        stack.prepend(2);
        stack.prepend(1);

        let mut iter = stack.iter_mut();
        *iter.next().unwrap() = 50;
        *iter.next().unwrap() = 100;
        assert_eq!(stack.pop_head(), Some(50));
        assert_eq!(stack.pop_head(), Some(100));
    }

    #[test]
    fn should_find_item() {
        let mut stack = Stack::empty();

        stack.prepend(2);
        stack.prepend(1);

        assert!(stack.contains(|item| *item == 1));
        assert!(!stack.contains(|item| *item == 3));
    }

    #[test]
    fn should_remove_item() {
        let mut stack = Stack::empty();

        stack.prepend(3);
        stack.prepend(2);
        stack.prepend(1);

        stack.remove_by(|v| *v == 2);

        assert_eq!(stack.pop_head(), Some(1));
        assert_eq!(stack.pop_head(), Some(3));
        assert_eq!(stack.pop_head(), None);
    }

    #[test]
    fn should_remove_head() {
        let mut stack = Stack::empty();

        stack.prepend(2);
        stack.prepend(1);

        stack.remove_by(|v| *v == 1);

        assert_eq!(stack.pop_head(), Some(2));
        assert_eq!(stack.pop_head(), None);
    }
}
