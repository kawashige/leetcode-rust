pub struct Solution {}

impl Solution {
    pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
        let mut score = 0;
        let mut result = 0;
        for i in 0..nums.len() {
            result += score;
            score = score.max(nums[i] as i64);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3282() {
        assert_eq!(Solution::find_maximum_score(vec![1, 3, 1, 5]), 7);
        assert_eq!(Solution::find_maximum_score(vec![4, 3, 1, 3, 2]), 16);
    }
}
