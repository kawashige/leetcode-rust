pub struct Solution {}

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        for k in 0..nums.len() {
            for j in 0..k {
                for i in 0..j {
                    result = result.max((nums[i] as i64 - nums[j] as i64) * nums[k] as i64);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2873() {
        assert_eq!(Solution::maximum_triplet_value(vec![12, 6, 1, 2, 7]), 77);
        assert_eq!(Solution::maximum_triplet_value(vec![1, 10, 3, 4, 19]), 133);
        assert_eq!(Solution::maximum_triplet_value(vec![1, 2, 3]), 0);
    }
}
