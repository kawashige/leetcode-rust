pub struct Solution {}

impl Solution {
    pub fn dfs(
        heights: &Vec<Vec<i32>>,
        i: usize,
        j: usize,
        max: i32,
        seen: &mut Vec<Vec<bool>>,
    ) -> bool {
        if i == heights.len() - 1 && j == heights[0].len() - 1 {
            return true;
        }

        let moves = [(-1_i32, 0), (0, 1), (1, 0), (0, -1)];
        for k in 0..4 {
            let next_i = moves[k].0 + i as i32;
            let next_j = moves[k].1 + j as i32;
            if next_i < 0
                || heights.len() as i32 <= next_i
                || next_j < 0
                || heights[0].len() as i32 <= next_j
            {
                continue;
            }
            if !seen[next_i as usize][next_j as usize]
                && (heights[i][j] - heights[next_i as usize][next_j as usize]).abs() <= max
            {
                seen[next_i as usize][next_j as usize] = true;
                if Self::dfs(heights, next_i as usize, next_j as usize, max, seen) {
                    return true;
                }
            }
        }
        false
    }

    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut max = 1_000_000;
        let mut min = 0;
        while min < max {
            let mid = (min + max) / 2;
            let mut seen = vec![vec![false; heights[0].len()]; heights.len()];
            seen[0][0] = true;
            if Self::dfs(&heights, 0, 0, mid, &mut seen) {
                max = mid;
            } else {
                min = mid + 1;
            }
        }

        max as i32
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
