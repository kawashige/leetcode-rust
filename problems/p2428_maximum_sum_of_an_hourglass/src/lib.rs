pub struct Solution {}

impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut sum = vec![vec![0; grid[0].len()]; grid.len()];
        let mut max = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                sum[i][j] = grid[i][j];
                if 0 < i {
                    sum[i][j] += sum[i - 1][j];
                }
                if 0 < j {
                    sum[i][j] += sum[i][j - 1];
                }
                if 0 < i && 0 < j {
                    sum[i][j] -= sum[i - 1][j - 1];
                }

                if 1 < i && 1 < j {
                    let mut tmp_sum = sum[i][j] - grid[i - 1][j] - grid[i - 1][j - 2];
                    if 2 < i {
                        tmp_sum -= sum[i - 3][j];
                    }
                    if 2 < j {
                        tmp_sum -= sum[i][j - 3];
                    }
                    if 2 < i && 2 < j {
                        tmp_sum += sum[i - 3][j - 3];
                    }
                    max = max.max(tmp_sum);
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
    fn test_2428() {
        assert_eq!(
            Solution::max_sum(vec![
                vec![6, 2, 1, 3],
                vec![4, 2, 1, 5],
                vec![9, 2, 8, 7],
                vec![4, 1, 2, 9]
            ]),
            30
        );
        assert_eq!(
            Solution::max_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            35
        );
    }
}
