pub struct Solution {}

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut max = nums[0].max(nums[1]);
        let mut max2 = (nums[0] - nums[1]) as i64;
        for i in 2..nums.len() {
            result = result.max(max2 * nums[i] as i64);
            max2 = max2.max((max - nums[i]) as i64);
            max = max.max(nums[i]);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2874() {
        assert_eq!(Solution::maximum_triplet_value(vec![12, 6, 1, 2, 7]), 77);
        assert_eq!(Solution::maximum_triplet_value(vec![1, 10, 3, 4, 19]), 133);
        assert_eq!(Solution::maximum_triplet_value(vec![1, 2, 3]), 0);
    }
}
