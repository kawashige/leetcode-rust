pub struct Solution {}

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let mut result = std::i32::MIN;
        let mut dp = vec![0; energy.len()];

        for i in (0..energy.len()).rev() {
            dp[i] += energy[i];
            if k <= i as i32 {
                dp[i - k as usize] += dp[i];
            }
            result = result.max(dp[i]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3147() {
        assert_eq!(Solution::maximum_energy(vec![5, 2, -10, -5, 1], 3), 3);
        assert_eq!(Solution::maximum_energy(vec![-2, -3, -1], 2), -1);
    }
}
