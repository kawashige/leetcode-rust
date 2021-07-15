use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let obstacles = obstacles
            .into_iter()
            .map(|v| (v[0], v[1]))
            .collect::<HashSet<(i32, i32)>>();
        let direction = vec![(0, 1), (-1, 0), (0, -1), (1, 0)];

        let mut d = 0;

        let mut max = 0;
        let mut p = (0, 0);
        for command in commands {
            match command {
                -2 => {
                    d = (d + 1) % 4;
                }
                -1 => {
                    d = (d + 4 - 1) % 4;
                }
                _ => {
                    let k = (1..=command)
                        .find(|i| {
                            obstacles
                                .contains(&(p.0 + direction[d].0 * i, p.1 + direction[d].1 * i))
                        })
                        .unwrap_or(command + 1);
                    p = (
                        p.0 + direction[d].0 * (k - 1),
                        p.1 + direction[d].1 * (k - 1),
                    );
                    max = std::cmp::max(max, p.0 * p.0 + p.1 * p.1);
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0874() {
        assert_eq!(Solution::robot_sim(vec![4, -1, 3], vec![]), 25);
        assert_eq!(
            Solution::robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]),
            65
        );
    }
}
