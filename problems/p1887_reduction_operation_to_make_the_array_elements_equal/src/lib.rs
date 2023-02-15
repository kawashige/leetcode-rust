pub struct Solution {}

impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut result = 0;
        let mut count = 1;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] != nums[i + 1] {
                result += count;
            }
            count += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1887() {
        assert_eq!(Solution::reduction_operations(vec![5, 1, 3]), 3);
        assert_eq!(Solution::reduction_operations(vec![1, 1, 1]), 0);
        assert_eq!(Solution::reduction_operations(vec![1, 1, 2, 2, 3]), 4);
    }
}
