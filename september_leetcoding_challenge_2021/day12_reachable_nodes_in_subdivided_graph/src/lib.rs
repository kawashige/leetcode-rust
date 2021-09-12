use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut count = vec![vec![0; n]; n];

        let mut list = vec![vec![]; n];
        for edge in &edges {
            list[edge[0] as usize].push((edge[1] as usize, edge[2] + 1));
            list[edge[1] as usize].push((edge[0] as usize, edge[2] + 1));
        }

        let mut dist = vec![std::i32::MAX; n];
        dist[0] = 0;
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), 0));

        while let Some((Reverse(d), i)) = heap.pop() {
            if dist[i] < d {
                continue;
            }

            for (next, next_d) in &list[i] {
                if count[i][*next] <= (max_moves - dist[i]).min(*next_d) {
                    count[i][*next] = (max_moves - dist[i]).min(*next_d);
                }
                if dist[*next] > dist[i] + next_d && dist[i] + next_d <= max_moves {
                    dist[*next] = dist[i] + next_d;
                    heap.push((Reverse(dist[*next]), *next));
                }
            }
        }

        (0..edges.len())
            .map(|i| {
                edges[i][2].min(
                    count[edges[i][0] as usize][edges[i][1] as usize]
                        + count[edges[i][1] as usize][edges[i][0] as usize],
                )
            })
            .sum::<i32>()
            + (0..n).filter(|i| dist[*i] < std::i32::MAX).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day12() {
        assert_eq!(
            Solution::reachable_nodes(vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 2, 2]], 6, 3),
            13
        );
        assert_eq!(
            Solution::reachable_nodes(
                vec![vec![0, 1, 4], vec![1, 2, 6], vec![0, 2, 8], vec![1, 3, 1]],
                10,
                4
            ),
            23
        );
        assert_eq!(
            Solution::reachable_nodes(
                vec![
                    vec![1, 2, 4],
                    vec![1, 4, 5],
                    vec![1, 3, 1],
                    vec![2, 3, 4],
                    vec![3, 4, 5]
                ],
                17,
                5
            ),
            1
        );
    }
}
