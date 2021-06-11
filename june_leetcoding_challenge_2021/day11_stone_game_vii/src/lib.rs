pub struct Solution {}

impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let turn = (stones.len() - 1) % 2;
        let mut dp = vec![vec![0; stones.len()]; stones.len()];
        let acc = std::iter::once(0)
            .chain(stones.iter().scan(0, |sum, x| {
                *sum += x;
                Some(*sum)
            }))
            .collect::<Vec<i32>>();

        for l in 1..stones.len() {
            for i in 0..(stones.len() - l) {
                dp[i][i + l] = if l % 2 == turn {
                    std::cmp::max(
                        dp[i + 1][i + l] + acc[i + l + 1] - acc[i + 1],
                        dp[i][i + l - 1] + acc[i + l] - acc[i],
                    )
                } else {
                    std::cmp::min(
                        dp[i + 1][i + l] - acc[i + l + 1] + acc[i + 1],
                        dp[i][i + l - 1] - acc[i + l] + acc[i],
                    )
                };
            }
        }

        dp[0][stones.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day11() {
        assert_eq!(Solution::stone_game_vii(vec![5, 3, 1, 4, 2]), 6);
        assert_eq!(
            Solution::stone_game_vii(vec![7, 90, 5, 1, 100, 10, 10, 2]),
            122
        );
    }
}
