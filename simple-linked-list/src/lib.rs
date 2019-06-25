#![warn(clippy::pedantic)]

type HeapNode<T> = Box<Node<T>>;
type MaybeNode<T> = Option<HeapNode<T>>;

struct Node<T> {
    data: T,
    next: MaybeNode<T>,
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
}