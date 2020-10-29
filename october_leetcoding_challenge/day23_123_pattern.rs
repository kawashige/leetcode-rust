pub struct Solution {}

use std::collections::BTreeMap;
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        let mut ranges = BTreeMap::new();
        let mut min = nums[0];
        let mut max = nums[0];
        for i in 1..nums.len() {
            if nums[i] < min {
                min = nums[i];
                max = nums[i];
            } else if max < nums[i] {
                max = nums[i];
                ranges.insert(min, max);
            }

            for r in ranges.range(..nums[i]) {
                if &nums[i] < r.1 {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day23() {
        assert!(!Solution::find132pattern(vec![1, 2, 3, 4]));
        assert!(Solution::find132pattern(vec![3, 1, 4, 2]));
        assert!(Solution::find132pattern(vec![-1, 3, 2, 0]));
        assert!(Solution::find132pattern(vec![3, 5, 0, 3, 4]));
    }
}
