pub struct Solution {}

impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut result = 0;
        let mut min_value = nums[0] - k;
        for i in 0..nums.len() {
            min_value = min_value.max(nums[i] - k);
            if (nums[i] - k..=nums[i] + k).contains(&min_value) {
                min_value += 1;
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
    fn test_3397() {
        assert_eq!(
            Solution::max_distinct_elements(vec![1, 2, 2, 3, 3, 4], 2),
            6
        );
        assert_eq!(Solution::max_distinct_elements(vec![4, 4, 4, 4], 1), 3);
    }
}
