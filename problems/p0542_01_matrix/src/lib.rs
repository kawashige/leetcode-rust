pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![-1; matrix[0].len()]; matrix.len()];
        let mut target = HashSet::new();
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    result[i][j] = 0;
                    target.insert((i, j));
                }
            }
        }

        while !target.is_empty() {
            let mut next = HashSet::new();
            for t in target {
                let v = result[t.0][t.1] + 1;
                if 0 < t.0 && result[t.0 - 1][t.1] == -1 {
                    result[t.0 - 1][t.1] = v;
                    next.insert((t.0 - 1, t.1));
                }
                if 0 < t.1 && result[t.0][t.1 - 1] == -1 {
                    result[t.0][t.1 - 1] = v;
                    next.insert((t.0, t.1 - 1));
                }
                if t.0 < matrix.len() - 1 && result[t.0 + 1][t.1] == -1 {
                    result[t.0 + 1][t.1] = v;
                    next.insert((t.0 + 1, t.1));
                }
                if t.1 < matrix[0].len() - 1 && result[t.0][t.1 + 1] == -1 {
                    result[t.0][t.1 + 1] = v;
                    next.insert((t.0, t.1 + 1));
                }
            }
            target = next;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0542() {
        assert_eq!(
            vec![vec![0], vec![0], vec![0], vec![0], vec![0]],
            Solution::update_matrix(vec![vec![0], vec![0], vec![0], vec![0], vec![0]])
        );
        assert_eq!(
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]],
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]])
        );
        assert_eq!(
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]],
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]])
        )
    }
}
