pub struct Solution {}

impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let mut land = 0;
        let mut stack = Vec::new();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    land += 1;
                    if i == 0 || i == grid.len() - 1 || j == 0 || j == grid[0].len() - 1 {
                        stack.push((i, j));
                    }
                }
            }
        }

        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
        while let Some((i, j)) = stack.pop() {
            if seen[i][j] {
                continue;
            }
            seen[i][j] = true;
            land -= 1;

            for (x, y) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let r = i as i32 + x;
                let c = j as i32 + y;
                if r < 0
                    || grid.len() as i32 <= r
                    || c < 0
                    || grid[0].len() as i32 <= c
                    || grid[r as usize][c as usize] == 0
                    || seen[r as usize][c as usize]
                {
                    continue;
                }
                stack.push((r as usize, c as usize));
            }
        }

        land
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1020() {
        assert_eq!(
            Solution::num_enclaves(vec![
                vec![0, 0, 0, 0],
                vec![1, 0, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0]
            ]),
            3
        );
        assert_eq!(
            Solution::num_enclaves(vec![
                vec![0, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 0]
            ]),
            0
        );
    }
}
