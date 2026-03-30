pub struct Solution {}

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut acc = vec![vec![0; matrix[0].len()]; matrix.len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                acc[i][j] += matrix[i][j];
                if 0 < i && acc[i][j] == 1 {
                    acc[i][j] += acc[i - 1][j];
                }
            }
        }
        let mut submatrix = 0;

        for i in 0..acc.len() {
            acc[i].sort_unstable_by(|a, b| b.cmp(&a));
            for j in 1..acc[i].len() {
                if acc[i][j] != acc[i][j - 1] {
                    submatrix = submatrix.max(acc[i][j - 1] * j as i32);
                }
            }
            submatrix = submatrix.max(acc[i].last().unwrap() * acc[i].len() as i32);
        }

        submatrix
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1727() {
        assert_eq!(
            Solution::largest_submatrix(vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]]),
            4
        );
        assert_eq!(Solution::largest_submatrix(vec![vec![1, 0, 1, 0, 1]]), 3);
        assert_eq!(
            Solution::largest_submatrix(vec![vec![1, 1, 0], vec![1, 0, 1]]),
            2
        );
    }
}
