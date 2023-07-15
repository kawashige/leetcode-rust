use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut days = events
            .iter()
            .map(|e| vec![e[0], e[1]])
            .flatten()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        days.sort_unstable();
        let day_indices = days.into_iter().zip(1..).collect::<HashMap<_, _>>();

        let mut events = events;
        events.sort_unstable();

        let mut dp = vec![vec![-1; k as usize + 1]; day_indices.len() + 1];
        dp[0][0] = 0;
        let mut accumulated = 0;

        for e in events {
            for i in accumulated + 1..day_indices[&e[0]] {
                for j in 0..dp[i].len() {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j]);
                }
            }
            accumulated = day_indices[&e[0]] - 1;

            for i in 0..k as usize {
                if dp[day_indices[&e[0]] - 1][i] != -1 {
                    dp[day_indices[&e[1]]][i + 1] =
                        dp[day_indices[&e[1]]][i + 1].max(dp[day_indices[&e[0]] - 1][i] + e[2]);
                }
            }
        }
        for i in accumulated + 1..=day_indices.len() {
            for j in 0..dp[i].len() {
                dp[i][j] = dp[i - 1][j].max(dp[i][j]);
            }
        }

        dp.into_iter().last().unwrap().into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1751() {
        assert_eq!(
            Solution::max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]], 2),
            7
        );
        assert_eq!(
            Solution::max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 10]], 2),
            10
        );
        assert_eq!(
            Solution::max_value(
                vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3], vec![4, 4, 4]],
                3
            ),
            9
        );
    }
}
