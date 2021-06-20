pub struct Solution {}

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ok = (n * n - 1) as i32;
        let mut ng = 0;

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;

            let mut stack = Vec::new();
            if grid[0][0] <= mid {
                stack.push((0, 0));
            }
            let mut seen = vec![vec![false; n]; n];
            let mut found = false;

            while let Some((i, j)) = stack.pop() {
                if seen[i][j] {
                    continue;
                }

                if (i, j) == (n - 1, n - 1) {
                    found = true;
                    break;
                }

                seen[i][j] = true;

                if i > 0 && !seen[i - 1][j] && grid[i - 1][j] <= mid {
                    stack.push((i - 1, j));
                }
                if j > 0 && !seen[i][j - 1] && grid[i][j - 1] <= mid {
                    stack.push((i, j - 1));
                }
                if i < n - 1 && !seen[i + 1][j] && grid[i + 1][j] <= mid {
                    stack.push((i + 1, j));
                }
                if j < n - 1 && !seen[i][j + 1] && grid[i][j + 1] <= mid {
                    stack.push((i, j + 1));
                }
            }

            if found {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day21() {
        assert_eq!(Solution::swim_in_water(vec![vec![3, 2], vec![0, 1]]), 3);
        assert_eq!(Solution::swim_in_water(vec![vec![0, 2], vec![1, 3]]), 3);
        assert_eq!(
            Solution::swim_in_water(vec![
                vec![0, 1, 2, 3, 4],
                vec![24, 23, 22, 21, 5],
                vec![12, 13, 14, 15, 16],
                vec![11, 17, 18, 19, 20],
                vec![10, 9, 8, 7, 6]
            ]),
            16
        );
    }
}
