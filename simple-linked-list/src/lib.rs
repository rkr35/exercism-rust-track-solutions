#![warn(clippy::pedantic)]

type HeapNode<T> = Box<Node<T>>;
type MaybeNode<T> = Option<HeapNode<T>>;

struct Node<T> {
    data: T,
    next: MaybeNode<T>,
}

impl<T> From<T> for Node<T> {
    fn from(data: T) -> Self {
        Self { data, next: None }
    }
}

pub struct SimpleLinkedList<T> {
    head: MaybeNode<T>,
    length: usize,
}

mod iterator;

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    // O(1) because &self maintains length.
    pub fn len(&self) -> usize {
        self.length
    }

    // O(1) because of self.len()
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // O(1) because &mut self has direct access to self.head and pushing
    // just updates self.head.
    pub fn push(&mut self, data: T) {
        self.head = Some(Box::new(Node {
            data,
            next: self.head.take(),
        }));
        self.length += 1;
    }

    // O(1) because &mut self has direct access to self.head.
    pub fn pop(&mut self) -> Option<T> {
        let popped = self.head.take()?;
        self.head = popped.next;
        self.length -= 1;
        Some(popped.data)
    }

    // O(1) since &self has direct access to self.head.
    pub fn peek(&self) -> Option<&T> {
        self.iter().nth(0)
    }

    // O(n) since self.pop() is O(1), and we are calling self.pop() n times (once per element).
    // Also O(n) in space, since we create and allocate elements in a new list.
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
    // O(s), where s is the length of the slice.
    // Self.push() is O(1), and we are calling this method s times, so O(s).
    fn from(slice: &[T]) -> Self {
        let mut list = Self::new();

        for element in slice {
            list.push(*element);
        }

        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    // O(n) for the same reason as self.rev()
    fn into(mut self) -> Vec<T> {
        // self.into_iter().map(|node| node.data).collect()
        let mut nodes = std::collections::VecDeque::new();

        while let Some(node) = self.pop() {
            nodes.push_front(node);
        }

        nodes.into()
    }
}
