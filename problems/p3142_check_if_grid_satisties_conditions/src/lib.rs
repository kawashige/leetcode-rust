pub struct Solution {}

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if (i < grid.len() - 1 && grid[i][j] != grid[i + 1][j])
                    || (j < grid[0].len() - 1 && grid[i][j] == grid[i][j + 1])
                {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3142() {
        assert!(Solution::satisfies_conditions(vec![
            vec![1, 0, 2],
            vec![1, 0, 2]
        ]));
        assert!(!Solution::satisfies_conditions(vec![
            vec![1, 1, 1],
            vec![0, 0, 0]
        ]));
        assert!(!Solution::satisfies_conditions(vec![
            vec![1],
            vec![2],
            vec![3]
        ]));
    }
}
