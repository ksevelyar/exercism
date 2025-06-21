use std::ptr::NonNull;

type Link<T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
}

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

#[derive(Debug)]
pub struct Cursor<'a, T> {
    cur: Link<T>,
    list: &'a mut LinkedList<T>,
}

pub struct Iter<'a, T> {
    cur: Link<T>,
    _list: &'a LinkedList<T>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<T> {
        Cursor {
            cur: self.front,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            cur: self.back,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            cur: self.front,
            _list: self,
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.cur.map(|node| &mut (*node.as_ptr()).elem) }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            self.cur.map(|cur| {
                if let Some(next) = (*cur.as_ptr()).back {
                    self.cur = Some(next);

                    Some(&mut (*next.as_ptr()).elem)
                } else {
                    None
                }
            })?
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            self.cur.map(|cur| {
                if let Some(prev) = (*cur.as_ptr()).front {
                    self.cur = Some(prev);

                    Some(&mut (*prev.as_ptr()).elem)
                } else {
                    None
                }
            })?
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        unsafe {
            self.cur.map(|cur| {
                self.list.len -= 1;

                self.cur = if let Some(next) = (*cur.as_ptr()).back {
                    (*next.as_ptr()).front = (*cur.as_ptr()).front;

                    if let Some(prev) = (*cur.as_ptr()).front {
                        (*prev.as_ptr()).back = Some(next);
                    }

                    Some(next)
                } else if let Some(prev) = (*cur.as_ptr()).front {
                    (*prev.as_ptr()).back = None;

                    if let Some(next) = (*cur.as_ptr()).back {
                        (*next.as_ptr()).front = Some(prev);
                    }

                    Some(prev)
                } else {
                    None
                };

                if self.list.front == Some(cur) {
                    self.list.front = self.cur;
                };

                if self.list.back == Some(cur) {
                    self.list.back = self.cur;
                };

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

                if let Some(prev) = (*cur.as_ptr()).front {
                    (*prev.as_ptr()).back = Some(new_node);
                }
                (*cur.as_ptr()).front = Some(new_node);

                if self.list.front == self.cur {
                    self.list.front = Some(new_node)
                };
            } else {
                self.cur = Some(new_node);

                self.list.front = Some(new_node);
                self.list.back = Some(new_node);
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

                if let Some(next) = (*cur.as_ptr()).back {
                    (*next.as_ptr()).front = Some(new_node);
                }
                (*cur.as_ptr()).back = Some(new_node);

                if self.list.back == self.cur {
                    self.list.back = Some(new_node)
                };
            } else {
                self.cur = Some(new_node);

                self.list.back = Some(new_node);
                self.list.front = Some(new_node);
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

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();

        for item in iter {
            list.cursor_back().insert_after(item);
        }

        list
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.cursor_front().take().is_some() {}
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

    #[test]
    fn cursor_insert_after_in_middle() {
        let mut list = (0..10).collect::<LinkedList<_>>();

        {
            let mut cursor = list.cursor_front();
            cursor.next();
            cursor.next();
            cursor.next();
            let didnt_run_into_end = cursor.next();

            assert!(didnt_run_into_end.is_some());

            for n in (0..10).rev() {
                cursor.insert_after(n);
            }
        }

        assert_eq!(list.len(), 20);

        let expected = (0..5).chain(0..10).chain(5..10);

        assert!(expected.eq(list.iter().cloned()));
    }

    #[test]
    fn cursor_insert_before_in_middle() {
        let mut list = (0..10).collect::<LinkedList<_>>();

        {
            let mut cursor = list.cursor_back();
            cursor.prev();
            cursor.prev();
            cursor.prev();
            let didnt_run_into_end = cursor.prev();

            assert!(didnt_run_into_end.is_some());

            for n in 0..10 {
                cursor.insert_before(n);
            }
        }

        assert_eq!(list.len(), 20);

        let expected = (0..5).chain(0..10).chain(5..10);

        assert!(expected.eq(list.iter().cloned()));
    }
}
