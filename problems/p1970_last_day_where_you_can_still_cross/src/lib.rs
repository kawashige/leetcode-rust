pub struct Solution {}

impl Solution {
    pub fn is_ok(day: usize, grid: &Vec<Vec<usize>>) -> bool {
        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
        let mut stack = (0..grid[0].len())
            .filter_map(|i| {
                if grid[0][i] <= day {
                    None
                } else {
                    Some((0, i))
                }
            })
            .collect::<Vec<_>>();

        while let Some((i, j)) = stack.pop() {
            if seen[i][j] {
                continue;
            }
            if i == grid.len() - 1 {
                return true;
            }
            seen[i][j] = true;
            for (dr, dc) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (r, c) = (i as i32 + dr, j as i32 + dc);
                if !(0..grid.len() as i32).contains(&r)
                    || !(0..grid[0].len() as i32).contains(&c)
                    || seen[r as usize][c as usize]
                    || grid[r as usize][c as usize] <= day
                {
                    continue;
                }
                stack.push((r as usize, c as usize));
            }
        }

        false
    }

    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let mut ok = 0;
        let mut ng = cells.len();

        let mut grid = vec![vec![cells.len() + 1; col as usize]; row as usize];
        for i in 0..cells.len() {
            grid[cells[i][0] as usize - 1][cells[i][1] as usize - 1] = i + 1;
        }

        println!("{:?}", grid);

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, &grid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1970() {
        assert_eq!(
            Solution::latest_day_to_cross(
                2,
                2,
                vec![vec![1, 1], vec![2, 1], vec![1, 2], vec![2, 2]]
            ),
            2
        );
        assert_eq!(
            Solution::latest_day_to_cross(
                2,
                2,
                vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]]
            ),
            1
        );
        assert_eq!(
            Solution::latest_day_to_cross(
                3,
                3,
                vec![
                    vec![1, 2],
                    vec![2, 1],
                    vec![3, 3],
                    vec![2, 2],
                    vec![1, 1],
                    vec![1, 3],
                    vec![2, 3],
                    vec![3, 2],
                    vec![3, 1]
                ]
            ),
            3
        );
    }
}
