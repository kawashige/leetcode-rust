pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = HashMap::new();
        let mut result = 0;
        for n in nums {
            if let Some(target) = counts.get_mut(&(k - n)) {
                if &0 < target {
                    result += 1;
                    *target -= 1;
                    continue;
                }
            }
            *counts.entry(n).or_insert(0) += 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day18() {
        assert_eq!(2, Solution::max_operations(vec![1, 2, 3, 4], 5));
        assert_eq!(1, Solution::max_operations(vec![3, 1, 3, 4, 3], 6));
    }
}
