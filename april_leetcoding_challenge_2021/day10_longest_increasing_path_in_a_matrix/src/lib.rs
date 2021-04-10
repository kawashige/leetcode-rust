use std::collections::BTreeMap;
pub struct Solution {}

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut map = BTreeMap::new();
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] > 0 {
                    (*map.entry(matrix[i][j]).or_insert(Vec::new())).push((i, j));
                }
            }
        }

        let mut len = vec![vec![1; matrix[0].len()]; matrix.len()];
        let mut max_len = 0;
        for (_, v) in map {
            for (i, j) in v {
                if i > 0 && matrix[i - 1][j] < matrix[i][j] {
                    len[i][j] = std::cmp::max(len[i - 1][j] + 1, len[i][j]);
                }
                if j > 0 && matrix[i][j - 1] < matrix[i][j] {
                    len[i][j] = std::cmp::max(len[i][j - 1] + 1, len[i][j]);
                }
                if i < matrix.len() - 1 && matrix[i + 1][j] < matrix[i][j] {
                    len[i][j] = std::cmp::max(len[i + 1][j] + 1, len[i][j]);
                }
                if j < matrix[0].len() - 1 && matrix[i][j + 1] < matrix[i][j] {
                    len[i][j] = std::cmp::max(len[i][j + 1] + 1, len[i][j]);
                }
                max_len = std::cmp::max(max_len, len[i][j]);
            }
        }

        max_len
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day10() {
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
            4
        );
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]),
            4
        );
        assert_eq!(Solution::longest_increasing_path(vec![vec![1]]), 1);
        assert_eq!(Solution::longest_increasing_path(vec![vec![1, 2]]), 2);
    }
}
