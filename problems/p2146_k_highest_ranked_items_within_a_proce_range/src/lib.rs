use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn highest_ranked_k_items(
        grid: Vec<Vec<i32>>,
        pricing: Vec<i32>,
        start: Vec<i32>,
        k: i32,
    ) -> Vec<Vec<i32>> {
        let mut values = Vec::with_capacity(grid.len() * grid[0].len());

        let mut queue = VecDeque::new();
        queue.push_back(((start[0] as usize, start[1] as usize), 0));

        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];

        while let Some(((r, c), d)) = queue.pop_front() {
            if seen[r][c] {
                continue;
            }
            seen[r][c] = true;
            if 1 < grid[r][c] && (pricing[0]..=pricing[1]).contains(&grid[r][c]) {
                values.push((d, grid[r][c], r, c));
            }

            for (dr, dc) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (next_r, next_c) = (r as i32 + dr, c as i32 + dc);
                if !(0..grid.len() as i32).contains(&next_r)
                    || !(0..grid[0].len() as i32).contains(&next_c)
                    || seen[next_r as usize][next_c as usize]
                    || grid[next_r as usize][next_c as usize] == 0
                {
                    continue;
                }
                queue.push_back(((next_r as usize, next_c as usize), d + 1));
            }
        }

        values.sort_unstable();
        values
            .into_iter()
            .take(k as usize)
            .map(|(_, _, r, c)| vec![r as i32, c as i32])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2146() {
        assert_eq!(
            Solution::highest_ranked_k_items(
                vec![vec![1, 2, 0, 1], vec![1, 3, 0, 1], vec![0, 2, 5, 1]],
                vec![2, 5],
                vec![0, 0],
                3
            ),
            vec![vec![0, 1], vec![1, 1], vec![2, 1]]
        );
        assert_eq!(
            Solution::highest_ranked_k_items(
                vec![vec![1, 2, 0, 1], vec![1, 3, 3, 1], vec![0, 2, 5, 1]],
                vec![2, 3],
                vec![2, 3],
                2
            ),
            vec![vec![2, 1], vec![1, 2]]
        );
        assert_eq!(
            Solution::highest_ranked_k_items(
                vec![vec![1, 1, 1], vec![0, 0, 1], vec![2, 3, 4]],
                vec![2, 3],
                vec![0, 0],
                3
            ),
            vec![vec![2, 1], vec![2, 0]]
        );
    }
}
