use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        for n in nums {
            if heap.len() < k {
                heap.push(Reverse(n))
            } else if heap.peek().unwrap() > &Reverse(n) {
                heap.pop();
                heap.push(Reverse(n));
            }
        }

        Self { heap, k }
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(Reverse(val))
        } else if self.heap.peek().unwrap() > &Reverse(val) {
            self.heap.pop();
            self.heap.push(Reverse(val));
        }
        match self.heap.peek().unwrap() {
            Reverse(n) => *n,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0703() {
        let mut obj = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(obj.add(3), 4);
        assert_eq!(obj.add(5), 5);
        assert_eq!(obj.add(10), 5);
        assert_eq!(obj.add(9), 8);
        assert_eq!(obj.add(4), 8);
    }
}
