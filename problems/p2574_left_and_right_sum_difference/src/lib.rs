pub struct Solution {}

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut right_sum = vec![0; nums.len()];
        for i in (0..nums.len() - 1).rev() {
            right_sum[i] = right_sum[i + 1] + nums[i + 1];
        }

        let mut left_sum = 0;
        let mut result = vec![0; nums.len()];

        for i in 0..nums.len() {
            result[i] = (left_sum - right_sum[i]).abs();
            left_sum += nums[i];
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2574() {
        assert_eq!(
            Solution::left_right_difference(vec![10, 4, 8, 3]),
            vec![15, 1, 11, 22]
        );
        assert_eq!(Solution::left_right_difference(vec![1]), vec![0]);
    }
}
