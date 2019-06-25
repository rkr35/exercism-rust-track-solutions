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
    length: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    // O(1) because all n elements of &self must be iterated.
    pub fn len(&self) -> usize {
        self.length
    }

    // O(1) because of self.len()
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // O(n) because all n elements of &mut self must be iterated.
    fn last_mut(&mut self) -> Option<&mut HeapNode<T>> {
        let mut current = self.head.as_mut()?;

        loop {
            if current.next.is_none() {
                return Some(current);
            }

            current = current.next.as_mut().unwrap();
        }
    }

    // O(n) because of self.last_mut()
    pub fn push(&mut self, data: T) {
        let new_node = Some(HeapNode::from(data));

        match self.last_mut() {
            Some(l) => l.next = new_node,
            None => self.head = new_node,
        };

        self.length += 1;
    }

    // O(n) because all n elements of &mut self must be iterated.
    pub fn pop(&mut self) -> Option<T> {
        fn extract_and_clear<T>(node: &mut MaybeNode<T>) -> Option<T> {
            node.take().map(|n| n.data)
        }

        let mut current = self.head.as_mut()?;

        self.length -= 1;

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

    // O(1) since &self owns the head resource.
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    // O(n^2) since self.pop() is O(n), and we are calling self.pop() n times (once per element).
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
    // O(s^2), where s is the length of the slice.
    // Self.push() is O(s), and we are calling this method s times, so O(s^2).
    fn from(slice: &[T]) -> Self {
        let mut list = Self::new();

        for element in slice {
            list.push(*element);
        }

        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    // O(n^2) for the same reason as self.rev()
    fn into(mut self) -> Vec<T> {
        let mut nodes = VecDeque::new();

        while let Some(node) = self.pop() {
            nodes.push_front(node);
        }

        nodes.into()
    }
}
