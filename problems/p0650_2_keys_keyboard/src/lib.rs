pub struct Solution {}

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = (0..=n).collect::<Vec<_>>();
        dp[1] = 0;
        for i in 1..=n {
            for j in (2..=n).take_while(|j| i * j <= n) {
                dp[i * j] = std::cmp::min(dp[i * j], dp[i] + j);
            }
        }
        dp[n] as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0650() {
        assert_eq!(Solution::min_steps(10), 7);
        assert_eq!(Solution::min_steps(3), 3);
    }
}
