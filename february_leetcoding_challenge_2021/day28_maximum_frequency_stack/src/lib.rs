use std::collections::HashMap;

struct FreqStack {
    index_map: HashMap<i32, usize>,
    stacks: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        Self {
            index_map: HashMap::new(),
            stacks: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        let entry = self.index_map.entry(x).or_insert(0);
        *entry += 1;
        if self.stacks.len() == *entry - 1 {
            self.stacks.push(vec![x]);
        } else {
            self.stacks[*entry - 1].push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        let n = self.stacks.last_mut().unwrap().pop().unwrap();
        *self.index_map.get_mut(&n).unwrap() -= 1;
        if self.stacks.len() > 1 && self.stacks.last().unwrap().is_empty() {
            self.stacks.pop();
        }
        n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day28() {
        let mut obj = FreqStack::new();
        obj.push(5);
        obj.push(7);
        obj.push(5);
        obj.push(7);
        obj.push(4);
        obj.push(5);
        assert_eq!(obj.pop(), 5);
        assert_eq!(obj.pop(), 7);
        assert_eq!(obj.pop(), 5);
        assert_eq!(obj.pop(), 4);
    }
}
