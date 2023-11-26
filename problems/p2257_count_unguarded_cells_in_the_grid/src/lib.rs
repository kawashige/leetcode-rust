pub struct Solution {}

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut grid = vec![vec![0; n as usize]; m as usize];

        for guard in guards {
            grid[guard[0] as usize][guard[1] as usize] = 1;
        }
        for wall in walls {
            grid[wall[0] as usize][wall[1] as usize] = 2;
        }

        for i in 0..grid.len() {
            let mut guarded = false;
            for j in 0..grid[0].len() {
                match grid[i][j] {
                    0 if guarded => grid[i][j] = 3,
                    1 => guarded = true,
                    2 => guarded = false,
                    _ => {}
                }
            }
            guarded = false;
            for j in (0..grid[0].len()).rev() {
                match grid[i][j] {
                    0 if guarded => grid[i][j] = 3,
                    1 => guarded = true,
                    2 => guarded = false,
                    _ => {}
                }
            }
        }

        let mut count = 0;

        for j in 0..grid[0].len() {
            let mut guarded = false;
            for i in 0..grid.len() {
                match grid[i][j] {
                    0 if guarded => grid[i][j] = 3,
                    1 => guarded = true,
                    2 => guarded = false,
                    _ => {}
                }
            }
            guarded = false;
            for i in (0..grid.len()).rev() {
                match grid[i][j] {
                    0 if guarded => grid[i][j] = 3,
                    1 => guarded = true,
                    2 => guarded = false,
                    0 if grid[i][j] == 0 => count += 1,
                    _ => {}
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2257() {
        assert_eq!(
            Solution::count_unguarded(
                4,
                6,
                vec![vec![0, 0], vec![1, 1], vec![2, 3]],
                vec![vec![0, 1], vec![2, 2], vec![1, 4]]
            ),
            7
        );
        assert_eq!(
            Solution::count_unguarded(
                3,
                3,
                vec![vec![1, 1]],
                vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]]
            ),
            4
        );
    }
}
