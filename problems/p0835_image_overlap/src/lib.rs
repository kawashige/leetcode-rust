use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        let mut indicies_a = Vec::with_capacity(a.len() * a[0].len());
        let mut indicies_b = Vec::with_capacity(b.len() * b[0].len());
        for i in 0..a.len() {
            for j in 0..a[0].len() {
                if a[i][j] == 1 {
                    indicies_a.push((i as i32, j as i32));
                }
                if b[i][j] == 1 {
                    indicies_b.push((i as i32, j as i32));
                }
            }
        }

        let mut count = HashMap::new();

        for (i_b, j_b) in indicies_b {
            for (i_a, j_a) in &indicies_a {
                *count.entry((i_a - i_b, j_a - j_b)).or_insert(0) += 1
            }
        }

        *count.values().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0835() {
        assert_eq!(
            Solution::largest_overlap(
                vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]]
            ),
            3
        );
        assert_eq!(Solution::largest_overlap(vec![vec![1]], vec![vec![1]]), 1);
        assert_eq!(Solution::largest_overlap(vec![vec![0]], vec![vec![0]]), 0);
    }
}
