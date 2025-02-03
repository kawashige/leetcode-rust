pub struct Solution {}

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut len = vec![1; 2];
        let mut result = 1;

        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                len[0] += 1;
                len[1] = 1;
            } else if nums[i - 1] < nums[i] {
                len[1] += 1;
                len[0] = 1;
            } else {
                len[0] = 1;
                len[1] = 1;
            }
            result = result.max(len[0].max(len[1]));
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3105() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 2, 1]), 3);
    }
}
