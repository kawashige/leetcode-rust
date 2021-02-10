struct MyCircularDeque {
    values: Vec<i32>,
    start: usize,
    end: usize,
    len: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    /** Initialize your data structure here. Set the size of the deque to be k. */
    fn new(k: i32) -> Self {
        Self {
            values: vec![0; k as usize],
            start: 0,
            end: 0,
            len: 0,
        }
    }

    /** Adds an item at the front of Deque. Return true if the operation is successful. */
    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            if !self.is_empty() {
                self.start = if self.start == 0 {
                    self.values.len() - 1
                } else {
                    self.start - 1
                };
            }
            self.values[self.start] = value;
            self.len += 1;
            true
        }
    }

    /** Adds an item at the rear of Deque. Return true if the operation is successful. */
    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            if !self.is_empty() {
                self.end = if self.end == self.values.len() - 1 {
                    0
                } else {
                    self.end + 1
                };
            }
            self.values[self.end] = value;
            self.len += 1;
            true
        }
    }

    /** Deletes an item from the front of Deque. Return true if the operation is successful. */
    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            if 1 < self.len {
                self.start = if self.start == self.values.len() - 1 {
                    0
                } else {
                    self.start + 1
                };
            }
            self.len -= 1;
            true
        }
    }

    /** Deletes an item from the rear of Deque. Return true if the operation is successful. */
    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            if 1 < self.len {
                self.end = if self.end == 0 {
                    self.values.len() - 1
                } else {
                    self.end - 1
                };
            }
            self.len -= 1;
            true
        }
    }

    /** Get the front item from the deque. */
    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.values[self.start]
        }
    }

    /** Get the last item from the deque. */
    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.values[self.end]
        }
    }

    /** Checks whether the circular deque is empty or not. */
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    /** Checks whether the circular deque is full or not. */
    fn is_full(&self) -> bool {
        self.len == self.values.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0641() {
        let mut obj = MyCircularDeque::new(3);
        assert!(obj.insert_last(1));
        assert!(obj.insert_last(2));
        assert!(obj.insert_front(3));
        assert!(!obj.insert_front(4));
        assert_eq!(obj.get_rear(), 2);
        assert!(obj.is_full());
        assert!(obj.delete_last());
        assert!(obj.insert_front(4));
        assert_eq!(obj.get_front(), 4);
    }
}
