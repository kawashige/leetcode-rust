pub struct Solution {}

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;

        fn recurse(start: &[i32; 2], end: &[i32; 2], remains: HashSet<[i32; 2]>) -> i32 {
            let moves = [[1, 0], [0, 1], [-1, 0], [0, -1]];
            let mut result = 0;
            for next in moves.iter().map(|v| [v[0] + start[0], v[1] + start[1]]) {
                if remains.contains(&next) {
                    let mut new_remains = remains.clone();
                    new_remains.remove(&next);
                    result += recurse(&next, end, new_remains);
                } else if &next == end && remains.len() == 0 {
                    result += 1;
                }
            }

            result
        }

        let mut remains = HashSet::new();
        let mut start = [0, 0];
        let mut end = [0, 0];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                match grid[i][j] {
                    0 => {
                        remains.insert([i as i32, j as i32]);
                    }
                    1 => {
                        start = [i as i32, j as i32];
                    }
                    2 => {
                        end = [i as i32, j as i32];
                    }
                    _ => {}
                }
            }
        }

        recurse(&start, &end, remains)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day20() {
        assert_eq!(
            2,
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]])
        );
        assert_eq!(
            4,
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]])
        );
        assert_eq!(0, Solution::unique_paths_iii(vec![vec![0, 1], vec![2, 0]]));
    }
}
