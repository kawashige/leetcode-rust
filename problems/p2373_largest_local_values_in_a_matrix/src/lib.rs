pub struct Solution {}

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; grid[0].len() - 2]; grid.len() - 2];

        for i in 0..result.len() {
            for j in 0..result[0].len() {
                for l in 0..3 {
                    for m in 0..3 {
                        result[i][j] = result[i][j].max(grid[i + l][j + m]);
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2373() {
        assert_eq!(
            Solution::largest_local(vec![
                vec![9, 9, 8, 1],
                vec![5, 6, 2, 6],
                vec![8, 2, 6, 4],
                vec![6, 2, 2, 2]
            ]),
            vec![vec![9, 9], vec![8, 6]]
        );
        assert_eq!(
            Solution::largest_local(vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 2, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1]
            ]),
            vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]]
        );
    }
}
