#![warn(clippy::pedantic)]
use std::rc::Rc;

type HeapNode<T> = Rc<Node<T>>;
type MaybeNode<T> = Option<HeapNode<T>>;

struct Node<T> {
    data: T,
    next: MaybeNode<T>,
}

pub struct SimpleLinkedList<T> {
    first_node: MaybeNode<T>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            first_node: None,
        }
    }

    pub fn len(&self) -> usize {
        0
    }
}