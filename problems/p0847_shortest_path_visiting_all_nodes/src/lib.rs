use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn dfs(
        prev: usize,
        seen: usize,
        n: usize,
        path: i32,
        min_path: &mut i32,
        dist: &Vec<Vec<i32>>,
        memo: &mut HashMap<(usize, usize), i32>,
    ) {
        if seen.count_ones() as usize == n {
            *min_path = std::cmp::min(*min_path, path);
            return;
        }

        if let Some(p) = memo.get(&(prev, seen)) {
            if *p <= path {
                return;
            }
        }
        memo.insert((prev, seen), path);

        if *min_path <= path {
            return;
        }

        for next in 0..n {
            if seen & 1 << next != 0 {
                continue;
            }
            Self::dfs(
                next,
                seen | 1 << next,
                n,
                path + dist[prev][next],
                min_path,
                dist,
                memo,
            );
        }
    }

    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let mut dist = vec![vec![std::i32::MAX; graph.len()]; graph.len()];
        for i in 0..graph.len() {
            dist[i][i] = 0;
            for j in &graph[i] {
                dist[i][*j as usize] = 1;
            }
        }

        for k in 0..graph.len() {
            for i in 0..graph.len() {
                for j in 0..graph.len() {
                    if dist[i][k] != std::i32::MAX && dist[k][j] != std::i32::MAX {
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
        }

        let mut min_path = std::i32::MAX;
        let mut memo = HashMap::new();

        for i in 0..graph.len() {
            Self::dfs(i, 1 << i, graph.len(), 0, &mut min_path, &dist, &mut memo);
        }

        min_path
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0847() {
        assert_eq!(
            Solution::shortest_path_length(vec![
                vec![2, 10],
                vec![2, 7],
                vec![0, 1, 3, 4, 5, 8],
                vec![2],
                vec![2],
                vec![2],
                vec![8],
                vec![9, 11, 8, 1],
                vec![7, 6, 2],
                vec![7],
                vec![11, 0],
                vec![7, 10]
            ]),
            15
        );
        assert_eq!(
            Solution::shortest_path_length(vec![vec![1, 2, 3], vec![0], vec![0], vec![0]]),
            4
        );
        assert_eq!(
            Solution::shortest_path_length(vec![
                vec![1],
                vec![0, 2, 4],
                vec![1, 3, 4],
                vec![2],
                vec![1, 2]
            ]),
            4
        );
    }
}
