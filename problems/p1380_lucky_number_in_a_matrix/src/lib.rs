pub struct Solution {}

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut row_min = vec![std::i32::MAX; matrix.len()];
        let mut column_max = vec![0; matrix[0].len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                row_min[i] = row_min[i].min(matrix[i][j]);
                column_max[j] = column_max[j].max(matrix[i][j]);
            }
        }

        let mut result = Vec::new();
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == row_min[i] && matrix[i][j] == column_max[j] {
                    result.push(matrix[i][j]);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1380() {
        assert_eq!(
            Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
            vec![15]
        );
        assert_eq!(
            Solution::lucky_numbers(vec![
                vec![1, 10, 4, 2],
                vec![9, 3, 8, 7],
                vec![15, 16, 17, 12]
            ]),
            vec![12]
        );
        assert_eq!(
            Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2]]),
            vec![7]
        );
    }
}
