pub struct Solution {}

impl Solution {
    pub fn dfs(
        prev: usize,
        seen: usize,
        n: usize,
        path: i32,
        min_path: &mut i32,
        dist: &Vec<Vec<i32>>,
    ) {
        if seen.count_ones() as usize == n {
            *min_path = std::cmp::min(*min_path, path);
            return;
        }

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

        for i in 0..graph.len() {
            Self::dfs(i, 1 << i, graph.len(), 0, &mut min_path, &dist);
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
