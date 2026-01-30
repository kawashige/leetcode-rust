use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        let strings = original
            .iter()
            .cloned()
            .chain(changed.iter().cloned())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let indices = strings
            .clone()
            .into_iter()
            .zip(0..)
            .collect::<HashMap<_, _>>();

        let mut dist = vec![vec![std::i64::MAX; indices.len()]; indices.len()];
        for i in 0..original.len() {
            dist[indices[&original[i]]][indices[&changed[i]]] =
                dist[indices[&original[i]]][indices[&changed[i]]].min(cost[i] as i64);
        }
        for i in 0..dist.len() {
            dist[i][i] = 0;
        }
        for k in 0..dist.len() {
            for i in 0..dist.len() {
                for j in 0..dist.len() {
                    if dist[i][k] != std::i64::MAX && dist[k][j] != std::i64::MAX {
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
        }

        let mut dp = vec![std::i64::MAX; source.len() + 1];
        dp[source.len()] = 0;

        for i in (0..source.len()).rev() {
            if source.as_bytes()[i] == target.as_bytes()[i] {
                dp[i] = dp[i + 1];
            }

            for j in 0..strings.len() {
                if !source[i..].starts_with(&strings[j]) {
                    continue;
                }
                if let Some(k) = indices.get(&target[i..i + strings[j].len()]) {
                    if dist[j][*k] != std::i64::MAX && dp[i + strings[j].len()] != std::i64::MAX {
                        dp[i] = dp[i].min(dp[i + strings[j].len()] + dist[j][*k]);
                    }
                }
            }
        }

        if dp[0] == std::i64::MAX {
            -1
        } else {
            dp[0]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2977() {
        assert_eq!(
            Solution::minimum_cost(
                "fjybg".to_string(),
                "apyyt".to_string(),
                vec![
                    "bg".to_string(),
                    "xr".to_string(),
                    "cc".to_string(),
                    "ip".to_string(),
                    "vq".to_string(),
                    "po".to_string(),
                    "ym".to_string(),
                    "rh".to_string(),
                    "vw".to_string(),
                    "lf".to_string(),
                    "lo".to_string(),
                    "ee".to_string(),
                    "qv".to_string(),
                    "yr".to_string(),
                    "f".to_string(),
                    "w".to_string(),
                    "i".to_string(),
                    "u".to_string(),
                    "g".to_string(),
                    "a".to_string(),
                    "e".to_string(),
                    "f".to_string(),
                    "s".to_string(),
                    "r".to_string(),
                    "p".to_string(),
                    "j".to_string(),
                    "o".to_string(),
                    "g".to_string(),
                    "i".to_string(),
                    "u".to_string()
                ],
                vec![
                    "xr".to_string(),
                    "cc".to_string(),
                    "ip".to_string(),
                    "vq".to_string(),
                    "po".to_string(),
                    "ym".to_string(),
                    "rh".to_string(),
                    "vw".to_string(),
                    "lf".to_string(),
                    "lo".to_string(),
                    "ee".to_string(),
                    "qv".to_string(),
                    "yr".to_string(),
                    "yt".to_string(),
                    "w".to_string(),
                    "i".to_string(),
                    "u".to_string(),
                    "g".to_string(),
                    "a".to_string(),
                    "e".to_string(),
                    "f".to_string(),
                    "s".to_string(),
                    "r".to_string(),
                    "p".to_string(),
                    "a".to_string(),
                    "o".to_string(),
                    "g".to_string(),
                    "i".to_string(),
                    "u".to_string(),
                    "p".to_string()
                ],
                vec![
                    97733, 90086, 87125, 85361, 75644, 46301, 21616, 79538, 52507, 95884, 79353,
                    61127, 58665, 96031, 95035, 12116, 41158, 91096, 47819, 88522, 25493, 80186,
                    66981, 87597, 56691, 86820, 89031, 99954, 41271, 39699
                ]
            ),
            1628332
        );
        assert_eq!(
            Solution::minimum_cost(
                "abcd".to_string(),
                "acbe".to_string(),
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "c".to_string(),
                    "e".to_string(),
                    "d".to_string()
                ],
                vec![
                    "b".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "e".to_string(),
                    "b".to_string(),
                    "e".to_string()
                ],
                vec![2, 5, 5, 1, 2, 20]
            ),
            28
        );
        assert_eq!(
            Solution::minimum_cost(
                "abcdefgh".to_string(),
                "acdeeghh".to_string(),
                vec!["bcd".to_string(), "fgh".to_string(), "thh".to_string()],
                vec!["cde".to_string(), "thh".to_string(), "ghh".to_string()],
                vec![1, 3, 5]
            ),
            9
        );
        assert_eq!(
            Solution::minimum_cost(
                "abcdefgh".to_string(),
                "addddddd".to_string(),
                vec!["bcd".to_string(), "defgh".to_string()],
                vec!["ddd".to_string(), "ddddd".to_string()],
                vec![100, 1578]
            ),
            -1
        );
    }
}
