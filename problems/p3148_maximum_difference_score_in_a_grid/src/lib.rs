pub struct Solution {}

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut max_value = vec![vec![0; grid[0].len()]; grid.len()];
        let mut result = std::i32::MIN;

        for i in (0..grid.len()).rev() {
            for j in (0..grid[0].len()).rev() {
                if i < grid.len() - 1 {
                    max_value[i][j] = max_value[i][j].max(max_value[i + 1][j]);
                }
                if j < grid[0].len() - 1 {
                    max_value[i][j] = max_value[i][j].max(max_value[i][j + 1]);
                }
                if max_value[i][j] != 0 {
                    result = result.max(max_value[i][j] - grid[i][j]);
                }
                max_value[i][j] = max_value[i][j].max(grid[i][j]);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3142() {
        assert_eq!(
            Solution::max_score(vec![
                vec![9, 5, 7, 3],
                vec![8, 9, 6, 1],
                vec![6, 7, 14, 3],
                vec![2, 5, 3, 1]
            ]),
            9
        );
        assert_eq!(Solution::max_score(vec![vec![4, 3, 2], vec![3, 2, 1]]), -1);
    }
}
