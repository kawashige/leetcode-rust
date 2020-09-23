use std::collections::BTreeMap;

#[derive(Default)]
pub struct MinStack {
    stack: Vec<i32>,
    values: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        let count = self.values.entry(x).or_insert(0);
        *count += 1;
    }

    fn pop(&mut self) {
        let x = self.stack.pop().unwrap();
        if let Some(count) = self.values.get_mut(&x) {
            *count -= 1;
        }
        if self.values[&x] == 0 {
            self.values.remove(&x);
        }
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().clone()
    }

    fn get_min(&self) -> i32 {
        self.values.keys().next().unwrap().clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0155() {
        let mut obj = MinStack::new();
        obj.push(-2);
        obj.push(0);
        obj.push(-3);
        assert_eq!(-3, obj.get_min());
        obj.pop();
        assert_eq!(0, obj.top());
        assert_eq!(-2, obj.get_min());
    }
}
