use super::{Node, SimpleLinkedList};

fn ref_option_box_as_option_ref<T>(option_box: &Option<Box<T>>) -> Option<&T> {
    option_box
        .as_ref() // Option<&Box<T>>
        .map(
            |boxed|    // &Box<T>
                &       // &T
                *       // T
                *boxed, // Box<T>
        )
}

fn ref_mut_option_box_as_option_ref<T>(option_box: &mut Option<Box<T>>) -> Option<&mut T> {
    option_box
        .as_mut() // Option<&mut Box<T>>
        .map(
            |boxed|    // &mut Box<T>
                &mut    // &mut T
                *       // mut T
                *boxed, // mut Box<T>
        )
}

pub struct Immutable<'a, T> {
    current_node: Option<&'a Node<T>>,
}

pub struct Mutable<'a, T> {
    current_node: Option<&'a mut Node<T>>, // None
}

impl<'a, T> Iterator for Immutable<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current_node.take()?;
        self.current_node = ref_option_box_as_option_ref(&node.next);
        Some(&node.data)
    }
}

impl<'a, T> Iterator for Mutable<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current_node.take()?;
        self.current_node = ref_mut_option_box_as_option_ref(&mut node.next);
        Some(&mut node.data)
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn iter(&self) -> Immutable<T> {
        Immutable::<T> {
            current_node: ref_option_box_as_option_ref(&self.head),
        }
    }

    pub fn iter_mut(&mut self) -> Mutable<T> {
        Mutable::<T> {
            current_node: ref_mut_option_box_as_option_ref(&mut self.head),
        }
    }
}
