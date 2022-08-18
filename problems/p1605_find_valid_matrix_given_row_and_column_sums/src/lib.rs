pub struct Solution {}

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; col_sum.len()]; row_sum.len()];

        loop {
            match (
                (0..row_sum.len()).filter(|i| 0 < row_sum[*i]).min(),
                (0..col_sum.len()).filter(|i| 0 < col_sum[*i]).min(),
            ) {
                (Some(row), Some(col)) => {
                    if row_sum[row] < col_sum[col] {
                        let target_col = (0..col_sum.len()).find(|i| 0 < col_sum[*i]).unwrap();
                        result[row][target_col] += row_sum[row];
                        col_sum[target_col] -= row_sum[row];
                        row_sum[row] = 0;
                    } else {
                        let target_row = (0..row_sum.len()).find(|i| 0 < row_sum[*i]).unwrap();
                        result[target_row][col] += col_sum[col];
                        row_sum[target_row] -= col_sum[col];
                        col_sum[col] = 0;
                    }
                }
                _ => break,
            }
            println!("{:?}", result);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1605() {
        assert_eq!(
            Solution::restore_matrix(vec![3, 8], vec![4, 7]),
            vec![vec![3, 0], vec![1, 7]]
        );
        assert_eq!(
            Solution::restore_matrix(vec![5, 7, 10], vec![8, 6, 8]),
            vec![vec![5, 0, 0], vec![3, 4, 0], vec![0, 2, 8]]
        );
    }
}
