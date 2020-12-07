pub struct Solution {}

use std::collections::BTreeSet;
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let top3 = nums
            .into_iter()
            .collect::<BTreeSet<i32>>()
            .into_iter()
            .rev()
            .take(3)
            .collect::<Vec<i32>>();

        if top3.len() == 3 {
            *top3.last().unwrap()
        } else {
            top3[0]
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0414() {
        assert_eq!(2, Solution::third_max(vec![1, 2, 2, 5, 3, 5]));
        assert_eq!(1, Solution::third_max(vec![3, 2, 1]));
        assert_eq!(2, Solution::third_max(vec![1, 2]));
        assert_eq!(1, Solution::third_max(vec![2, 2, 3, 1]));
    }
}
