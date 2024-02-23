use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable();

        let mut can_use = BinaryHeap::new();
        let mut cannot_use = BinaryHeap::new();

        for i in 0..intervals.len() {
            while let Some(Reverse(end)) = cannot_use.pop() {
                if intervals[i][0] <= end {
                    cannot_use.push(Reverse(end));
                    break;
                }
                can_use.push(Reverse(end));
            }

            if let Some(Reverse(end)) = can_use.pop() {
                if intervals[i][0] <= end {
                    can_use.push(Reverse(end));
                }
                cannot_use.push(Reverse(intervals[i][1]));
            } else {
                cannot_use.push(Reverse(intervals[i][1]));
            }
        }

        (can_use.len() + cannot_use.len()) as i32
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2406() {
        assert_eq!(Solution::min_groups(vec![vec![5,10],vec![6,8],vec![1,5],vec![2,3],vec![1,10]]), 3);
        assert_eq!(Solution::min_groups(vec![vec![1,3],vec![5,6],vec![8,10],vec![11,13]]), 1);
    }
}
