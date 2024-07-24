pub struct Solution {}

impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        (0..grid[0].len())
            .map(|i| {
                (0..grid.len())
                    .map(|j| grid[j][i].to_string().len() as i32)
                    .max()
                    .unwrap()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2639() {
        assert_eq!(
            Solution::find_column_width(vec![vec![1], vec![22], vec![333]]),
            vec![3]
        );
        assert_eq!(
            Solution::find_column_width(vec![vec![-15, 1, 3], vec![15, 7, 12], vec![5, 6, -2]]),
            vec![3, 1, 2]
        );
    }
}
