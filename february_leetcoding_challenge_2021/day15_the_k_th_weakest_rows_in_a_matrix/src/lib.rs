use std::collections::BTreeSet;
pub struct Solution {}

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        mat.into_iter()
            .enumerate()
            .fold(BTreeSet::new(), |mut set, (i, m)| {
                set.insert((m.iter().take_while(|x| *x == &1).count(), i));
                set
            })
            .into_iter()
            .take(k as usize)
            .map(|(_, i)| i as i32)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day15() {
        assert_eq!(
            Solution::k_weakest_rows(
                vec![
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 1]
                ],
                3
            ),
            vec![2, 0, 3]
        );
        assert_eq!(
            Solution::k_weakest_rows(
                vec![
                    vec![1, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![1, 0, 0, 0],
                    vec![1, 0, 0, 0]
                ],
                2
            ),
            vec![0, 2]
        );
    }
}
