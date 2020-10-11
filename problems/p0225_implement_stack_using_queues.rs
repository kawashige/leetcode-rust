use std::collections::VecDeque;
#[derive(Default)]
struct MyStack {
    queue1: VecDeque<i32>,
    queue2: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        if self.queue1.is_empty() {
            self.queue1.push_back(x);
            while !self.queue2.is_empty() {
                self.queue1.push_back(self.queue2.pop_front().unwrap());
            }
        } else {
            self.queue2.push_back(x);
            while !self.queue1.is_empty() {
                self.queue2.push_back(self.queue1.pop_front().unwrap());
            }
        }
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.queue1.is_empty() {
            self.queue2.pop_front().unwrap()
        } else {
            self.queue1.pop_front().unwrap()
        }
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        if self.queue1.is_empty() {
            self.queue2[0]
        } else {
            self.queue1[0]
        }
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.queue1.is_empty() && self.queue2.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0225() {
        let mut obj = MyStack::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(2, obj.top());
        obj.pop();
        assert_eq!(1, obj.top());
        assert!(!obj.empty());
    }
}
