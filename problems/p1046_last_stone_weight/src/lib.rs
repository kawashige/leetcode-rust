use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);
        while let Some(y) = heap.pop() {
            if let Some(x) = heap.pop() {
                if y > x {
                    heap.push(y - x);
                }
            } else {
                return y;
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1046() {
        assert_eq!(Solution::last_stone_weight(vec![2, 4, 7, 1, 8, 1]), 1);
        assert_eq!(Solution::last_stone_weight(vec![1]), 1);
    }
}
