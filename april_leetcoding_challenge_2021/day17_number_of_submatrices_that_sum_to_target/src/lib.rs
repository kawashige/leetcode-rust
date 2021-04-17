pub struct Solution {}

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let mut sum = vec![vec![0; matrix[0].len()]; matrix.len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                sum[i][j] = matrix[i][j];
                if i > 0 {
                    sum[i][j] += sum[i - 1][j];
                }
                if j > 0 {
                    sum[i][j] += sum[i][j - 1];
                }
                if i > 0 && j > 0 {
                    sum[i][j] -= sum[i - 1][j - 1];
                }
            }
        }

        let mut count = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                for k in i..matrix.len() {
                    for l in j..matrix[0].len() {
                        let mut s = sum[k][l];
                        if i > 0 {
                            s -= sum[i - 1][l];
                        }
                        if j > 0 {
                            s -= sum[k][j - 1];
                        }
                        if i > 0 && j > 0 {
                            s += sum[i - 1][j - 1];
                        }
                        if s == target {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day17() {
        assert_eq!(
            Solution::num_submatrix_sum_target(
                vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]],
                0
            ),
            4
        );
        assert_eq!(
            Solution::num_submatrix_sum_target(vec![vec![1, -1], vec![-1, 1]], 0),
            5
        );
        assert_eq!(Solution::num_submatrix_sum_target(vec![vec![904]], 0), 0);
    }
}
