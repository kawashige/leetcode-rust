pub struct Solution {}

impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        (0..grid.len())
            .find(|i| (0..grid[0].len()).all(|j| i == &j || grid[*i][j] == 1))
            .unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2923() {
        assert_eq!(Solution::find_champion(vec![vec![0, 1], vec![0, 0]]), 0);
        assert_eq!(
            Solution::find_champion(vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 0, 0]]),
            1
        );
    }
}
