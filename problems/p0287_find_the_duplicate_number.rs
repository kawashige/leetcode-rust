pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut sets = HashSet::new();
        for n in nums {
            if sets.contains(&n) {
                return n;
            } else {
                sets.insert(n);
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0287() {
        assert_eq!(2, Solution::find_duplicate(vec![1, 3, 4, 2, 2]));
        assert_eq!(3, Solution::find_duplicate(vec![3, 1, 3, 4, 2]));
        assert_eq!(1, Solution::find_duplicate(vec![1, 1]));
        assert_eq!(1, Solution::find_duplicate(vec![1, 1, 2]));
        assert_eq!(2, Solution::find_duplicate(vec![2, 2, 2, 2, 2]));
    }
}
