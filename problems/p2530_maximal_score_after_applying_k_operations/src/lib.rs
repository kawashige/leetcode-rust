use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::from_iter(nums.into_iter());
        let mut score = 0;

        for _ in 0..k {
            let val = heap.pop().unwrap();
            score += val as i64;
            heap.push((val + 2) / 3);
        }

        score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2530() {
        assert_eq!(Solution::max_kelements(vec![10, 10, 10, 10, 10], 5), 50);
        assert_eq!(Solution::max_kelements(vec![1, 10, 3, 3, 3], 3), 17);
    }
}
