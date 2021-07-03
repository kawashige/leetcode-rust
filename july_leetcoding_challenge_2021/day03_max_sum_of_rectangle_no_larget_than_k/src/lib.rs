pub struct Solution {}

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut acc = vec![vec![0; matrix[0].len()]; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                acc[i][j] = matrix[i][j];
                if i > 0 {
                    acc[i][j] += acc[i - 1][j];
                }
                if j > 0 {
                    acc[i][j] += acc[i][j - 1];
                }
                if i > 0 && j > 0 {
                    acc[i][j] -= acc[i - 1][j - 1];
                }
            }
        }

        let mut max = std::i32::MIN;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                for l in i..matrix.len() {
                    for m in j..matrix[0].len() {
                        let mut sum = acc[l][m];
                        if i > 0 {
                            sum -= acc[i - 1][m];
                        }
                        if j > 0 {
                            sum -= acc[l][j - 1];
                        }
                        if i > 0 && j > 0 {
                            sum += acc[i - 1][j - 1];
                        }
                        if sum <= k {
                            max = std::cmp::max(max, sum);
                        }
                    }
                }
            }
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day03() {
        assert_eq!(
            Solution::max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2),
            2
        );
        assert_eq!(Solution::max_sum_submatrix(vec![vec![2, 2, -1]], 3), 3);
        assert_eq!(Solution::max_sum_submatrix(vec![vec![2, 2, -1]], 0), -1);
    }
}
