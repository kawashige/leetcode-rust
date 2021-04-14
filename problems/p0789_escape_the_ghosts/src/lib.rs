pub struct Solution {}

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        ghosts
            .into_iter()
            .map(|g| (g[0] - target[0]).abs() + (g[1] - target[1]).abs())
            .min()
            .unwrap()
            > target[0].abs() + target[1].abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0789() {
        assert!(!Solution::escape_ghosts(vec![vec![2, 2]], vec![1, 1]));
        assert!(Solution::escape_ghosts(
            vec![vec![1, 0], vec![0, 3]],
            vec![0, 1]
        ));
        assert!(!Solution::escape_ghosts(vec![vec![1, 0]], vec![2, 0]));
        assert!(!Solution::escape_ghosts(vec![vec![2, 0]], vec![1, 0]));
        assert!(!Solution::escape_ghosts(
            vec![
                vec![5, 0],
                vec![-10, -2],
                vec![0, -5],
                vec![-2, -2],
                vec![-7, 1]
            ],
            vec![7, 7]
        ));
        assert!(Solution::escape_ghosts(
            vec![
                vec![-1, 0],
                vec![0, 1],
                vec![-1, 0],
                vec![0, 1],
                vec![-1, 0]
            ],
            vec![0, 0]
        ));
    }
}
