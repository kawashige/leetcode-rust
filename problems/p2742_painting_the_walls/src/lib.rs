pub struct Solution {}

impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let mut dp = vec![vec![std::i32::MAX; cost.len() + 1]; cost.len() + 1];
        dp[0][0] = 0;

        for i in 0..cost.len() {
            for j in 0..=cost.len() {
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
                if dp[i][j] != std::i32::MAX {
                    let walls = (j + time[i] as usize + 1).min(cost.len());
                    dp[i + 1][walls] = dp[i + 1][walls].min(dp[i][j] + cost[i]);
                }
            }
        }

        dp[cost.len()][cost.len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2742() {
        assert_eq!(
            Solution::paint_walls(vec![8, 7, 5, 15], vec![1, 1, 2, 1]),
            12
        );
        assert_eq!(Solution::paint_walls(vec![1, 2, 3, 2], vec![1, 2, 3, 2]), 3);
        assert_eq!(Solution::paint_walls(vec![2, 3, 4, 2], vec![1, 1, 1, 1]), 4);
    }
}
