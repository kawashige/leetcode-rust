pub struct Solution {}

impl Solution {
    pub fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut skip = false;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if !skip {
                    result.push(grid[i][if i % 2 == 0 { j } else { grid[0].len() - 1 - j }]);
                }
                skip = !skip;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3417() {
        assert_eq!(
            Solution::zigzag_traversal(vec![vec![1, 2], vec![3, 4]]),
            vec![1, 4]
        );
        assert_eq!(
            Solution::zigzag_traversal(vec![vec![2, 1], vec![2, 1], vec![2, 1]]),
            vec![2, 1, 2]
        );
        assert_eq!(
            Solution::zigzag_traversal(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 3, 5, 7, 9]
        );
    }
}
