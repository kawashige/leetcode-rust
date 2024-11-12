pub struct Solution {}

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut sum = -1;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i].to_string().chars().max().unwrap()
                    == nums[j].to_string().chars().max().unwrap()
                {
                    sum = sum.max(nums[i] + nums[j]);
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2815() {
        assert_eq!(Solution::max_sum(vec![112, 131, 411]), -1);
        assert_eq!(Solution::max_sum(vec![2536, 1613, 3366, 162]), 5902);
        assert_eq!(Solution::max_sum(vec![51, 71, 17, 24, 42]), 88);
    }
}
