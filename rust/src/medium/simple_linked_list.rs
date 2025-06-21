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

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn recursive_len(node: &Node<T>, count: usize) -> usize {
        match &node.next {
            None => count,
            Some(node) => Self::recursive_len(node, count + 1),
        }
    }

    fn recursive_peek(node: &Node<T>) -> &T {
        match &node.next {
            None => &node.data,
            Some(node) => Self::recursive_peek(node),
        }
    }

    fn recursive_push(node: &mut Node<T>, new_node: Node<T>) {
        match &mut node.next {
            None => node.next = Some(Box::new(new_node)),
            Some(node) => Self::recursive_push(node, new_node),
        }
    }

    fn recursive_pop(node: &mut Node<T>) -> Option<T> {
        let is_end_reached = node.next.as_ref()?.next.is_none();

        match is_end_reached {
            true => Some(node.next.take()?.data),
            false => Self::recursive_pop(&mut *node.next.as_mut()?),
        }
    }

    pub fn len(&self) -> usize {
        match &self.head {
            None => 0,
            Some(node) => Self::recursive_len(node, 1),
        }
    }

    pub fn push(&mut self, element: T) {
        let new_node = Node {
            data: element,
            next: None,
        };

        match &mut self.head {
            None => self.head = Some(Box::new(new_node)),
            Some(node) => Self::recursive_push(node, new_node),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match &self.head.as_ref()?.next {
            None => Some(self.head.take()?.data),
            Some(_) => Self::recursive_pop(&mut *self.head.as_mut()?),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        Some(Self::recursive_peek(self.head.as_ref()?))
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut linked_list = SimpleLinkedList::new();

        while let Some(i) = self.pop() {
            linked_list.push(i)
        }

        linked_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut linked_list = SimpleLinkedList::new();

        for i in iter {
            linked_list.push(i);
        }

        linked_list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v = vec![];

        while let Some(i) = linked_list.pop() {
            v.push(i)
        }

        v.reverse();
        v
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

#[test]
fn test_pop_decrements_length() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();

    list.push(1);

    list.push(2);

    list.pop();

    assert_eq!(list.len(), 1, "list's length must be 1");

    list.pop();

    assert_eq!(list.len(), 0, "list's length must be 0");
}

#[test]
fn test_peek_returns_reference_to_head_element_but_does_not_remove_it() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();

    assert_eq!(list.peek(), None, "No element should be contained in list");

    list.push(2);

    assert_eq!(list.peek(), Some(&2), "Element must be 2");

    assert_eq!(list.peek(), Some(&2), "Element must be still 2");

    list.push(3);

    assert_eq!(list.peek(), Some(&3), "Head element is now 3");

    assert_eq!(list.pop(), Some(3), "Element must be 3");

    assert_eq!(list.peek(), Some(&2), "Head element is now 2");

    assert_eq!(list.pop(), Some(2), "Element must be 2");

    assert_eq!(list.peek(), None, "No element should be contained in list");
}
