pub struct Solution {}

impl Solution {
    pub fn max_subarrays(nums: Vec<i32>) -> i32 {
        let mut num = std::i32::MAX;
        let mut count = 0;
        for i in 0..nums.len() {
            num &= nums[i];
            if num == 0 {
                count += 1;
                num = std::i32::MAX;
            }
        }
        count.max(1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2871() {
        assert_eq!(Solution::max_subarrays(vec![1, 0, 2, 0, 1, 2]), 3);
        assert_eq!(Solution::max_subarrays(vec![5, 7, 1, 3]), 1);
    }
}
