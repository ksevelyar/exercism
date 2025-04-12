// use std::marker::PhantomData;
use std::ptr::NonNull;

type Link<T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    // _hint: PhantomData<T>,
}

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

// pub struct Cursor<'a, T>(std::marker::PhantomData<&'a mut T>);
#[derive(Debug)]
pub struct Cursor<'a, T> {
    cur: Link<T>,
    list: &'a mut LinkedList<T>,
    index: Option<usize>,
}

pub struct Iter<'a, T> {
    cur: Link<T>,
    list: &'a LinkedList<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            // _hint: PhantomData,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.front.is_none() && self.back.is_none()
    }

    pub fn len(&self) -> usize {
        0
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<T> {
        let index = self.front.map(|_| 0);

        Cursor {
            cur: self.front,
            list: self,
            index: index,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            cur: self.back,
            list: self,
            index: None,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            cur: self.front,
            list: self,
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<'a, T: std::fmt::Debug> Cursor<'a, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.cur.map(|node| &mut (*node.as_ptr()).elem) }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        unsafe {
            self.cur.map(|cur| {
                self.list.len -= 1;

                if let Some(next) = (*cur.as_ptr()).back {
                    self.cur = Some(next);
                } else if let Some(prev) = (*cur.as_ptr()).front {
                    self.cur = Some(prev);
                }

                Box::from_raw(cur.as_ptr()).elem
            })
        }
    }

    pub fn insert_before(&mut self, element: T) {
        unsafe {
            let new_node = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem: element,
            })));

            if let Some(cur) = self.cur {
                (*new_node.as_ptr()).back = self.cur;
                (*new_node.as_ptr()).front = (*cur.as_ptr()).front;
                (*cur.as_ptr()).front = Some(new_node);

                // set new front if list.front is cur
                if self.list.front == self.cur {
                    self.list.front = Some(new_node)
                };

                *self.index.as_mut().unwrap() += 1;
            } else {
                self.cur = Some(new_node);

                self.list.front = Some(new_node);
                self.list.back = Some(new_node);
                // self.index = Some(0)
            }

            self.list.len += 1;
        }
    }

    pub fn insert_after(&mut self, element: T) {
        unsafe {
            let new_node = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem: element,
            })));

            if let Some(cur) = self.cur {
                (*new_node.as_ptr()).front = self.cur;
                (*new_node.as_ptr()).back = (*cur.as_ptr()).back;
                (*cur.as_ptr()).back = Some(new_node);

                // set new front if list.front is cur
                if self.list.back == self.cur {
                    self.list.back = Some(new_node)
                };

                // *self.index.as_mut().unwrap() += 1;
            } else {
                self.cur = Some(new_node);

                self.list.back = Some(new_node);
                self.list.front = Some(new_node);
                // self.index = Some(0)
            }

            self.list.len += 1;
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            self.cur.map(|node| {
                self.cur = (*node.as_ptr()).back;

                &(*node.as_ptr()).elem
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics_empty_list() {
        let list: LinkedList<i32> = LinkedList::new();

        assert_eq!(list.len(), 0);

        assert!(list.is_empty());
    }

    #[test]
    fn cursor_insert_before_on_empty_list() {
        let mut list = LinkedList::new();

        list.cursor_front().insert_before(0);

        assert_eq!(Some(0), list.cursor_front().take());
    }

    #[test]
    fn iter() {
        let mut list: LinkedList<i32> = LinkedList::new();

        for num in 0..10 {
            list.cursor_back().insert_after(num);
        }

        for (num, &entered_num) in (0..10).zip(list.iter()) {
            assert_eq!(num, entered_num);
        }
    }
}
