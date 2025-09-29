pub struct Solution {}

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut left = 0;
        let mut result = 0;
        for i in 0..nums.len() - 1 {
            left += nums[i];
            if (left - (sum - left)) % 2 == 0 {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3432() {
        assert_eq!(Solution::count_partitions(vec![10, 10, 3, 7, 6]), 4);
        assert_eq!(Solution::count_partitions(vec![1, 2, 2]), 0);
        assert_eq!(Solution::count_partitions(vec![2, 4, 6, 8]), 3);
    }
}
