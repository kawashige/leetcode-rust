use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn finding_users_active_minutes(mut logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        logs.sort_unstable();

        let mut uam = HashMap::new();
        for i in 0..logs.len() {
            if i == 0 || !(logs[i - 1][0] == logs[i][0] && logs[i - 1][1] == logs[i][1]) {
                *uam.entry(logs[i][0]).or_insert(0) += 1;
            }
        }

        println!("{:?}", uam);

        let mut result = vec![0; k as usize];

        for x in uam.values() {
            result[*x as usize - 1] += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1817() {
        assert_eq!(
            Solution::finding_users_active_minutes(
                vec![vec![0, 5], vec![1, 2], vec![0, 2], vec![0, 5], vec![1, 3]],
                5
            ),
            vec![0, 2, 0, 0, 0]
        );
        assert_eq!(
            Solution::finding_users_active_minutes(vec![vec![1, 1], vec![2, 2], vec![2, 3]], 4),
            vec![1, 1, 0, 0]
        );
        assert_eq!(
            Solution::finding_users_active_minutes(
                vec![
                    vec![326924761, 47805],
                    vec![326924766, 47806],
                    vec![326924762, 47806],
                    vec![326924765, 47803],
                    vec![326924764, 47802],
                    vec![326924764, 47804],
                    vec![326924765, 47806],
                    vec![326924762, 47806],
                    vec![326924764, 47805]
                ],
                5
            ),
            vec![3, 1, 1, 0, 0]
        );
    }
}
