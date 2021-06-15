pub struct Solution {}

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 {
            return 1.0;
        }

        let mut dp = vec![0.0; (k + max_pts) as usize];
        let mut acc = vec![0.0; (k + max_pts) as usize];
        dp[0] = 1.0;
        acc[0] = 1.0;
        dp[1] = 1.0 / max_pts as f64;
        acc[1] = acc[0] + dp[1];

        for i in 2..((k + max_pts) as usize) {
            let min = if i <= max_pts as usize {
                0.0
            } else {
                acc[i - 1 - max_pts as usize]
            };
            let max = if i > k as usize {
                acc[k as usize - 1]
            } else {
                acc[i - 1]
            };
            dp[i] = (max - min) / max_pts as f64;
            acc[i] = dp[i] + acc[i - 1];
        }

        (acc[std::cmp::min(n as usize, acc.len() - 1)] - acc[k as usize - 1])
            / (acc[acc.len() - 1] - acc[k as usize - 1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0837() {
        assert_eq!(Solution::new21_game(185, 183, 2), 1.00000);
        assert_eq!(Solution::new21_game(10, 1, 10), 1.00000);
        assert_eq!(Solution::new21_game(6, 1, 10), 0.60000);
        assert_eq!(Solution::new21_game(21, 17, 10), 0.73278);
    }
}
