use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

struct SmallestInfiniteSet {
    set: BinaryHeap<Reverse<i32>>,
    exists: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        let mut set = BinaryHeap::new();
        let mut exists = HashSet::new();
        for i in 1..=1000 {
            set.push(Reverse(i));
            exists.insert(i);
        }
        Self { set, exists }
    }

    fn pop_smallest(&mut self) -> i32 {
        if let Some(Reverse(smallest)) = self.set.pop() {
            self.exists.remove(&smallest);
            smallest
        } else {
            -1
        }
    }

    fn add_back(&mut self, num: i32) {
        if !self.exists.contains(&num) {
            self.set.push(Reverse(num));
            self.exists.insert(num);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2336() {
        let mut obj = SmallestInfiniteSet::new();
        obj.add_back(2);
        assert_eq!(obj.pop_smallest(), 1);
        assert_eq!(obj.pop_smallest(), 2);
        assert_eq!(obj.pop_smallest(), 3);
        obj.add_back(1);
        assert_eq!(obj.pop_smallest(), 1);
        assert_eq!(obj.pop_smallest(), 4);
        assert_eq!(obj.pop_smallest(), 5);
    }
}
