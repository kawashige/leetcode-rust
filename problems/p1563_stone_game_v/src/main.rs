pub struct Solution {}

impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();
        let mut dp = vec![vec![0; n]; n];

        let mut acc = vec![0; n + 1];
        for i in 0..n {
            acc[i + 1] = acc[i] + stone_value[i];
        }

        for l in 2..=n {
            for i in 0..n + 1 - l {
                let sum = acc[i + l] - acc[i];
                let mut max = 0;
                for j in i..i + l - 1 {
                    let value = if sum - (acc[j + 1] - acc[i]) == acc[j + 1] - acc[i] {
                        sum - (acc[j + 1] - acc[i]) + dp[j + 1][i + l - 1].max(dp[i][j])
                    } else if sum - (acc[j + 1] - acc[i]) < acc[j + 1] - acc[i] {
                        sum - (acc[j + 1] - acc[i]) + dp[j + 1][i + l - 1]
                    } else {
                        acc[j + 1] - acc[i] + dp[i][j]
                    };
                    max = max.max(value);
                }
                dp[i][i + l - 1] = max;
            }
        }

        dp[0][n - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1563() {
        assert_eq!(Solution::stone_game_v(vec![1, 1, 2]), 0);
        assert_eq!(Solution::stone_game_v(vec![6, 2, 3, 4, 5, 5]), 18);
        assert_eq!(Solution::stone_game_v(vec![7, 7, 7, 7, 7, 7, 7]), 28);
        assert_eq!(Solution::stone_game_v(vec![4]), 0);
    }
}

fn main() {}
