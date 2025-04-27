pub struct Solution {}

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() - 2 {
            if (nums[i] + nums[i + 2]) * 2 == nums[i + 1] {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3392() {
        assert_eq!(Solution::count_subarrays(vec![1, 2, 1, 4, 1]), 1);
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1]), 0);
    }
}
