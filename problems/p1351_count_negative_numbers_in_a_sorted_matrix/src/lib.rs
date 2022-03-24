pub struct Solution {}

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let mut i = grid.len();

        for j in 0..grid[0].len() {
            while 1 <= i && grid[i - 1][j] < 0 {
                i -= 1;
            }
            count += grid.len() - i;
        }

        count as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1351() {
        assert_eq!(
            Solution::count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3]
            ]),
            8
        );
        assert_eq!(Solution::count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
    }
}
