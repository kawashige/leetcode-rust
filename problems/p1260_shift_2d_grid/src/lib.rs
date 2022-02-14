pub struct Solution {}

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; grid[0].len()]; grid.len()];

        for j in 0..grid[0].len() {
            let rotate = (j + k as usize) / grid[0].len();
            for i in 0..grid.len() {
                result[(i + rotate) % grid.len()][(j + k as usize) % grid[0].len()] = grid[i][j];
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1260() {
        assert_eq!(
            Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 0),
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
        );
        assert_eq!(
            Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
            vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
        );
        assert_eq!(
            Solution::shift_grid(
                vec![
                    vec![3, 8, 1, 9],
                    vec![19, 7, 2, 5],
                    vec![4, 6, 11, 10],
                    vec![12, 0, 21, 13]
                ],
                4
            ),
            vec![
                vec![12, 0, 21, 13],
                vec![3, 8, 1, 9],
                vec![19, 7, 2, 5],
                vec![4, 6, 11, 10]
            ]
        );
    }
}
