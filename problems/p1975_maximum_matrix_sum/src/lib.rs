pub struct Solution {}

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut min_abs = std::i64::MAX;
        let mut sum = 0;
        let mut negative_count = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                negative_count += (matrix[i][j] < 0) as usize;
                min_abs = min_abs.min(matrix[i][j].abs() as i64);
                sum += matrix[i][j].abs() as i64;
            }
        }

        if negative_count % 2 == 1 {
            sum - min_abs * 2
        } else {
            sum
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1975() {
        assert_eq!(Solution::max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]), 4);
        assert_eq!(
            Solution::max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]]),
            16
        );
    }
}
