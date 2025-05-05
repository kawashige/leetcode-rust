pub struct Solution {}

impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let mut result = 1;
        let mut l = 1;
        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                l = 1;
            } else {
                l += 1;
            }
            result += l;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3101() {
        assert_eq!(Solution::count_alternating_subarrays(vec![0, 1, 1, 1]), 5);
        assert_eq!(Solution::count_alternating_subarrays(vec![1, 0, 1, 0]), 10);
    }
}
