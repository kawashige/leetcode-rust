pub struct Solution {}

impl Solution {
    pub fn rotate(grid: Vec<Vec<i32>>, i: usize) -> Vec<Vec<i32>> {
        let mut new_grid = grid.clone();
        let (mut r, mut c) = (i, i);

        for (dr, dc) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
            loop {
                let new_r = r as i32 + dr;
                let new_c = c as i32 + dc;

                if !(i as i32..=(grid.len() - 1 - i) as i32).contains(&new_r)
                    || !(i as i32..=(grid[0].len() - 1 - i) as i32).contains(&new_c)
                {
                    break;
                }

                new_grid[new_r as usize][new_c as usize] = grid[r][c];
                r = new_r as usize;
                c = new_c as usize;
            }
        }

        new_grid
    }

    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut grid = grid;

        for i in 0..(grid.len() / 2).min(grid[0].len() / 2) {
            let k = (k as usize) % ((grid.len() - i * 2) * 2 + (grid[0].len() - i * 2) * 2 - 4);
            for _ in 0..k {
                grid = Self::rotate(grid, i);
            }
        }

        grid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1914() {
        assert_eq!(
            Solution::rotate_grid(vec![vec![40, 10], vec![30, 20]], 1),
            vec![vec![10, 20], vec![40, 30]]
        );
        assert_eq!(
            Solution::rotate_grid(
                vec![
                    vec![1, 2, 3, 4],
                    vec![5, 6, 7, 8],
                    vec![9, 10, 11, 12],
                    vec![13, 14, 15, 16]
                ],
                2
            ),
            vec![
                vec![3, 4, 8, 12],
                vec![2, 11, 10, 16],
                vec![1, 7, 6, 15],
                vec![5, 9, 13, 14]
            ]
        );
    }
}
