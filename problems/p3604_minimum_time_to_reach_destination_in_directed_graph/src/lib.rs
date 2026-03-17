use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut list = vec![vec![]; n];
        for e in edges {
            list[e[0] as usize].push((e[1] as usize, e[2], e[3]));
        }
        let mut seen = vec![std::i32::MAX; n];
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), 0));

        while let Some((Reverse(t), i)) = heap.pop() {
            if i == n - 1 {
                return t;
            }
            if seen[i] <= t {
                continue;
            }
            seen[i] = t;

            for (next, start, end) in &list[i] {
                if end < &t || seen[*next] <= t.max(*start) + 1 {
                    continue;
                }
                heap.push((Reverse(t.max(*start) + 1), *next))
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3604() {
        assert_eq!(
            Solution::min_time(3, vec![vec![0, 1, 0, 1], vec![1, 2, 2, 5]]),
            3
        );
        assert_eq!(
            Solution::min_time(
                4,
                vec![
                    vec![0, 1, 0, 3],
                    vec![1, 3, 7, 8],
                    vec![0, 2, 1, 5],
                    vec![2, 3, 4, 7]
                ]
            ),
            5
        );
        assert_eq!(
            Solution::min_time(3, vec![vec![1, 0, 1, 3], vec![1, 2, 3, 5]]),
            -1
        );
    }
}
