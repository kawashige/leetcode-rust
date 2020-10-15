#[derive(Default)]
struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        if self.stack1.is_empty() {
            while !self.stack2.is_empty() {
                self.stack1.push(self.stack2.pop().unwrap())
            }
            self.stack2.push(x);
            while !self.stack1.is_empty() {
                self.stack2.push(self.stack1.pop().unwrap())
            }
        } else {
            while !self.stack1.is_empty() {
                self.stack2.push(self.stack1.pop().unwrap())
            }
            self.stack1.push(x);
            while !self.stack2.is_empty() {
                self.stack1.push(self.stack2.pop().unwrap())
            }
        }
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.stack1.is_empty() {
            self.stack2.pop().unwrap()
        } else {
            self.stack1.pop().unwrap()
        }
    }

    /** Get the front element. */
    fn peek(&self) -> i32 {
        if self.stack1.is_empty() {
            *self.stack2.last().unwrap()
        } else {
            *self.stack1.last().unwrap()
        }
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0232() {
        let mut obj = MyQueue::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(1, obj.peek());
        assert_eq!(1, obj.pop());
        assert!(!obj.empty());
    }
}
