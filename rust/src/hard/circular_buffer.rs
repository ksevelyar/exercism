use std::collections::VecDeque;

#[derive(Debug)]
pub struct CircularBuffer<T> {
    data: VecDeque<T>,
    capacity: usize,
    index: Option<usize>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
            capacity,
            index: None,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.data.len() == self.capacity {
            return Err(Error::FullBuffer);
        }

        self.data.push_back(element);
        self.index.get_or_insert(0);

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if let Some(num) = self.index {
            let value = self.data.remove(num).ok_or(Error::EmptyBuffer)?;

            if self.data.is_empty() {
                self.index = None;
            } else {
                self.index = Some(num.min(self.data.len().saturating_sub(1)));
            }

            Ok(value)
        } else {
            Err(Error::EmptyBuffer)
        }
    }

    pub fn clear(&mut self) {
        self.index = None;
        self.data.clear()
    }

    pub fn overwrite(&mut self, element: T) {
        if self.data.len() < self.capacity {
            self.write(element).unwrap();
            return;
        }

        let index = self.index.unwrap();
        self.data[index] = element;
        self.index = Some((index + 1) % self.capacity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_empty_buffer_should_fail() {
        let mut buffer = CircularBuffer::<i32>::new(1);

        assert_eq!(buffer.read(), Err(Error::EmptyBuffer));
    }

    #[test]
    fn can_read_an_item_just_written() {
        let mut buffer = CircularBuffer::new(1);

        assert!(buffer.write(1).is_ok());

        assert_eq!(buffer.read(), Ok(1));
    }

    #[test]
    fn items_are_read_in_the_order_they_are_written() {
        let mut buffer = CircularBuffer::new(2);

        assert!(buffer.write(1).is_ok());

        assert!(buffer.write(2).is_ok());

        assert_eq!(buffer.read(), Ok(1));

        assert_eq!(buffer.read(), Ok(2));
    }

    #[test]
    fn overwrite_replaces_the_oldest_item_remaining_in_buffer_following_a_read() {
        let mut buffer = CircularBuffer::new(3);

        assert!(buffer.write(1).is_ok());

        assert!(buffer.write(2).is_ok());

        assert!(buffer.write(3).is_ok());

        assert_eq!(buffer.read(), Ok(1));

        assert!(buffer.write(4).is_ok());

        buffer.overwrite(5);

        assert_eq!(buffer.read(), Ok(3));

        assert_eq!(buffer.read(), Ok(4));

        assert_eq!(buffer.read(), Ok(5));
    }

    #[test]
    fn each_item_may_only_be_read_once() {
        let mut buffer = CircularBuffer::new(1);

        assert!(buffer.write(1).is_ok());

        assert_eq!(buffer.read(), Ok(1));

        assert_eq!(buffer.read(), Err(Error::EmptyBuffer));
    }

    #[test]
    fn full_buffer_can_t_be_written_to() {
        let mut buffer = CircularBuffer::new(1);

        assert!(buffer.write(1).is_ok());

        assert_eq!(buffer.write(2), Err(Error::FullBuffer));
    }

    #[test]
    fn initial_clear_does_not_affect_wrapping_around() {
        let mut buffer = CircularBuffer::new(2);

        buffer.clear();

        assert!(buffer.write(1).is_ok());

        assert!(buffer.write(2).is_ok());

        buffer.overwrite(3);

        buffer.overwrite(4);

        assert_eq!(buffer.read(), Ok(3));

        assert_eq!(buffer.read(), Ok(4));

        assert_eq!(buffer.read(), Err(Error::EmptyBuffer));
    }
}
