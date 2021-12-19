pub struct Solution {}

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut count = matrix
            .iter()
            .filter(|rows| rows.iter().all(|x| x == &0) || rows.iter().all(|x| x == &1))
            .count();

        for i in 0..matrix.len() {
            if matrix[i].iter().all(|x| x == &0) || matrix[i].iter().all(|x| x == &1) {
                continue;
            }

            let tmp_count = (0..matrix.len())
                .filter(|j| {
                    j != &i
                        && ((0..matrix[0].len()).all(|k| matrix[i][k] ^ matrix[*j][k] == 0)
                            || (0..matrix[0].len()).all(|k| matrix[i][k] ^ matrix[*j][k] == 1))
                })
                .count();
            count = count.max(tmp_count + 1);
        }

        count as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1072() {
        assert_eq!(
            Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]]),
            1
        );
        assert_eq!(
            Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 0]]),
            2
        );
        assert_eq!(
            Solution::max_equal_rows_after_flips(vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]),
            2
        );
    }
}
