use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let grid = grid
            .into_iter()
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut keys = 0;
        let mut queue = VecDeque::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j].is_ascii_lowercase() {
                    keys += 1;
                }
                if grid[i][j] == '@' {
                    queue.push_back((i, j, 0_usize, 0));
                }
            }
        }

        let mut states =
            vec![vec![vec![std::i32::MAX; 2_usize.pow(keys as u32)]; grid[0].len()]; grid.len()];

        while let Some((i, j, state, count)) = queue.pop_front() {
            let mut state = state;
            if grid[i][j].is_ascii_lowercase() {
                state |= 1 << (grid[i][j] as u8 - b'a') as usize;
            }
            if state.count_ones() as usize == keys {
                return count;
            }
            if states[i][j][state] <= count {
                continue;
            }
            states[i][j][state] = count;

            for (dr, dc) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (r, c) = (i as i32 + dr, j as i32 + dc);
                if !(0..grid.len() as i32).contains(&r)
                    || !(0..grid[0].len() as i32).contains(&c)
                    || grid[r as usize][c as usize] == '#'
                    || (grid[r as usize][c as usize].is_ascii_uppercase()
                        && state & 1 << (grid[r as usize][c as usize] as u8 - b'A') as usize == 0)
                {
                    continue;
                }
                queue.push_back((r as usize, c as usize, state, count + 1));
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0864() {
        assert_eq!(
            Solution::shortest_path_all_keys(vec!["@abcdeABCDEFf".to_string()]),
            -1
        );
        assert_eq!(
            Solution::shortest_path_all_keys(vec![
                "@.a..".to_string(),
                "###.#".to_string(),
                "b.A.B".to_string()
            ]),
            8
        );
        assert_eq!(
            Solution::shortest_path_all_keys(vec![
                "@..aA".to_string(),
                "..B#.".to_string(),
                "....b".to_string()
            ]),
            6
        );
        assert_eq!(
            Solution::shortest_path_all_keys(vec!["@Aa".to_string()]),
            -1
        );
    }
}
