pub struct Solution {}

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_sum = vec![vec![0; grid[0].len()]; grid.len()];
        let mut column_sum = vec![vec![0; grid[0].len()]; grid.len()];
        let mut diagonal_sum1 = vec![vec![0; grid[0].len()]; grid.len()];
        let mut diagonal_sum2 = vec![vec![0; grid[0].len()]; grid.len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                row_sum[i][j] = grid[i][j] + if j == 0 { 0 } else { row_sum[i][j - 1] };
                column_sum[i][j] = grid[i][j] + if i == 0 { 0 } else { column_sum[i - 1][j] };
                diagonal_sum1[i][j] = grid[i][j]
                    + if i == 0 || j == 0 {
                        0
                    } else {
                        diagonal_sum1[i - 1][j - 1]
                    };
                diagonal_sum2[i][j] = grid[i][j]
                    + if i == 0 || j == grid[0].len() - 1 {
                        0
                    } else {
                        diagonal_sum2[i - 1][j + 1]
                    };
            }
        }

        println!("r: {:?}", row_sum);
        println!("c: {:?}", column_sum);

        let mut max_size = 1;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let l = (grid.len() - 1 - i).min(grid[0].len() - 1 - j);
                for k in (1..=l).rev() {
                    if k < max_size {
                        break;
                    }
                    let target = row_sum[i][j + k] - if 0 < j { row_sum[i][j - 1] } else { 0 };
                    if (1..=k).all(|m| {
                        target
                            == row_sum[i + m][j + k] - if 0 < j { row_sum[i + m][j - 1] } else { 0 }
                    }) && (0..=k).all(|m| {
                        target
                            == column_sum[i + k][j + m]
                                - if 0 < i { column_sum[i - 1][j + m] } else { 0 }
                    }) && target
                        == diagonal_sum1[i + k][j + k]
                            - if i == 0 || j == 0 {
                                0
                            } else {
                                diagonal_sum1[i - 1][j - 1]
                            }
                        && target
                            == diagonal_sum2[i + k][j]
                                - if i == 0 || j + k == grid[0].len() - 1 {
                                    0
                                } else {
                                    diagonal_sum2[i - 1][j + k + 1]
                                }
                    {
                        max_size = k + 1;
                        break;
                    }
                }
            }
        }

        max_size as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1895() {
        assert_eq!(
            Solution::largest_magic_square(vec![
                vec![1, 17, 15, 17, 5, 16, 8, 9],
                vec![1, 19, 11, 18, 8, 18, 3, 18],
                vec![6, 6, 5, 8, 3, 15, 6, 11],
                vec![19, 5, 6, 11, 9, 2, 14, 13],
                vec![12, 16, 16, 15, 14, 18, 10, 7],
                vec![3, 11, 15, 15, 7, 1, 9, 8],
                vec![15, 5, 11, 17, 18, 20, 14, 17],
                vec![13, 17, 7, 20, 12, 2, 13, 19]
            ]),
            1
        );
        assert_eq!(
            Solution::largest_magic_square(vec![
                vec![7, 1, 4, 5, 6],
                vec![2, 5, 1, 6, 4],
                vec![1, 5, 4, 3, 2],
                vec![1, 2, 7, 3, 4]
            ]),
            3
        );
        assert_eq!(
            Solution::largest_magic_square(vec![
                vec![5, 1, 3, 1],
                vec![9, 3, 3, 1],
                vec![1, 3, 3, 8]
            ]),
            2
        );
    }
}
