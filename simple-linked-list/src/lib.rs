#![warn(clippy::pedantic)]

type HeapNode<T> = Box<Node<T>>;
type MaybeNode<T> = Option<HeapNode<T>>;

struct Node<T> {
    data: T,
    next: MaybeNode<T>,
}

impl<T> From<T> for HeapNode<T> {
    fn from(data: T) -> Self {
        Self::new(Node {
            data,
            next: None
        })
    }
}

pub struct SimpleLinkedList<T> {
    head: MaybeNode<T>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut node = &self.head;

        while let Some(n) = node {
            length += 1;
            node = &n.next;
        }

        length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn last_mut(&mut self) -> Option<&mut HeapNode<T>> {
        let mut current = self.head.as_mut()?;

        loop {
            if current.next.is_none() {
                return Some(current);
            }

            current = current.next.as_mut().unwrap();
        }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Some(HeapNode::from(data));

        match self.last_mut() {
            Some(l) => l.next = new_node,
            None => self.head = new_node,
        };
    }
}