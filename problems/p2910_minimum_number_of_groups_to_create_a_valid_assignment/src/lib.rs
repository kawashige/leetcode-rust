use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_groups_for_valid_assignment(balls: Vec<i32>) -> i32 {
        let mut count = balls
            .into_iter()
            .fold(HashMap::new(), |mut count, ball| {
                *count.entry(ball).or_insert(0) += 1;
                count
            })
            .values()
            .cloned()
            .collect::<Vec<_>>();
        count.sort_unstable();

        let mut min_groups = std::i32::MAX;
        for s in 1..=count[0] {
            let mut groups = 0;
            let mut is_ok = true;
            for i in 0..count.len() {
                let a = count[i] / (s + 1);
                let b = count[i] % (s + 1);
                if b == 0 {
                    groups += a;
                } else if s - b <= a {
                    groups += a + 1;
                } else {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                min_groups = min_groups.min(groups);
            }
        }

        min_groups
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2910() {
        assert_eq!(
            Solution::min_groups_for_valid_assignment(vec![3, 2, 3, 2, 3]),
            2
        );
        assert_eq!(
            Solution::min_groups_for_valid_assignment(vec![10, 10, 10, 3, 1, 1]),
            4
        );
    }
}
