pub struct Solution {}

use std::collections::BTreeSet;
impl Solution {
    pub fn dfs(
        diffs: &Vec<Vec<[i32; 4]>>,
        i: usize,
        j: usize,
        max: i32,
        seen: &mut Vec<Vec<bool>>,
    ) -> bool {
        if i == diffs.len() - 1 && j == diffs[0].len() - 1 {
            return true;
        }

        let moves = [(-1_i32, 0), (0, 1), (1, 0), (0, -1)];
        for k in 0..4 {
            if diffs[i][j][k] != -1 && diffs[i][j][k] <= max {
                let next_i = (moves[k].0 + i as i32) as usize;
                let next_j = (moves[k].1 + j as i32) as usize;
                if !seen[next_i][next_j] {
                    seen[next_i][next_j] = true;
                    if Self::dfs(diffs, next_i, next_j, max, seen) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut diffs = BTreeSet::new();
        let mut adjacent_diffs = vec![vec![[-1; 4]; heights[0].len()]; heights.len()];
        for i in 0..heights.len() {
            for j in 0..heights[0].len() {
                if i < heights.len() - 1 {
                    let d = (heights[i][j] - heights[i + 1][j]).abs();
                    adjacent_diffs[i][j][2] = d;
                    adjacent_diffs[i + 1][j][0] = d;
                    diffs.insert(d);
                }
                if j < heights[0].len() - 1 {
                    let d = (heights[i][j] - heights[i][j + 1]).abs();
                    adjacent_diffs[i][j][1] = d;
                    adjacent_diffs[i][j + 1][3] = d;
                    diffs.insert(d);
                }
            }
        }

        if diffs.is_empty() {
            return 0;
        }

        let mut max = diffs.len() - 1;
        let mut min = 0;
        let diffs: Vec<i32> = diffs.into_iter().collect();
        while min < max {
            let mid = (min + max) / 2;
            let mut seen = vec![vec![false; heights[0].len()]; heights.len()];
            seen[0][0] = true;
            if Self::dfs(&adjacent_diffs, 0, 0, diffs[mid], &mut seen) {
                max = mid;
            } else {
                min = mid + 1;
            }
        }

        diffs[max]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day26() {
        assert_eq!(Solution::minimum_effort_path(vec![vec![3, 5]]), 2);
        assert_eq!(Solution::minimum_effort_path(vec![vec![3], vec![5]]), 2);
        assert_eq!(Solution::minimum_effort_path(vec![vec![3]]), 0);
        assert_eq!(
            Solution::minimum_effort_path(vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]]),
            1
        );
        assert_eq!(
            Solution::minimum_effort_path(vec![
                vec![1, 2, 1, 1, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 1, 1, 2, 1]
            ]),
            0
        );
    }
}
