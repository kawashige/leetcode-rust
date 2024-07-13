pub struct Solution {}

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let mut robots = Vec::with_capacity(positions.len());
        for i in 0..positions.len() {
            robots.push((positions[i], healths[i], directions.as_bytes()[i], i));
        }
        robots.sort_unstable();

        let mut right: Vec<(i32, usize)> = Vec::with_capacity(robots.len());
        let mut left = Vec::with_capacity(robots.len());
        for i in 0..robots.len() {
            if robots[i].2 == b'L' {
                let mut health = robots[i].1;
                while 0 < health && !right.is_empty() {
                    if right[right.len() - 1].0 == health {
                        right.pop();
                        health = 0;
                    } else if right[right.len() - 1].0 < health {
                        right.pop();
                        health -= 1;
                    } else {
                        right.last_mut().unwrap().0 -= 1;
                        if right[right.len() - 1].0 == 0 {
                            right.pop();
                        }
                        health = 0;
                    }
                }
                if 0 < health {
                    left.push((health, robots[i].3));
                }
            } else {
                right.push((robots[i].1, robots[i].3));
            }
        }

        let mut survived = left
            .into_iter()
            .chain(right.into_iter())
            .collect::<Vec<_>>();
        survived.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        survived.into_iter().map(|(h, _)| h).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2751() {
        assert_eq!(
            Solution::survived_robots_healths(
                vec![5, 4, 3, 2, 1],
                vec![2, 17, 9, 15, 10],
                "RRRRR".to_string()
            ),
            vec![2, 17, 9, 15, 10]
        );
        assert_eq!(
            Solution::survived_robots_healths(
                vec![3, 5, 2, 6],
                vec![10, 10, 15, 12],
                "RLRL".to_string()
            ),
            vec![14]
        );
        assert_eq!(
            Solution::survived_robots_healths(
                vec![1, 2, 5, 6],
                vec![10, 10, 11, 11],
                "RLRL".to_string()
            ),
            vec![]
        );
    }
}
