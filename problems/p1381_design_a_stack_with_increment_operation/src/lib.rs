struct CustomStack {
    max_size: usize,
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    fn new(max_size: i32) -> Self {
        Self {
            max_size: max_size as usize,
            stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() < self.max_size {
            self.stack.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..(k as usize).min(self.stack.len()) {
            self.stack[i] += val;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1381() {
        let mut obj = CustomStack::new(3);
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.pop(), 2);
        obj.push(2);
        obj.push(3);
        obj.push(4);
        obj.increment(5, 100);
        obj.increment(2, 100);
        assert_eq!(obj.pop(), 103);
        assert_eq!(obj.pop(), 202);
        assert_eq!(obj.pop(), 201);
        assert_eq!(obj.pop(), -1);
    }
}
