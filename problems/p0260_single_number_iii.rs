pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut results = HashSet::new();
        for n in nums {
            if results.contains(&n) {
                results.remove(&n);
            } else {
                results.insert(n);
            }
        }
        results.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0260() {
        assert_eq!(vec![3, 5], Solution::single_number(vec![1, 2, 1, 3, 2, 5]));
        assert_eq!(vec![-1, 0], Solution::single_number(vec![-1, 0]));
        assert_eq!(vec![0, 1], Solution::single_number(vec![0, 1]));
    }
}
