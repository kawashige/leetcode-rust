pub struct Solution {}

impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut column_max = vec![-1; matrix[0].len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                column_max[j] = column_max[j].max(matrix[i][j]);
            }
        }

        let mut result = vec![vec![-1; matrix[0].len()]; matrix.len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                result[i][j] = if matrix[i][j] == -1 {
                    column_max[j]
                } else {
                    matrix[i][j]
                };
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3033() {
        assert_eq!(
            Solution::modified_matrix(vec![vec![1, 2, -1], vec![4, -1, 6], vec![7, 8, 9]]),
            vec![vec![1, 2, 9], vec![4, 8, 6], vec![7, 8, 9]]
        );
        assert_eq!(
            Solution::modified_matrix(vec![vec![3, -1], vec![5, 2]]),
            vec![vec![3, 2], vec![5, 2]]
        );
    }
}
