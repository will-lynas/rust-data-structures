use std::fmt::{self, Display, Formatter};

use thiserror::Error;

type NodePointer<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: NodePointer<T>,
}

#[derive(Error, Debug, PartialEq)]
pub enum LinkedListError {
    #[error("The requested index is out of bounds")]
    OutOfBounds,
}

pub struct LinkedList<T> {
    head: NodePointer<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current: self.head.as_deref(),
        }
    }

    pub fn push(&mut self, value: T) {
        let node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn insert(&mut self, index: usize, value: T) -> Result<(), LinkedListError> {
        if index == 0 {
            self.push(value);
            return Ok(());
        }

        let mut current = &mut self.head;
        let mut count = 0;

        while let Some(node) = current {
            if count + 1 == index {
                let new_node = Box::new(Node {
                    value,
                    next: node.next.take(),
                });
                node.next = Some(new_node);
                return Ok(());
            }
            count += 1;
            current = &mut node.next;
        }

        Err(LinkedListError::OutOfBounds)
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;

        while let Some(node) = current {
            write!(f, "{} -> ", node.value)?;
            current = &node.next;
        }

        write!(f, "None")
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut list = LinkedList::new();
        for value in vec.into_iter().rev() {
            list.push(value);
        }
        list
    }
}

pub struct LinkedListIterator<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref();
            &node.value
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.to_string(), "None");
    }

    #[test]
    fn push() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.to_string(), "3 -> 2 -> 1 -> None");
    }

    #[test]
    fn pop() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.to_string(), "1 -> None");
    }

    #[test]
    fn pop_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn insert_at_head() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(1);
        list.push(2);
        list.insert(0, 3).unwrap();
        assert_eq!(list.to_string(), "3 -> 2 -> 1 -> None");
    }

    #[test]
    fn insert_in_middle() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(1);
        list.push(3);
        list.insert(1, 2).unwrap();
        assert_eq!(list.to_string(), "3 -> 2 -> 1 -> None");
    }

    #[test]
    fn insert_out_of_bounds() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(1);
        let result = list.insert(10, 2);
        assert_eq!(result, Err(LinkedListError::OutOfBounds));
    }

    #[test]
    fn insert_at_tail() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(2);
        list.push(1);
        list.insert(2, 3).unwrap();
        assert_eq!(list.to_string(), "1 -> 2 -> 3 -> None");
    }

    #[test]
    fn iter_empty_list() {
        let list: LinkedList<i32> = LinkedList::new();
        let mut iter = list.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_single_element() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(1);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_multiple_elements() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_does_not_consume_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(list.to_string(), "3 -> 2 -> 1 -> None");
    }

    #[test]
    fn from_vec() {
        let vec = vec![1, 2, 3, 4];
        let list: LinkedList<i32> = LinkedList::from(vec);

        assert_eq!(list.to_string(), "1 -> 2 -> 3 -> 4 -> None");
    }
}
