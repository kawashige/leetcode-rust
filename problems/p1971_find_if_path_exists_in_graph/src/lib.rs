use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut list = vec![vec![]; n as usize];
        for edge in edges {
            list[edge[0] as usize].push(edge[1] as usize);
            list[edge[1] as usize].push(edge[0] as usize);
        }

        let mut queue = VecDeque::new();
        let mut seen = vec![false; n as usize];
        queue.push_back(source as usize);
        while let Some(node) = queue.pop_front() {
            if node == destination as usize {
                return true;
            }
            if seen[node] {
                continue;
            }
            seen[node] = true;

            for next in &list[node] {
                if !seen[*next] {
                    queue.push_back(*next);
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1971() {
        assert!(Solution::valid_path(
            3,
            vec![vec![0, 1], vec![1, 2], vec![2, 0]],
            0,
            2
        ));
        assert!(!Solution::valid_path(
            6,
            vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
            0,
            5
        ));
    }
}
