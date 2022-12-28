use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut sum = 0;
        for pile in piles {
            sum += pile;
            heap.push(pile);
        }

        for _ in 0..k {
            if let Some(pile) = heap.pop() {
                sum -= pile / 2;
                if 0 < pile - pile / 2 {
                    heap.push(pile - pile / 2);
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1962() {
        assert_eq!(Solution::min_stone_sum(vec![5, 4, 9], 2), 12);
        assert_eq!(Solution::min_stone_sum(vec![4, 3, 6, 7], 3), 12);
    }
}
