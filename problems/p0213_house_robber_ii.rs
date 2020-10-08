pub struct Solution {}

use std::cmp::max;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut dp1 = vec![0; nums.len() - 1];
        let mut dp2 = vec![0; nums.len() - 1];
        for i in 0..nums.len() {
            if i < nums.len() - 1 {
                if i == 0 {
                    dp1[i] = nums[0];
                } else if i == 1 {
                    dp1[i] = max(nums[1], dp1[0]);
                } else {
                    dp1[i] = max(dp1[i - 1], dp1[i - 2] + nums[i]);
                }
            }
            if 0 < i {
                if i == 1 {
                    dp2[i - 1] = nums[1];
                } else if i == 2 {
                    dp2[i - 1] = max(nums[2], dp2[0]);
                } else {
                    dp2[i - 1] = max(dp2[i - 2], dp2[i - 3] + nums[i]);
                }
            }
        }
        *max(dp1.last().unwrap(), dp2.last().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0213() {
        assert_eq!(3, Solution::rob(vec![2, 3, 2]));
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
        assert_eq!(0, Solution::rob(vec![0]));
        assert_eq!(13, Solution::rob(vec![10, 2, 3, 1]));
        assert_eq!(0, Solution::rob(vec![0, 0]));
        assert_eq!(7, Solution::rob(vec![0, 7]));
        assert_eq!(103, Solution::rob(vec![1, 3, 1, 3, 100]));
    }
}
