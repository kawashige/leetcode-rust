use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut group = vec![0; graph.len()];

        for i in 0..graph.len() {
            if group[i] != 0 {
                continue;
            }
            group[i] = 1;
            let mut queue = graph[i]
                .iter()
                .map(|j| (*j as usize, 2))
                .collect::<VecDeque<(usize, usize)>>();
            while let Some(node) = queue.pop_front() {
                if group[node.0] == 0 {
                    group[node.0] = node.1;
                    for n in &graph[node.0] {
                        queue.push_front((*n as usize, if node.1 == 1 { 2 } else { 1 }));
                    }
                } else if group[node.0] != node.1 {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day14() {
        assert!(!Solution::is_bipartite(vec![
            vec![],
            vec![2, 4, 6],
            vec![1, 4, 8, 9],
            vec![7, 8],
            vec![1, 2, 8, 9],
            vec![6, 9],
            vec![1, 5, 7, 8, 9],
            vec![3, 6, 9],
            vec![2, 3, 4, 6, 9],
            vec![2, 4, 5, 6, 7, 8]
        ]));
        assert!(Solution::is_bipartite(vec![
            vec![1, 3],
            vec![0, 2],
            vec![1, 3],
            vec![0, 2]
        ]));
        assert!(!Solution::is_bipartite(vec![
            vec![1, 2, 3],
            vec![0, 2],
            vec![0, 1, 3],
            vec![0, 2]
        ]));
    }
}
