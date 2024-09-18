pub struct Solution {}

impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        const M: i64 = 1_000_000_007;

        let mut nums = (0..nums.len())
            .map(|i| nums[i] as i64 + if s.as_bytes()[i] == b'L' { -1 } else { 1 } * d as i64)
            .collect::<Vec<_>>();
        nums.sort_unstable();

        let mut result = 0;
        let mut prev = 0;
        for i in 1..nums.len() {
            prev = prev + (nums[i] - nums[i - 1]) * i as i64;
            result += prev;
            result %= M;
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2731() {
        assert_eq!(
            Solution::sum_distance(vec![-10, -13, 10, 14, 11], "LRLLR".to_string(), 2),
            146
        );
        assert_eq!(
            Solution::sum_distance(vec![-2, 0, 2], "RLL".to_string(), 3),
            8
        );
        assert_eq!(Solution::sum_distance(vec![1, 0], "RL".to_string(), 2), 5);
    }
}
