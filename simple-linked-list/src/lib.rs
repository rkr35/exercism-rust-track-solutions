#![warn(clippy::pedantic)]

use std::collections::VecDeque;

type HeapNode<T> = Box<Node<T>>;
type MaybeNode<T> = Option<HeapNode<T>>;

struct Node<T> {
    data: T,
    next: MaybeNode<T>,
}

impl<T> From<T> for HeapNode<T> {
    fn from(data: T) -> Self {
        Self::new(Node { data, next: None })
    }
}

pub struct SimpleLinkedList<T> {
    head: MaybeNode<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
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

    pub fn pop(&mut self) -> Option<T> {
        fn extract_and_clear<T>(node: &mut MaybeNode<T>) -> Option<T> {
            node.take().map(|n| n.data)
        }

        let mut current = self.head.as_mut()?;

        if current.next.is_none() {
            return extract_and_clear(&mut self.head);
        }

        loop {
            let is_penultimate = {
                let next = current.next.as_ref().unwrap();
                let next_next = &next.next;
                next_next.is_none()
            };

            if is_penultimate {
                return extract_and_clear(&mut current.next);
            }

            current = current.next.as_mut().unwrap();
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(mut self) -> Self {
        let mut reversed_list = Self::new();

        while let Some(node) = self.pop() {
            reversed_list.push(node);
        }

        reversed_list
    }
}

impl<T> From<&[T]> for SimpleLinkedList<T>
where
    T: Copy,
{
    fn from(slice: &[T]) -> Self {
        let mut list = Self::new();

        for element in slice {
            list.push(*element);
        }

        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut nodes = VecDeque::new();

        while let Some(node) = self.pop() {
            nodes.push_front(node);
        }

        nodes.into()
    }
}