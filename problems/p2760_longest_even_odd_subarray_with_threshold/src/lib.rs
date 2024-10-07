pub struct Solution {}

impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            if threshold < nums[i] || nums[i] % 2 != 0 {
                continue;
            }
            let mut len = 1;
            for j in i + 1..nums.len() {
                if threshold < nums[j] || nums[j - 1] % 2 == nums[j] % 2 {
                    break;
                }
                len += 1;
            }
            result = result.max(len);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2760() {
        assert_eq!(Solution::longest_alternating_subarray(vec![4], 1), 0);
        assert_eq!(
            Solution::longest_alternating_subarray(vec![3, 2, 5, 4], 5),
            3
        );
        assert_eq!(Solution::longest_alternating_subarray(vec![1, 2], 2), 1);
        assert_eq!(
            Solution::longest_alternating_subarray(vec![2, 3, 4, 5], 4),
            3
        );
    }
}
