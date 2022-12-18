use std::iter::FromIterator;

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Debug> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T: std::fmt::Debug> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        dbg!(self);
        1
    }

    pub fn push(&mut self, element: T) {
        let new_node = Node {
            data: element,
            next: None,
        };

        match self.head {
            None => self.head = Some(Box::new(new_node)),
            Some(ref mut head) => head.next = Some(Box::new(new_node)),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        unimplemented!()
    }
}

#[test]
fn test_new_list_is_empty() {
    let list: SimpleLinkedList<u32> = SimpleLinkedList::new();

    assert_eq!(list.len(), 0, "list's length must be 0");
}

#[test]
fn test_push_increments_length() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();

    list.push(1);

    assert_eq!(list.len(), 1, "list's length must be 1");

    list.push(2);

    assert_eq!(list.len(), 2, "list's length must be 2");
}
