pub struct Solution {}

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    if i == 0 || grid[i - 1][j] == 0 {
                        perimeter += 1;
                    }
                    if j == 0 || grid[i][j - 1] == 0 {
                        perimeter += 1;
                    }
                    if i == grid.len() - 1 || grid[i + 1][j] == 0 {
                        perimeter += 1;
                    }
                    if j == grid[0].len() - 1 || grid[i][j + 1] == 0 {
                        perimeter += 1;
                    }
                }
            }
        }

        perimeter
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_p0463() {
        assert_eq!(
            16,
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ])
        );
        assert_eq!(4, Solution::island_perimeter(vec![vec![1]]));
        assert_eq!(4, Solution::island_perimeter(vec![vec![1, 0]]));
    }
}
