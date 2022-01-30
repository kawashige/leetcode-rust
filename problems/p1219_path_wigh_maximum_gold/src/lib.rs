pub struct Solution {}

impl Solution {
    pub fn dfs(
        i: usize,
        j: usize,
        grid: &Vec<Vec<i32>>,
        seen: &mut Vec<Vec<bool>>,
        gold: i32,
        max_gold: &mut i32,
    ) {
        let mut is_end = true;
        for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (x, y) = (i as i32 + dx, j as i32 + dy);
            if x < 0
                || grid.len() as i32 <= x
                || y < 0
                || grid[0].len() as i32 <= y
                || seen[x as usize][y as usize]
                || grid[x as usize][y as usize] == 0
            {
                continue;
            }

            is_end = false;
            seen[x as usize][y as usize] = true;
            Self::dfs(
                x as usize,
                y as usize,
                grid,
                seen,
                gold + grid[i][j],
                max_gold,
            );
            seen[x as usize][y as usize] = false;
        }

        if is_end {
            *max_gold = std::cmp::max(*max_gold, gold + grid[i][j]);
        }
    }

    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    continue;
                }
                let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
                seen[i][j] = true;
                let mut max_gold = 0;
                Self::dfs(i, j, &grid, &mut seen, 0, &mut max_gold);
                result = result.max(max_gold);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1219() {
        assert_eq!(
            Solution::get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]]),
            24
        );
        assert_eq!(
            Solution::get_maximum_gold(vec![
                vec![1, 0, 7],
                vec![2, 0, 6],
                vec![3, 4, 5],
                vec![0, 3, 0],
                vec![9, 0, 20]
            ]),
            28
        );
    }
}
