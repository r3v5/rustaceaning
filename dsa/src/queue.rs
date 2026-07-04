use crate::node::Node;

pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    size: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: std::ptr::null_mut(),
            size: 0,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        let mut node = Box::new(Node {
            value,
            next: None,
        });
        let raw: *mut Node<T> = &mut *node;

        if self.tail.is_null() {
            self.head = Some(node);
        } else {
            unsafe {
                (*self.tail).next = Some(node);
            }
        }
        self.tail = raw;
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }
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

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_and_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn peek() {
        let mut queue = Queue::new();
        assert_eq!(queue.peek(), None);
        queue.enqueue(42);
        assert_eq!(queue.peek(), Some(&42));
        queue.enqueue(99);
        assert_eq!(queue.peek(), Some(&42));
    }

    #[test]
    fn len_and_empty() {
        let mut queue = Queue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        queue.enqueue("a");
        queue.enqueue("b");
        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 2);
        queue.dequeue();
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn fifo_order() {
        let mut queue = Queue::new();
        queue.enqueue("first");
        queue.enqueue("second");
        queue.enqueue("third");
        assert_eq!(queue.dequeue(), Some("first"));
        assert_eq!(queue.dequeue(), Some("second"));
        assert_eq!(queue.dequeue(), Some("third"));
    }
}
