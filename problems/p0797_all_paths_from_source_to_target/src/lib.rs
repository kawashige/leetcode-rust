pub struct Solution {}

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = graph.len() as i32;
        let mut result = Vec::new();

        let mut stack = vec![vec![0_i32]];

        while let Some(path) = stack.pop() {
            if path.last() == Some(&(n - 1)) {
                result.push(path);
                continue;
            }

            for next in &graph[*path.last().unwrap() as usize] {
                stack.push(path.iter().cloned().chain(std::iter::once(*next)).collect());
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0797() {
        assert_eq!(
            Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]),
            vec![vec![0, 2, 3], vec![0, 1, 3]]
        );
        assert_eq!(
            Solution::all_paths_source_target(vec![
                vec![4, 3, 1],
                vec![3, 2, 4],
                vec![3],
                vec![4],
                vec![]
            ]),
            vec![
                vec![0, 1, 4],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 3, 4],
                vec![0, 4],
            ]
        );
        assert_eq!(
            Solution::all_paths_source_target(vec![vec![1], vec![]]),
            vec![vec![0, 1]]
        );
        assert_eq!(
            Solution::all_paths_source_target(vec![vec![1, 2, 3], vec![2], vec![3], vec![]]),
            vec![vec![0, 3], vec![0, 2, 3], vec![0, 1, 2, 3]]
        );
        assert_eq!(
            Solution::all_paths_source_target(vec![vec![1, 3], vec![2], vec![3], vec![]]),
            vec![vec![0, 3], vec![0, 1, 2, 3]]
        );
    }
}
