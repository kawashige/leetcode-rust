pub struct Solution {}

impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let max = rods.iter().sum::<i32>() as usize;
        let mut dp = vec![vec![-1; max + 1]; rods.len() + 1];
        dp[0][max / 2] = 0;

        for i in 0..rods.len() {
            for j in 0..dp[i].len() {
                if dp[i][j] == -1 {
                    continue;
                }
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
                if (j + rods[i] as usize) < dp[i].len() {
                    dp[i + 1][j + rods[i] as usize] =
                        dp[i + 1][j + rods[i] as usize].max(dp[i][j] + rods[i]);
                }
                if (rods[i] as usize) < j {
                    dp[i + 1][j - rods[i] as usize] = dp[i + 1][j - rods[i] as usize].max(dp[i][j]);
                }
            }
        }

        dp[rods.len()][max / 2]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0956() {
        assert_eq!(
            Solution::tallest_billboard(vec![
                243, 269, 278, 237, 208, 279, 229, 231, 262, 256, 248, 261, 232, 275, 254, 224, 264
            ]),
            2125
        );
        assert_eq!(
            Solution::tallest_billboard(vec![
                102, 101, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100
            ]),
            900,
        );
        assert_eq!(Solution::tallest_billboard(vec![1, 2, 3, 6]), 6);
        assert_eq!(Solution::tallest_billboard(vec![1, 2, 3, 4, 5, 6]), 10);
        assert_eq!(Solution::tallest_billboard(vec![1, 2]), 0);
    }
}
