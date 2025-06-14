pub struct Solution {}

impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let mut rows = vec![0; grid.len()];
        let mut columns = vec![0; grid[0].len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    rows[i] += 1;
                    columns[j] += 1;
                }
            }
        }

        let mut result = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 || rows[i] == 1 || columns[j] == 1 {
                    continue;
                }
                result += (rows[i] as i64 - 1) * (columns[j] as i64 - 1);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3128() {
        assert_eq!(
            Solution::number_of_right_triangles(vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 0]]),
            2
        );
        assert_eq!(
            Solution::number_of_right_triangles(vec![
                vec![1, 0, 0, 0],
                vec![0, 1, 0, 1],
                vec![1, 0, 0, 0]
            ]),
            0
        );
        assert_eq!(
            Solution::number_of_right_triangles(vec![vec![1, 0, 1], vec![1, 0, 0], vec![1, 0, 0]]),
            2
        );
    }
}
