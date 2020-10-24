pub struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut squares = Vec::new();
        for i in 1..=n {
            let square = i.pow(2);
            if n < square {
                break;
            }
            squares.push(square);
        }

        let mut dp = vec![vec![std::i32::MAX - 10000; n as usize + 1]; squares.len() + 2];
        dp[0][0] = 0;
        for i in 0..squares.len() {
            for j in 0..=(n as usize) {
                dp[i + 1][j] = std::cmp::min(dp[i][j], dp[i + 1][j]);
                if j >= squares[i] as usize {
                    dp[i + 1][j] = std::cmp::min(dp[i][j - squares[i] as usize] + 1, dp[i + 1][j]);
                    dp[i + 1][j] =
                        std::cmp::min(dp[i + 1][j - squares[i] as usize] + 1, dp[i + 1][j]);
                }
            }
        }
        dp[squares.len()][n as usize]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0279() {
        assert_eq!(1, Solution::num_squares(1));
        assert_eq!(2, Solution::num_squares(2));
        assert_eq!(3, Solution::num_squares(12));
        assert_eq!(2, Solution::num_squares(13));
        assert_eq!(3, Solution::num_squares(70));
        assert_eq!(3, Solution::num_squares(499));
        assert_eq!(4, Solution::num_squares(496));
    }
}
