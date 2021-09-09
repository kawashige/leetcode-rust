pub struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        for i in 1..matrix.len() {
            for j in 0..matrix[0].len() {
                let mut x = matrix[i - 1][j];
                if j > 0 {
                    x = x.min(matrix[i - 1][j - 1])
                }
                if j < matrix[0].len() - 1 {
                    x = x.min(matrix[i - 1][j + 1])
                }
                matrix[i][j] += x;
            }
        }

        matrix
            .into_iter()
            .last()
            .unwrap()
            .into_iter()
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0931() {
        assert_eq!(
            Solution::min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]]),
            13
        );
        assert_eq!(
            Solution::min_falling_path_sum(vec![vec![-19, 57], vec![-40, -5]]),
            -59
        );
    }
}
