use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        const M: usize = 1_000_000_007;

        let mut matrix = vec![vec![-1; n as usize]; n as usize];
        for road in roads {
            matrix[road[0] as usize][road[1] as usize] = road[2];
            matrix[road[1] as usize][road[0] as usize] = road[2];
        }

        let mut min_time = vec![std::usize::MAX; n as usize];
        let mut count = vec![0; n as usize];
        min_time[0] = 0;
        count[0] = 1;

        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), 0));

        while let Some((Reverse(time), node)) = heap.pop() {
            if min_time[node] < time {
                continue;
            }

            for next in 0..n as usize {
                if matrix[node][next] == -1 || min_time[next] < time + matrix[node][next] as usize {
                    continue;
                }
                if min_time[next] == time + matrix[node][next] as usize {
                    count[next] += count[node];
                    count[next] %= M;
                    continue;
                }
                min_time[next] = time + matrix[node][next] as usize;
                count[next] = count[node];

                heap.push((Reverse(min_time[next]), next));
            }
        }

        count[n as usize - 1] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1976() {
        assert_eq!(
            Solution::count_paths(
                7,
                vec![
                    vec![0, 6, 7],
                    vec![0, 1, 2],
                    vec![1, 2, 3],
                    vec![1, 3, 3],
                    vec![6, 3, 3],
                    vec![3, 5, 1],
                    vec![6, 5, 1],
                    vec![2, 5, 1],
                    vec![0, 4, 5],
                    vec![4, 6, 2]
                ]
            ),
            4
        );
        assert_eq!(Solution::count_paths(2, vec![vec![1, 0, 10]]), 1);
    }
}
