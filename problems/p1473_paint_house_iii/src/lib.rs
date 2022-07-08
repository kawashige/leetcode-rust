pub struct Solution {}

impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, _m: i32, n: i32, target: i32) -> i32 {
        let mut dp = vec![vec![std::i32::MAX; target as usize + 1]; n as usize + 1];
        dp[0][0] = 0;

        for i in 0..houses.len() {
            let mut new_dp = vec![vec![std::i32::MAX; target as usize + 1]; n as usize + 1];
            if houses[i] == 0 {
                for color in 0..=n as usize {
                    for neiborhodds in 0..=target as usize {
                        for new_color in 1..=n as usize {
                            let new_neiborhoods =
                                neiborhodds + if color == new_color { 0 } else { 1 };
                            if new_neiborhoods <= target as usize
                                && dp[color][neiborhodds] != std::i32::MAX
                            {
                                new_dp[new_color][new_neiborhoods] = new_dp[new_color]
                                    [new_neiborhoods]
                                    .min(dp[color][neiborhodds] + cost[i][new_color - 1])
                            }
                        }
                    }
                }
            } else {
                for color in 0..=n as usize {
                    for neiborhodds in 0..=target as usize {
                        let new_neiborhoods =
                            neiborhodds + if color == houses[i] as usize { 0 } else { 1 };
                        if new_neiborhoods <= target as usize
                            && dp[color][neiborhodds] != std::i32::MAX
                        {
                            new_dp[houses[i] as usize][new_neiborhoods] = new_dp[houses[i] as usize]
                                [new_neiborhoods]
                                .min(dp[color][neiborhodds])
                        }
                    }
                }
            }
            dp = new_dp;
        }

        let result = (1..=n)
            .map(|i| dp[i as usize][target as usize])
            .min()
            .unwrap();
        if result == std::i32::MAX {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1473() {
        assert_eq!(
            Solution::min_cost(
                vec![0, 0, 0, 0, 0],
                vec![
                    vec![1, 10],
                    vec![10, 1],
                    vec![10, 1],
                    vec![1, 10],
                    vec![5, 1]
                ],
                5,
                2,
                3
            ),
            9
        );
        assert_eq!(
            Solution::min_cost(
                vec![0, 2, 1, 2, 0],
                vec![
                    vec![1, 10],
                    vec![10, 1],
                    vec![10, 1],
                    vec![1, 10],
                    vec![5, 1]
                ],
                5,
                2,
                3
            ),
            11
        );
        assert_eq!(
            Solution::min_cost(
                vec![3, 1, 2, 3],
                vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
                4,
                3,
                3
            ),
            -1
        );
    }
}
