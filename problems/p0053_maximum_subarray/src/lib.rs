pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut min_sum = 0;
        let mut sum = 0;
        let mut r = std::i32::MIN;

        for i in 0..nums.len() {
            sum += nums[i];
            r = r.max(sum - min_sum);
            min_sum = min_sum.min(sum);
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0053() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
