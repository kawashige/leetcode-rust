pub struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut count = 0;
        let mut minus = 0;
        for i in 0..nums.len() {
            if minus < nums[i] {
                count += 1;
                minus = nums[i]
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2357() {
        assert_eq!(Solution::minimum_operations(vec![1, 5, 0, 3, 5]), 3);
        assert_eq!(Solution::minimum_operations(vec![0]), 0);
    }
}
