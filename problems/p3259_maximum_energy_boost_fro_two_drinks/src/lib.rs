pub struct Solution {}

impl Solution {
    pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
        let mut dp = vec![vec![0; 2]; energy_drink_a.len() + 1];
        for i in 0..energy_drink_a.len() {
            dp[i + 1][0] = energy_drink_a[i] as i64
                + if 1 < i {
                    dp[i][0].max(dp[i - 1][1])
                } else {
                    dp[i][0]
                };
            dp[i + 1][1] = energy_drink_b[i] as i64
                + if 1 < i {
                    dp[i][1].max(dp[i - 1][0])
                } else {
                    dp[i][1]
                };
        }
        *dp.last().unwrap().into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3259() {
        assert_eq!(Solution::max_energy_boost(vec![1, 3, 1], vec![3, 1, 1]), 5);
        assert_eq!(Solution::max_energy_boost(vec![4, 1, 1], vec![1, 1, 3]), 7);
    }
}
