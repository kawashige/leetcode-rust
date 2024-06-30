pub struct Solution {}

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| b.cmp(&a));
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i] as i64;
            if sum <= 0 {
                return i as i32;
            }
        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2587() {
        assert_eq!(Solution::max_score(vec![2, -1, 0, 1, -3, 3, -3]), 6);
        assert_eq!(Solution::max_score(vec![-2, -3, 0]), 0);
    }
}
