pub struct Solution {}

impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const M: i32 = 12345;
        let mut prefix = vec![vec![0; grid[0].len()]; grid.len()];
        let mut suffix = vec![vec![0; grid[0].len()]; grid.len()];
        let mut p = 1;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                prefix[i][j] = p;
                p = (p * (grid[i][j] % M)) % M;
            }
        }
        let mut p = 1;
        for i in (0..grid.len()).rev() {
            for j in (0..grid[0].len()).rev() {
                suffix[i][j] = p;
                p = (p * (grid[i][j] % M)) % M;
            }
        }
        let mut result = vec![vec![1; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                result[i][j] = prefix[i][j] * suffix[i][j] % M
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2906() {
        assert_eq!(
            Solution::construct_product_matrix(vec![vec![1, 2], vec![3, 4]]),
            vec![vec![24, 12], vec![8, 6]]
        );
        assert_eq!(
            Solution::construct_product_matrix(vec![vec![12345], vec![2], vec![1]]),
            vec![vec![2], vec![0], vec![0]]
        );
    }
}
