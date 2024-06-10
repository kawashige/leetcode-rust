use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::from_iter(gifts.into_iter());
        for _ in 0..k {
            let v = heap.pop().unwrap();
            heap.push((v as f64).sqrt() as i32);
        }
        heap.into_iter().fold(0, |acc, num| acc + num as i64)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2558() {
        assert_eq!(Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4), 29);
        assert_eq!(Solution::pick_gifts(vec![1, 1, 1, 1], 4), 4);
    }
}
