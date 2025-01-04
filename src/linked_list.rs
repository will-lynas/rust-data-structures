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

    pub fn push_head(&mut self, value: T) {
        let node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    pub fn pop_head(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn insert(&mut self, index: usize, value: T) -> Result<(), LinkedListError> {
        if index == 0 {
            self.push_head(value);
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
        list.push_head(1);
        list.push_head(2);
        list.push_head(3);
        assert_eq!(list.to_string(), "3 -> 2 -> 1 -> None");
    }

    #[test]
    fn pop() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_head(1);
        list.push_head(2);
        assert_eq!(list.pop_head(), Some(2));
        assert_eq!(list.to_string(), "1 -> None");
    }

    #[test]
    fn pop_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.pop_head(), None);
    }

    #[test]
    fn insert_at_head() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_head(1);
        list.push_head(2);
        list.insert(0, 3).unwrap();
        assert_eq!(list.to_string(), "3 -> 2 -> 1 -> None");
    }

    #[test]
    fn insert_in_middle() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_head(1);
        list.push_head(3);
        list.insert(1, 2).unwrap();
        assert_eq!(list.to_string(), "3 -> 2 -> 1 -> None");
    }

    #[test]
    fn insert_out_of_bounds() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_head(1);
        let result = list.insert(10, 2);
        assert_eq!(result, Err(LinkedListError::OutOfBounds));
    }

    #[test]
    fn insert_at_tail() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_head(2);
        list.push_head(1);
        list.insert(2, 3).unwrap();
        assert_eq!(list.to_string(), "1 -> 2 -> 3 -> None");
    }
}
