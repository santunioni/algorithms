struct Node<T> {
    item: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Stack<T> {
    head: Link<T>,
}

impl<T> Stack<T> {
    pub fn empty() -> Self {
        Stack { head: None }
    }

    fn pop_head_node(&mut self) -> Link<T> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.next.take();
            old_head
        })
    }

    fn peek_head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    fn peek_head_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.item)
    }

    pub fn push_head(&mut self, item: T) {
        self.head = Some(Box::new(Node {
            item,
            next: self.pop_head_node(),
        }));
    }

    pub fn pop_head(&mut self) -> Option<T> {
        self.pop_head_node().map(|v| v.item)
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        while self.pop_head_node().is_some() {}
    }
}

pub struct IntoIter<T>(Stack<T>);

impl<T> Stack<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_head()
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_4_stack::Stack;

    #[test]
    fn should_add_first_and_pop_first() {
        let mut stack = Stack::empty();

        stack.push_head(2);
        stack.push_head(1);

        assert_eq!(stack.peek_head().unwrap(), &1);
        assert_eq!(stack.pop_head().unwrap(), 1);
        assert_eq!(stack.pop_head().unwrap(), 2)
    }

    #[test]
    fn should_peek_head() {
        let mut stack = Stack::empty();

        stack.push_head(2);
        stack.push_head(1);

        assert_eq!(stack.peek_head().unwrap(), &1);
    }

    #[test]
    fn should_mutate_head() {
        let mut stack = Stack::empty();

        stack.push_head(2);
        stack.push_head(1);

        stack.peek_head_mut().map(|ptr| *ptr = 50);
        assert_eq!(stack.peek_head().unwrap(), &50);
    }

    #[test]
    fn should_drain_stack() {
        let mut stack = Stack::empty();

        stack.push_head(2);
        stack.push_head(1);

        assert_eq!(stack.into_iter().collect::<Vec<i32>>(), vec![1, 2])
    }
}
