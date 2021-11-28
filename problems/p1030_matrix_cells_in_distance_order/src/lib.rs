use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn all_cells_dist_order(
        rows: i32,
        cols: i32,
        r_center: i32,
        c_center: i32,
    ) -> Vec<Vec<i32>> {
        let mut r = Vec::with_capacity((rows * cols) as usize);

        let mut queue = VecDeque::new();
        queue.push_back((r_center, c_center));
        let mut seen = vec![vec![false; cols as usize]; rows as usize];

        while let Some((i, j)) = queue.pop_front() {
            if seen[i as usize][j as usize] {
                continue;
            }
            seen[i as usize][j as usize] = true;
            r.push(vec![i, j]);

            for (x, y) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (r, c) = (i + x, y + j);
                if r < 0 || rows <= r || c < 0 || cols <= c || seen[r as usize][c as usize] {
                    continue;
                }
                queue.push_back((r, c));
            }
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1030() {
        assert_eq!(
            Solution::all_cells_dist_order(1, 2, 0, 0),
            vec![vec![0, 0], vec![0, 1]]
        );
        assert_eq!(
            Solution::all_cells_dist_order(2, 2, 0, 1),
            vec![vec![0, 1], vec![0, 0], vec![1, 1], vec![1, 0]]
        );
        assert_eq!(
            Solution::all_cells_dist_order(2, 3, 1, 2),
            vec![
                vec![1, 2],
                vec![0, 2],
                vec![1, 1],
                vec![0, 1],
                vec![1, 0],
                vec![0, 0]
            ]
        );
    }
}
