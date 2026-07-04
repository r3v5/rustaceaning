use crate::node::Node;

pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn print(&self)
    where
        T: std::fmt::Display,
    {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("[{}]", node.value);
            if node.next.is_some() {
                print!(" -> ");
            }
            current = &node.next;
        }
        println!();
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        stack.push(99);
        assert_eq!(stack.peek(), Some(&99));
    }

    #[test]
    fn len_and_empty() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        stack.push("a");
        stack.push("b");
        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 2);
        stack.pop();
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn works_with_strings() {
        let mut stack = Stack::new();
        stack.push(String::from("hello"));
        stack.push(String::from("world"));
        assert_eq!(stack.pop(), Some(String::from("world")));
        assert_eq!(stack.peek(), Some(&String::from("hello")));
    }
}
