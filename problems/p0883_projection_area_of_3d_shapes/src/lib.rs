pub struct Solution {}

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        grid.iter()
            .map(|v| v.iter().filter(|x| x > &&0).count() as i32)
            .sum::<i32>()
            + grid.iter().map(|v| v.iter().max().unwrap()).sum::<i32>()
            + (0..grid[0].len())
                .map(|i| (0..grid.len()).map(|j| grid[j][i]).max().unwrap())
                .sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0883() {
        assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
        assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
        assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
        assert_eq!(
            Solution::projection_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            14
        );
        assert_eq!(
            Solution::projection_area(vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]]),
            21
        );
    }
}
