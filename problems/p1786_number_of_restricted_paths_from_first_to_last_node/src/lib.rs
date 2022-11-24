use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        const M: i32 = 1_000_000_007;

        let mut list = vec![vec![]; n as usize];
        for edge in edges {
            list[edge[0] as usize - 1].push((edge[1] as usize - 1, edge[2]));
            list[edge[1] as usize - 1].push((edge[0] as usize - 1, edge[2]));
        }

        let mut heap = BinaryHeap::new();
        let mut dist = vec![std::i32::MAX; n as usize];
        let mut count = vec![0; n as usize];
        heap.push((Reverse(0), n as usize - 1));
        count[n as usize - 1] = 1;

        while let Some((Reverse(d), node)) = heap.pop() {
            if dist[node] != std::i32::MAX {
                continue;
            }
            dist[node] = d;

            for (next, next_d) in &list[node] {
                if dist[*next] == std::i32::MAX {
                    heap.push((Reverse(d + next_d), *next));
                } else if dist[*next] < dist[node] {
                    count[node] = (count[*next] + count[node]) % M;
                }
            }
        }

        count[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1786() {
        assert_eq!(
            Solution::count_restricted_paths(
                5,
                vec![
                    vec![1, 2, 3],
                    vec![1, 3, 3],
                    vec![2, 3, 1],
                    vec![1, 4, 2],
                    vec![5, 2, 2],
                    vec![3, 5, 1],
                    vec![5, 4, 10]
                ]
            ),
            3
        );
        assert_eq!(
            Solution::count_restricted_paths(
                7,
                vec![
                    vec![1, 3, 1],
                    vec![4, 1, 2],
                    vec![7, 3, 4],
                    vec![2, 5, 3],
                    vec![5, 6, 1],
                    vec![6, 7, 2],
                    vec![7, 5, 3],
                    vec![2, 6, 4]
                ]
            ),
            1
        );
    }
}
