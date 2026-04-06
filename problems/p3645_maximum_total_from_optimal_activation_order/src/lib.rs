use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub struct Solution {}

impl Solution {
    pub fn max_total(value: Vec<i32>, limit: Vec<i32>) -> i64 {
        let mut group = HashMap::new();
        for i in 0..value.len() {
            (*group.entry(limit[i]).or_insert(BinaryHeap::new())).push(Reverse(value[i]));
            if (limit[i] as usize) < group.get_mut(&limit[i]).unwrap().len() {
                group.get_mut(&limit[i]).unwrap().pop();
            }
        }
        group
            .values()
            .map(|h| {
                let mut sum = 0;
                let mut h = h.clone();
                while let Some(Reverse(v)) = h.pop() {
                    sum += v as i64;
                }
                sum
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3645() {
        assert_eq!(Solution::max_total(vec![3, 5, 8], vec![2, 1, 3]), 16);
        assert_eq!(Solution::max_total(vec![4, 2, 6], vec![1, 1, 1]), 6);
        assert_eq!(Solution::max_total(vec![4, 1, 5, 2], vec![3, 3, 2, 3]), 12);
    }
}
