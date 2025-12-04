pub struct Solution {}

impl Solution {
    pub fn max_score(
        n: i32,
        k: i32,
        stay_score: Vec<Vec<i32>>,
        travel_score: Vec<Vec<i32>>,
    ) -> i32 {
        let mut dp = vec![vec![0; n as usize]; k as usize + 1];
        for i in 0..k as usize {
            for curr in 0..n as usize {
                for dest in 0..n as usize {
                    dp[i + 1][dest] = dp[i + 1][dest].max(
                        dp[i][curr]
                            + if curr == dest {
                                stay_score[i][curr]
                            } else {
                                travel_score[curr][dest]
                            },
                    );
                }
            }
        }
        dp.into_iter().last().unwrap().into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3332() {
        assert_eq!(
            Solution::max_score(2, 1, vec![vec![2, 3]], vec![vec![0, 2], vec![1, 0]]),
            3
        );
        assert_eq!(
            Solution::max_score(
                3,
                2,
                vec![vec![3, 4, 2], vec![2, 1, 2]],
                vec![vec![0, 2, 1], vec![2, 0, 4], vec![3, 2, 0]]
            ),
            8
        );
    }
}
