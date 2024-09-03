pub struct Solution {}

impl Solution {
    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; grid[0].len()]; grid.len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let mut left_above = vec![false; 51];
                for k in 1..=i.min(j) {
                    left_above[grid[i - k][j - k] as usize] = true;
                }
                let mut right_below = vec![false; 51];
                for k in 1..(grid.len() - i).min(grid[0].len() - j) {
                    right_below[grid[i + k][j + k] as usize] = true;
                }
                result[i][j] = (left_above.into_iter().filter(|v| *v).count() as i32
                    - right_below.into_iter().filter(|v| *v).count() as i32)
                    .abs();
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2711() {
        assert_eq!(
            Solution::difference_of_distinct_values(vec![
                vec![1, 2, 3],
                vec![3, 1, 5],
                vec![3, 2, 1]
            ]),
            vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]]
        );
        assert_eq!(
            Solution::difference_of_distinct_values(vec![vec![1]]),
            vec![vec![0]]
        );
    }
}
