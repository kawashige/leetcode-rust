pub struct Solution {}

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; n as usize]; n as usize];

        for q in queries {
            matrix[q[0] as usize][q[1] as usize] += 1;
            if q[2] != n - 1 {
                matrix[q[2] as usize + 1][q[1] as usize] -= 1;
            }
            if q[3] != n - 1 {
                matrix[q[0] as usize][q[3] as usize + 1] -= 1;
            }
            if q[2] != n - 1 && q[3] != n - 1 {
                matrix[q[2] as usize + 1][q[3] as usize + 1] += 1;
            }
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if 0 < i {
                    matrix[i][j] += matrix[i - 1][j];
                }
                if 0 < j {
                    matrix[i][j] += matrix[i][j - 1];
                }
                if 0 < i && 0 < j {
                    matrix[i][j] -= matrix[i - 1][j - 1];
                }
            }
        }

        matrix
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2536() {
        assert_eq!(
            Solution::range_add_queries(3, vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]]),
            vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]]
        );
        assert_eq!(
            Solution::range_add_queries(2, vec![vec![0, 0, 1, 1]]),
            vec![vec![1, 1], vec![1, 1]]
        );
    }
}
