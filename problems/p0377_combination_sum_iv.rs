pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        fn recurse(nums: &[i32], target: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            if target == 0 {
                return 1;
            }
            if let Some(n) = memo.get(&target) {
                return *n;
            }
            let mut result = 0;
            for i in 0..nums.len() {
                if target < nums[i] {
                    break;
                }
                let n = target - nums[i];
                let r = recurse(nums, n, memo);
                memo.insert(n, r);
                result += r;
            }
            result
        }

        if nums.is_empty() || target == 0 {
            return 0;
        }

        let mut nums = nums;
        nums.sort();
        recurse(&nums, target, &mut HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0377() {
        assert_eq!(181997601, Solution::combination_sum4(vec![1, 2, 3], 32));
        assert_eq!(7, Solution::combination_sum4(vec![1, 2, 3], 4));
        assert_eq!(0, Solution::combination_sum4(vec![1, 2, 3], 0));
        assert_eq!(0, Solution::combination_sum4(vec![10, 20, 30], 1));
    }
}
