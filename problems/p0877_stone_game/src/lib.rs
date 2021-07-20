pub struct Solution {}

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let mut dp = vec![vec![0; piles.len()]; piles.len()];
        for i in 0..piles.len() {
            dp[i][i] = piles[i];
        }

        for i in 1..piles.len() {
            for j in 0..(piles.len() - i) {
                dp[j][i + j] = (piles[j] - dp[j + 1][j + i]).max(piles[i + j] - dp[j][i + j - 1]);
            }
        }

        dp[0][piles.len() - 1] > 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0877() {
        assert!(Solution::stone_game(vec![5, 3, 4, 5]));
        assert!(Solution::stone_game(vec![1, 3]));
    }
}
