use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn modified_graph_edges(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        let mut weights = vec![vec![]; n as usize];
        for i in 0..edges.len() {
            weights[edges[i][0] as usize].push((edges[i][1] as usize, i));
            weights[edges[i][1] as usize].push((edges[i][0] as usize, i));
        }

        let mut heap = BinaryHeap::new();
        let mut dist = vec![std::i32::MAX; n as usize];
        heap.push((Reverse(0), source as usize));
        while let Some((Reverse(d), i)) = heap.pop() {
            if dist[i] <= d {
                continue;
            }
            dist[i] = d;
            for (next_i, edge_i) in &weights[i] {
                if edges[*edge_i][2] == -1 || dist[*next_i] <= d + edges[*edge_i][2] {
                    continue;
                }
                heap.push((Reverse(d + edges[*edge_i][2]), *next_i));
            }
        }

        if dist[destination as usize] < target {
            return vec![];
        }
        if dist[destination as usize] == target {
            return edges
                .into_iter()
                .map(|e| {
                    vec![
                        e[0],
                        e[1],
                        if e[2] == -1 { 2 * 1_000_000_000 } else { e[2] },
                    ]
                })
                .collect();
        }

        let mut heap = BinaryHeap::new();
        let mut dist = vec![std::i32::MAX; n as usize];
        let mut prev = vec![(n as usize, edges.len()); n as usize];
        heap.push((Reverse(0), source as usize, n as usize, edges.len()));
        while let Some((Reverse(d), i, prev_i, prev_edge_i)) = heap.pop() {
            if dist[i] <= d {
                continue;
            }
            dist[i] = d;
            prev[i] = (prev_i, prev_edge_i);
            for (next_i, edge_i) in &weights[i] {
                let new_d = d + if edges[*edge_i][2] == -1 {
                    1
                } else {
                    edges[*edge_i][2]
                };
                if dist[*next_i] <= new_d {
                    continue;
                }
                heap.push((Reverse(new_d), *next_i, i, *edge_i));
            }
        }
        if target < dist[destination as usize] {
            return vec![];
        }
        let mut targets = vec![];
        let mut i = destination as usize;
        while i != source as usize {
            if edges[prev[i].1][2] == -1 {
                targets.push(prev[i].1);
            }
            i = prev[i].0;
        }

        let mut edges = edges;
        for i in 0..edges.len() {
            if edges[i][2] == -1 {
                edges[i][2] = 2 * 1_000_000_000;
            }
        }
        for k in &targets {
            edges[*k][2] = 1;
        }

        for k in targets {
            let mut heap = BinaryHeap::new();
            let mut dist1 = vec![target as usize + 1; n as usize];
            heap.push((Reverse(0), edges[k][0] as usize));
            while let Some((Reverse(d), i)) = heap.pop() {
                if dist1[i] <= d {
                    continue;
                }
                dist1[i] = d;
                for (next_i, edge_i) in &weights[i] {
                    let new_d = d + if edges[*edge_i][2] == -1 {
                        1
                    } else {
                        edges[*edge_i][2] as usize
                    };
                    if edge_i == &k || dist1[*next_i] <= new_d {
                        continue;
                    }
                    heap.push((Reverse(new_d), *next_i));
                }
            }

            let mut heap = BinaryHeap::new();
            let mut dist2 = vec![target as usize + 1; n as usize];
            heap.push((Reverse(0), edges[k][1] as usize));
            while let Some((Reverse(d), i)) = heap.pop() {
                if dist2[i] <= d {
                    continue;
                }
                dist2[i] = d;
                for (next_i, edge_i) in &weights[i] {
                    let new_d = d + if edges[*edge_i][2] == -1 {
                        1
                    } else {
                        edges[*edge_i][2] as usize
                    };
                    if edge_i == &k || dist2[*next_i] <= new_d {
                        continue;
                    }
                    heap.push((Reverse(new_d), *next_i));
                }
            }

            if dist1[source as usize] + dist2[destination as usize] < target as usize {
                edges[k][2] =
                    target - dist1[source as usize] as i32 - dist2[destination as usize] as i32;
            }
            if dist2[source as usize] + dist1[destination as usize] < target as usize {
                edges[k][2] = edges[k][2].max(
                    target - dist2[source as usize] as i32 - dist1[destination as usize] as i32,
                )
            }
            if edges[k][2] == -1 {
                edges[k][2] = 2 * 1_000_000_000;
            }
        }

        edges
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2699() {
        assert_eq!(
            Solution::modified_graph_edges(
                4,
                vec![vec![0, 1, 1], vec![1, 2, 2], vec![2, 3, 3]],
                0,
                2,
                1
            ),
            vec![] as Vec<Vec<i32>>
        );
        assert_eq!(
            Solution::modified_graph_edges(
                5,
                vec![
                    vec![4, 1, -1],
                    vec![2, 0, -1],
                    vec![0, 3, -1],
                    vec![4, 3, -1]
                ],
                0,
                1,
                5
            ),
            vec![
                vec![4, 1, 3],
                vec![2, 0, 2000000000],
                vec![0, 3, 1],
                vec![4, 3, 1]
            ]
        );
        assert_eq!(
            Solution::modified_graph_edges(3, vec![vec![0, 1, -1], vec![0, 2, 5]], 0, 2, 6),
            vec![] as Vec<Vec<i32>>
        );
        assert_eq!(
            Solution::modified_graph_edges(
                4,
                vec![vec![1, 0, 4], vec![1, 2, 3], vec![2, 3, 5], vec![0, 3, -1]],
                0,
                2,
                6
            ),
            vec![vec![1, 0, 4], vec![1, 2, 3], vec![2, 3, 5], vec![0, 3, 1]]
        );
    }
}
