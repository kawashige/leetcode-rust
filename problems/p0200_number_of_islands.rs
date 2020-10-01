pub struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        fn flip(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
            grid[i][j] = '0';
            if 0 < i && grid[i - 1][j] == '1' {
                flip(grid, i - 1, j);
            }
            if i < grid.len() - 1 && grid[i + 1][j] == '1' {
                flip(grid, i + 1, j);
            }
            if 0 < j && grid[i][j - 1] == '1' {
                flip(grid, i, j - 1);
            }
            if j < grid[0].len() - 1 && grid[i][j + 1] == '1' {
                flip(grid, i, j + 1);
            }
        }

        let mut grid = grid;

        let mut result = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    result += 1;
                    flip(&mut grid, i, j);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0200() {
        assert_eq!(
            1,
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ])
        );

        assert_eq!(
            3,
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ])
        );
    }
}
