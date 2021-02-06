#[derive(Debug)]
struct MyCircularQueue {
    values: Vec<i32>,
    start: usize,
    end: usize,
    len: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            values: vec![0; k as usize],
            start: 0,
            end: 0,
            len: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            if !self.is_empty() {
                self.end = (self.end + 1) % self.values.len();
            }
            self.values[self.end] = value;
            self.len += 1;
            true
        }
    }

    fn de_queue(&mut self) -> bool {
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

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.values[self.start]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.values[self.end]
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.values.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::MyCircularQueue;

    #[test]
    fn test_0622() {
        let mut obj = MyCircularQueue::new(3);
        assert!(obj.en_queue(1));
        assert!(obj.en_queue(2));
        assert!(obj.en_queue(3));
        assert!(!obj.en_queue(4));
        assert_eq!(obj.rear(), 3);
        assert!(obj.is_full());
        assert!(obj.de_queue());
        assert!(obj.en_queue(4));
        assert_eq!(obj.rear(), 4);
    }
}
