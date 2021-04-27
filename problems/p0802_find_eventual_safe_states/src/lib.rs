use std::collections::HashSet;
pub struct Solution {}

impl Solution {
    pub fn dfs(
        graph: &Vec<Vec<i32>>,
        i: usize,
        is_safe: &mut Vec<bool>,
        seen: &mut Vec<bool>,
        path: &mut HashSet<usize>,
    ) {
        if path.contains(&i) || !is_safe[i] {
            path.iter().for_each(|j| is_safe[*j] = false);
            return;
        }

        path.insert(i);

        for next in &graph[i] {
            if seen[i] {
                continue;
            }
            Self::dfs(graph, *next as usize, is_safe, seen, path);
        }

        path.remove(&i);
        seen[i] = true;
    }

    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut is_safe = vec![true; graph.len()];
        let mut seen = vec![false; graph.len()];

        for i in 0..graph.len() {
            if seen[i] {
                continue;
            }
            Self::dfs(&graph, i, &mut is_safe, &mut seen, &mut HashSet::new());
        }

        (0..is_safe.len())
            .filter(|i| is_safe[*i])
            .map(|i| i as i32)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0802() {
        assert_eq!(
            Solution::eventual_safe_nodes(vec![
                vec![1, 2],
                vec![2, 3],
                vec![5],
                vec![0],
                vec![5],
                vec![],
                vec![]
            ]),
            vec![2, 4, 5, 6]
        );
        assert_eq!(
            Solution::eventual_safe_nodes(vec![
                vec![1, 2, 3, 4],
                vec![1, 2],
                vec![3, 4],
                vec![0, 4],
                vec![]
            ]),
            vec![4]
        );
    }
}
