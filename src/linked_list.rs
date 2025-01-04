use std::fmt::{self, Display, Formatter};

type NodePointer<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: NodePointer<T>,
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
    fn non_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_head(1);
        list.push_head(2);
        list.push_head(3);
        assert_eq!(list.to_string(), "3 -> 2 -> 1 -> None");
    }
}
