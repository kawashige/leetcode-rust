pub struct Solution {}

impl Solution {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut values = vec![0; grid[0].len()];

        for i in 0..grid.len() {
            let mut row = grid[i].clone();
            row.sort_unstable();
            for j in 0..row.len() {
                values[j] = values[j].max(row[j]);
            }
        }

        values.into_iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2500() {
        assert_eq!(
            Solution::delete_greatest_value(vec![vec![1, 2, 4], vec![3, 3, 1]]),
            8
        );
        assert_eq!(Solution::delete_greatest_value(vec![vec![10]]), 10);
    }
}
