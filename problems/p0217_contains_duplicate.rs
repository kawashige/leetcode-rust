pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.len() != nums.into_iter().collect::<HashSet<i32>>().len()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0217() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
        assert!(!Solution::contains_duplicate(vec![1, 2, 3]));
        assert!(Solution::contains_duplicate(vec![
            1, 1, 1, 3, 3, 4, 3, 2, 4, 2
        ]));
    }
}
