pub struct Solution {}

impl Solution {
    pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = grid.clone();
        let (x, y) = (x as usize, y as usize);
        let k = k as usize;
        for i in y..y + k {
            for j in 0..k {
                result[x + j][i] = grid[x + k - 1 - j][i];
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3643() {
        assert_eq!(
            Solution::reverse_submatrix(
                vec![
                    vec![1, 2, 3, 4],
                    vec![5, 6, 7, 8],
                    vec![9, 10, 11, 12],
                    vec![13, 14, 15, 16]
                ],
                1,
                0,
                3
            ),
            vec![
                vec![1, 2, 3, 4],
                vec![13, 14, 15, 8],
                vec![9, 10, 11, 12],
                vec![5, 6, 7, 16]
            ]
        );
        assert_eq!(
            Solution::reverse_submatrix(vec![vec![3, 4, 2, 3], vec![2, 3, 4, 2]], 0, 2, 2),
            vec![vec![3, 4, 4, 2], vec![2, 3, 2, 3]]
        );
    }
}
