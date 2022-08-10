pub struct Solution {}

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        const M: usize = 1_000_000_007;

        let mut positive = vec![vec![0; grid[0].len()]; grid.len()];
        let mut negative = vec![vec![0; grid[0].len()]; grid.len()];
        let mut has_zero = false;

        if grid[0][0] < 0 {
            negative[0][0] = grid[0][0].abs() as usize;
        } else {
            positive[0][0] = grid[0][0] as usize;
        }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    has_zero = true;
                    continue;
                }
                let mut target = Vec::new();
                if 0 < i {
                    target.push((i - 1, j));
                }
                if 0 < j {
                    target.push((i, j - 1));
                }

                for (x, y) in target {
                    if grid[i][j] < 0 {
                        positive[i][j] =
                            positive[i][j].max(negative[x][y] * grid[i][j].abs() as usize);
                        negative[i][j] =
                            negative[i][j].max(positive[x][y] * grid[i][j].abs() as usize);
                    } else if 0 < grid[i][j] {
                        positive[i][j] = positive[i][j].max(positive[x][y] * grid[i][j] as usize);
                        negative[i][j] = negative[i][j].max(negative[x][y] * grid[i][j] as usize);
                    }
                }
            }
        }

        if 0 < positive[grid.len() - 1][grid[0].len() - 1] {
            (positive[grid.len() - 1][grid[0].len() - 1] % M) as i32
        } else if has_zero {
            0
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1594() {
        assert_eq!(
            Solution::max_product_path(vec![
                vec![2, 1, 3, 0, -3, 3, -4, 4, 0, -4],
                vec![-4, -3, 2, 2, 3, -3, 1, -1, 1, -2],
                vec![-2, 0, -4, 2, 4, -3, -4, -1, 3, 4],
                vec![-1, 0, 1, 0, -3, 3, -2, -3, 1, 0],
                vec![0, -1, -2, 0, -3, -4, 0, 3, -2, -2],
                vec![-4, -2, 0, -1, 0, -3, 0, 4, 0, -3],
                vec![-3, -4, 2, 1, 0, -4, 2, -4, -1, -3],
                vec![3, -2, 0, -4, 1, 0, 1, -3, -1, -1],
                vec![3, -4, 0, 2, 0, -2, 2, -4, -2, 4],
                vec![0, 4, 0, -3, -4, 3, 3, -1, -2, -2]
            ]),
            19215865
        );
        assert_eq!(
            Solution::max_product_path(vec![
                vec![1, 4, 4, 0],
                vec![-2, 0, 0, 1],
                vec![1, -1, 1, 1]
            ]),
            2
        );
        assert_eq!(
            Solution::max_product_path(vec![vec![-1, -2, -3], vec![-2, -3, -3], vec![-3, -3, -2]]),
            -1
        );
        assert_eq!(
            Solution::max_product_path(vec![vec![1, -2, 1], vec![1, -2, 1], vec![3, -4, 1]]),
            8
        );
        assert_eq!(Solution::max_product_path(vec![vec![1, 3], vec![0, -4]]), 0);
    }
}
